mod cli;
use crate::cli::{FuddConnect, FuddCommands, get_random_phrase};
use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::ffi;
use std::io::{Error, BufReader, Read, Write};
use snafu::{Snafu, ResultExt};
use amiquip::{Get, Connection, Publish, Channel, Error as AmiquipError};

use std::thread;
use std::time::{Duration, SystemTime, SystemTimeError, UNIX_EPOCH};
use uuid::Uuid;


 

type FuddResult<T, E=FuddErrors> = std::result::Result<T,E>;

#[derive(Debug, Snafu)]
enum FuddErrors { 
    #[snafu(display("{} is invalid", detail), context(suffix(false)))]
    ValidationError { detail: String},
    
    #[snafu(display("Invalid path '{}'", path.display()), context(suffix(false)))]
    InvalidPathError { path: PathBuf, source: std::io::Error },

    #[snafu(display("Error obtaining metadata from path '{}'", path.display()), context(suffix(false)))]
    MetadataError { path: PathBuf, source: std::io::Error},

    #[snafu(display("Error accessing '{}'", path.display()), context(suffix(false)))]
    FileAccessError { path: PathBuf, source: std::io::Error },

    #[snafu(display("no content read for '{}'", path.display()), context(suffix(false)))]
    NoContentReadError {path: PathBuf},

    #[snafu(display("No file with extension of '{}' was found at '{}'", file_ext, path.display()), context(suffix(false)))]
    InputFilesNotFoundError{ file_ext: String,  path: PathBuf},

    #[snafu(display("Error while opening a channel."), context(suffix(false)))]
    ChannelOpeningError { source: AmiquipError},

    #[snafu(display("Error while publishing! Published {} out of {} messages", expected, done), context(suffix(false)))]
    PublishingDiffError { expected: usize, done: usize},

    #[snafu(display("An error ocurred while accessing or creating directory(ies). Verify permissions or if the provided path is actually a diretory: '{}'", path.display()), context(suffix(false)))]
    OutputDirectoryError{ path: PathBuf, source: std::io::Error},

    #[snafu(display("An invalid message quantity was provided. {} < 1", qt), context(suffix(false)))]
    InvalidMessageRangeError{qt: usize},

    #[snafu(display("An unexpected error ocurred. Details:{}, Source: {}", detail, source), context(suffix(false)))]
    InvalidPathGenerationError{ detail: String, source: std::io::Error},

    #[snafu(display("Error calculating time in milliseconds since unix epoch"),context(suffix(false)))]
    SystemTimeInMillisError{ source: SystemTimeError },


    #[snafu(display("An error ocurred while saving file '{}'. Details: {}", path.display(), source), context(suffix(false)))]
    SaveFileError{path: PathBuf, source: std::io::Error},

    #[snafu(display("Error converting message body to UTF-8 String"))]
    InvalidUTF8MessageBodyError,
}


fn get_time_in_milliseconds() -> FuddResult<u128> {
    let milliseconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context(SystemTimeInMillis)?.as_millis();
    Ok(milliseconds)
} 


fn new_filename(output_dir: PathBuf, base_filename: &str) -> FuddResult<PathBuf> {
    let mut output_dir_alt = output_dir.to_owned();
    output_dir_alt.push(format!("{}.json", base_filename));
    if output_dir_alt.exists() {
        let now = get_time_in_milliseconds()?;
        return new_filename(output_dir.to_owned(), &format!("{}_{}", base_filename, now));
    }
    Ok(output_dir_alt)
}


fn save(message: Get, output_dir: &PathBuf, filename: &str) -> FuddResult<PathBuf> {
    let mut complete_path = new_filename(output_dir.to_owned(), filename)?;
    let mut file = fs::File::create(&complete_path).context(SaveFile{path: output_dir.to_owned()})?;
    //get message body and dump it to file
    match String::from_utf8(message.delivery.body) {
        Err(e) => Err(FuddErrors::InvalidUTF8MessageBodyError),
        Ok(content) => {
            file.write_all(content.as_bytes()).context(SaveFile{path: complete_path.to_owned()})?;
            Ok(complete_path)
        }
    }
}

fn get_messages(connection: &mut Connection, queue: &str, message_qt: &usize, output:&PathBuf, ack: &bool) -> FuddResult<()>{

    println!("hey");
    if *message_qt < 1 {
       return Err(FuddErrors::InvalidMessageRangeError{qt: *message_qt});
   }
   
   println!("testing output directory");
   if enable_output_directory(output)? {
       println!("output directory is OK?");
      //if let Ok(outdir) = enable_output_directory(output) {
        
       let channel = connection.open_channel(None).context(ChannelOpening)?;
       let mut qt:usize = 0;
                 

        for i in 0..*message_qt {
            println!("trying to get message {} ", &i);
            //get messages from queue
            if let Ok(message) = channel.basic_get(queue, !ack) {
                //save message to output
                if let Some(available_message) = message {
                    if let Ok(new_file_path) = save(available_message, output, Uuid::new_v4().to_string().as_str()) {
                        print!("Saving message to '{}'... ", new_file_path.display());
                        if new_file_path.exists() {
                            print!("[OK]\n");
                        } else {
                            print!("[ERR]\n");
                            eprintln!("An error occurred saving a message to file '{}'", new_file_path.display())
                        }
                    }
                } else {
                    eprintln!("GET Message is not available.");
                }
            } else {
                println!("\nNo messages left on queue {}. Aborting...", queue);
                break;
            }            
        }

        drop(channel);
   }

  Ok(()) 

}

