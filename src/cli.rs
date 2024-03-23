use clap::{Parser, Subcommand};
use std::path::PathBuf;
use rand::{thread_rng, Rng};


pub const PHRASES: [&'static str; 8] = [
    "Wabbits wove cawwots. Pwetty cwafty, eh?",
    "You're the wabbit in disguise. Off with it, you twickster.",
    "Ooh, you double-cwossing wabbit! You tweacherous miscweant!", 
    "Shhhh... Be vewy vewy quiet. i`m hunting wabbits.",
    "KILL THE WABBIT!!!",
    "ITS WABBIT SEASON",
    "Scwewy Wabbit!", 
    "wascawwy wabbit!"
];

pub fn get_random_phrase() -> &'static str {
    let random_idx = thread_rng().gen_range(0..PHRASES.len());
    return PHRASES[random_idx];
}


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
    pub uri: String,

    #[command(subcommand)]
    pub subcommand: FuddCommands,
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    fn phrases_as_hashmap() -> HashMap<String, String> {
         
        return PHRASES.iter().map(|x| (x.to_string(), x.to_string()) ).collect::<HashMap<String, String>>();
    }



    #[test]
    fn test_get_random_phrase_always_returns_a_valid_phrase() { //ultra important test
        let phrases_map: HashMap<String, String> = phrases_as_hashmap();
        for _ in 0..15 {
            let a_phrase = phrases_map.get(get_random_phrase());
            assert!(a_phrase.is_some())
        }
        
    }

}



