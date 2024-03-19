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

fn consume_messages(queue: &str, output: &PathBuf, ack: &bool) {
    dbg!(queue);
    dbg!(output);
    dbg!(ack);
}

fn publish_messages(exchange: &str, routing_key: &str, input:&PathBuf) {
    dbg!(exchange);
    dbg!(routing_key);
    dbg!(input);
}

pub fn main() {
    
    let parsed_args = FuddConnect::parse();
    
    match &parsed_args.subcommand {
        FuddCommands::Get{queue, message_qt, output, ack} =>  get_messages(queue, message_qt, output, ack),
        FuddCommands::Consume{queue, output, ack}=> consume_messages(queue, output, ack),
        FuddCommands::Publish{exchange, routing_key, input} => publish_messages(exchange, routing_key, input),
    }


}
