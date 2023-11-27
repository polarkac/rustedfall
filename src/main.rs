use std::error::Error;
use clap::Parser;

use rustedfall::{commands, cards};

fn main() -> Result<(), Box<dyn Error>> {
    let args = commands::Cli::parse();
    match args.command {
        commands::Command::Search(args) => {
            let card_list = cards::search(args)?;
            println!("{}", card_list.total_cards().unwrap());
            card_list.data().iter().for_each(|card| {
                println!("{}\n\n", card);
            });
        },
        commands::Command::Fuzzy(args) => {
            let card = cards::fuzzy(args)?;
            println!("{}", card);
        },
        commands::Command::Exact(args) => {
            let card = cards::exact(args)?;
            println!("{}", card);
        },
        _ => {},
    }

    Ok(())
}
