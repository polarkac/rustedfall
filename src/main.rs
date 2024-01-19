use std::error::Error;

use rustedfall::{cards::Card, list::List, uri::URI};

fn main() -> Result<(), Box<dyn Error>> {
    let mut card_list = fetch_list(URI::new(
        "https://api.scryfall.com/cards/search?q=c:white"
    ))?;

    while card_list.has_more {
        print_cards(&card_list);
        card_list = fetch_list(card_list.next_page.unwrap())?;
    }

    Ok(())
}

fn fetch_list(uri: URI<List<Card>>) -> Result<List<Card>, Box<dyn Error>> {
    let result = uri.fetch();

    let card_list: List<Card> = match result {
        Ok(list) => list,
        Err(_) => return Err("Error".into()),
    };

    Ok(card_list)
}

fn print_cards(list: &List<Card>) {
    for card in &list.data {
        println!("{}", card.name);
    }
}
