use std::{env::args, error::Error};

use rustedfall::cards;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args();
    match args.nth(1) {
        Some(command) => {
            match command.as_str() {
                "search" => {
                    let card = cards::named_fuzzy("Cheeky House-Mouse", Some("woe"))?;
                    println!("{}", card);
                }
                _ => println!("Unknown command"),
            }
        },
        None => println!("No command"),
    }

    Ok(())
}
