use serde::Deserialize;

use crate::uri::URI;

#[derive(Deserialize, Debug)]
pub struct List<T> {
    pub object: String,
    pub data: Vec<T>,
    pub has_more: bool,
    pub next_page: Option<URI<Self>>,
    pub total_cards: Option<u32>,
    pub warnings: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::List;
    use crate::{cards::Card, uri::URI};

    #[test]
    fn search_cards() {
        let uri: URI<List<Card>> = URI::new("https://api.scryfall.com/cards/search?q=c:white");
        let list = uri.fetch().unwrap();
        assert_eq!(list.total_cards, Some(5702));
        if list.has_more {
            let new_list = list.next_page.unwrap().fetch().unwrap();
            assert_eq!(new_list.total_cards, Some(1024));
        }
    }
}