fn consume_messages(queue: &str, output: &PathBuf, ack: &bool) {
    dbg!(queue);
    dbg!(output);
    dbg!(ack);
}


fn enable_output_directory(output: &PathBuf) -> FuddResult<bool> {
    if !output.exists() {
        match fs::create_dir_all(output) {
            Ok(_) => return Ok(true),
            Err(e) => return Err(FuddErrors::OutputDirectoryError{path: output.to_owned(), source: e}),
        };
    }
    Ok(true)
}


///
///Given the input, json file(s), publish a message directed to the provided exchange using the
///given routing key. It waits for a opened connection and it opens and drops its own channel.
///If successful returns the message count 
///
fn publish_messages(connection: &mut Connection, exchange: &str, routing_key: &str, input:&PathBuf) -> FuddResult<usize>{
    let file_extension = "json";
    let mut pub_count:usize = 0;

    let channel = connection.open_channel(None).context(ChannelOpening)?;
    
    if let Ok(files) = list_files(input, file_extension) {
        files.iter().for_each(|file_path|{
            print!("\n\tReading contents from '{}'...", file_path.display());
            if let Ok(content) = get_file_content(file_path) {
                //publish message
                print!("Done.\tPublishing message...");
                let _ = channel.basic_publish(exchange, Publish::new(content.as_bytes(), routing_key));
                pub_count += 1;
                print!("Done.\n");
            }
        });
        
        if pub_count != files.len() {
            drop(channel);
            return Err(FuddErrors::PublishingDiffError{expected: files.len(), done: pub_count});
        }

    } else {
        drop(channel);
        return Err(FuddErrors::InputFilesNotFoundError{file_ext: file_extension.to_string(), path: input.to_owned()});
    }

    drop(channel);
    Ok(pub_count)

}





/// Lists the files of a given extension on a given path.
/// if the path is already a file, returns it if the file has the same extension otherwise, 
/// if its a directory, seeks files of the given extension directly below the path provided.
///
/// #Examples
///
/// Suppose that we want to search json files at '~/downloads/'.
/// ```
/// let downloads_path = std::path::PathBuf::from("~/downloads/");
/// 
/// if let Ok(json_files) = crate::list_files() {
///     //json_files is a Vec<PathBuf> of json files found at downloads_path
/// }
/// ```
/// or download_path could be a direct path to a json file, like "~/downloads/input001.json".
///
/// #Errors
///
/// If files of the provided extension are not found, the FuddErrors::InputFilesNotFoundError is
/// returned.
/// If a problem occurs while reading the path's metadata, FuddErrors::MetadataError is returned.
///
fn list_files(buf: &PathBuf, file_extension: &str) -> FuddResult<Vec<PathBuf>> {

    let meta = fs::metadata(buf).context(Metadata{path: buf.to_owned()})?;

    if meta.is_file() {

        if buf.as_path().extension().and_then(ffi::OsStr::to_str) == Some(file_extension) {
            return Ok(vec![buf.to_owned()]);
            
        } 

    } else if meta.is_dir() {
        
        if let Ok(entries) = fs::read_dir(buf) {
            let shallow_search = entries.filter_map(|entry| {
                match entry {
                    Ok(e) => {
                      if let Some(ext) = e.path().extension().and_then(ffi::OsStr::to_str) {
                        if ext.to_lowercase() == file_extension {
                            return Some(e.path());
                        } 
                      } 
                      None
                    },
                    
                    _ => None,
                }
            }).collect::<Vec<PathBuf>>();
            if !shallow_search.is_empty() {
                return Ok(shallow_search);
            }
        }
    }
    Err(FuddErrors::InputFilesNotFoundError{ path: buf.to_owned(), file_ext: file_extension.to_string() })
}

///
/// Reads the file content from the provided file path as a string.
///
/// #Examples
/// ```
/// if let Ok(content) = get_file_content(&path) {
///     //content is a String
/// }
/// ```
///
fn get_file_content(path: &PathBuf) -> FuddResult<String> {
    let mut file = fs::File::open(path).context(FileAccess{ path: path.to_owned() });
    if let Ok(f) = file {
        let mut reader = BufReader::new(f);
        let mut content = String::new();
        match reader.read_to_string(&mut content) {
            Ok(_) => return Ok(content),
            Err(ioerror) => return Err(FuddErrors::NoContentReadError{ path: path.to_owned() }),
        }
       
    }

    Err(FuddErrors::NoContentReadError{path: path.to_owned()})
    
 }


pub fn main() -> FuddResult<()> {
    
    let parsed_args = FuddConnect::parse();
    println!("\n\t{}", get_random_phrase());
    //open connection    
    match amiquip::Connection::insecure_open(&parsed_args.uri) {
        Ok(mut connection) => {
           match &parsed_args.subcommand {
                FuddCommands::Get{queue, message_qt, output, ack} => {
                    println!("GET WAS FOUND");
                    let _ = get_messages(&mut connection, queue, message_qt, output, ack);
                    return Ok(());

                },
                FuddCommands::Consume{queue, output, ack}=> todo!(),//consume_messages(queue, output, ack),
                FuddCommands::Publish{exchange, routing_key, input} => { 
                    println!("PUBLISH DETECTED");
                    match publish_messages(&mut connection, exchange, routing_key, input) {
                        Ok(msg_qtd) => println!("{} messages sent in total", msg_qtd),
                        Err(e) => eprintln!("{:?}", e),
                    }
                    return Ok(());
                },
            }
           drop(connection);
        }, 
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
    Ok(())
}
