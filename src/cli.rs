use clap::{Parser, Subcommand};
use std::path::PathBuf;


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
