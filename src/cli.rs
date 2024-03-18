use clap::{Parser, Subcommand};
use std::path::PathBuf;


pub const PHRASES: Vec<&str> = vec![
    "Wabbits wove cawwots. Pwetty cwafty, eh?",
    "You're the wabbit in disguise. Off with it, you twickster.",
    "Ooh, you double-cwossing wabbit! You tweacherous miscweant!", 
    "Shhhh... Be vewy vewy quiet. i`m hunting wabbits.",
    "KILL THE WABBIT!!!",
    "ITS WABBIT SEASON",
    "Scwewy Wabbit!", 
    "wascawwy wabbit!"
];


#[derive(Subcommand, Debug)]
pub enum FuddCommands {
    Get {
        #[arg(short, long)]
        queue: String,

        #[arg(short, long, default_value_t=1)]
        message_qt: usize,

        #[arg(short, long, value_name="OUTPUT PATH")]
        output: PathBuf,

        #[arg(short, long, default_value_t=false)]
        ack: bool,
    },

    Consume {
        #[arg(short, long)]
        queue: String,

        #[arg(short, long, value_name="OUTPUT PATH")]
        output: PathBuf,

        #[arg(short, long, default_value_t=false)]
        ack: bool,
    },

    Publish {
        #[arg(short, long)]
        exchange: String,

        #[arg(short, long, value_name="INPUT PATH")]
        input: PathBuf,

        #[arg(short, long, default_value="")]
        routing_key: String,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct FuddConnect {
    #[arg(short, long)]
    uri: Option<String>,

    #[command(subcommand)]
    subcommand: Option<FuddCommands>,
}
