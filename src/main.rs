mod cli;
use crate::cli::{FuddConnect, FuddCommands};
use clap::Parser;
use std::path::PathBuf;

fn get_messages(queue: &str, message_qt: &usize, output:&PathBuf, ack: &bool) {
    dbg!(queue);
    dbg!(message_qt);
    dbg!(output);
    dbg!(ack);
}

pub fn main() {
    
    let parsed_args = FuddConnect::parse();
    
    match &parsed_args.subcommand {
        FuddCommands::Get{queue, message_qt, output, ack} =>  get_messages(queue, message_qt, output, ack),
        FuddCommands::Consume{..}=> todo!(),
        FuddCommands::Publish{..} => todo!(),
    }


}
