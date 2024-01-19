use serde::Deserialize;
use serde_json::Value;

use crate::{
    utils::UUID, uri::URI
};

#[derive(Deserialize, Debug)]
pub struct Card {
    // Core Card Fields
    arena_id: Option<u32>,
    id: UUID,
    lang: Language,
    mtgo_id: Option<u32>,
    mtgo_foil_id: Option<u32>,
    multiverse_ids: Option<Vec<u32>>,
    tcgplayer_id: Option<u32>,
    tcgplayer_etched_id: Option<u32>,
    cardmarket_id: Option<u32>,
    object: String,
    layout: String,
    oracle_id: Option<UUID>,
    prints_search_uri: URI<Value>,
    rulings_uri: URI<Value>,
    scryfall_uri: URI<Value>,
    uri: URI<Self>,
    // Gameplay Fields
    all_parts: Option<Vec<RelatedCard>>,
    card_faces: Option<Vec<CardFace>>,
    cmc: f32,
    color_identity: ManaColors,
    color_indicator: Option<ManaColors>,
    colors: Option<ManaColors>,
    defense: Option<String>,
    edhrec_rank: Option<u32>,
    hand_modifier: Option<String>,
    keywords: Vec<String>,
    legalities: Option<Legalities>,
    life_modifier: Option<String>,
    loyalty: Option<String>,
    mana_cost: Option<String>,
    pub name: String,
    oracle_text: Option<String>,
    penny_rank: Option<u32>,
    power: Option<String>,
    produced_mana: Option<ManaColors>,
    reserved: bool,
    toughness: Option<String>,
    type_line: String,
    // Print Fields
    artist: Option<String>,
    artist_ids: Option<Vec<UUID>>,
    attraction_lights: Option<Vec<u32>>,
    booster: bool,
    border_color: BorderColor,
    card_back_id: Option<UUID>,
    collector_number: String,
    content_warning: Option<bool>,
    digital: bool,
    finishes: Vec<Finish>,
    flavor_name: Option<String>,
    flavor_text: Option<String>,
    frame_effects: Option<Vec<String>>,
    frame: String,
    games: Vec<String>,
    highres_image: bool,
    illustration_id: Option<UUID>,
    image_status: String,
    image_uris: Option<Images>,
    oversized: bool,
    prices: Prices,
    printed_name: Option<String>,
    printed_text: Option<String>,
    printed_type_line: Option<String>,
    promo: bool,
    promo_types: Option<Vec<String>>,
    purchase_uris: Option<PurchaseURIs>,
    rarity: Rarity,
    // TODO
    related_uris: Value,
    released_at: String,
    reprint: bool,
    scryfall_set_uri: URI<String>,
    set_name: String,
    set_search_uri: URI<String>,
    set_type: String,
    set_uri: URI<String>,
    set: String,
    set_id: UUID,
    story_spotlight: bool,
    textless: bool,
    variation: bool,
    variation_of: Option<UUID>,
    security_stamp: Option<String>,
    watermark: Option<String>,
    #[serde(rename = "preview.previewed_at")]
    preview_previewed_at: Option<String>,
    #[serde(rename = "preview.source_uri")]
    preview_source_uri: Option<URI<String>>,
    #[serde(rename = "preview.source")]
    preview_source: Option<String>,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.id.0 == other.id.0
    }
}

#[derive(Deserialize, Debug)]
pub struct RelatedCard {
    id: UUID,
    object: String,
    component: String,
    name: String,
    type_line: String,
    uri: URI<Card>,
}

#[derive(Deserialize, Debug)]
pub struct CardFace {
    artist: Option<String>,
    artist_id: Option<UUID>,
    cmc: Option<f32>,
    color_indicator: Option<ManaColors>,
    colors: Option<ManaColors>,
    defense: Option<String>,
    flavor_text: Option<String>,
    illustration_id: Option<UUID>,
    image_uris: Option<Images>,
    layout: Option<String>,
    loyalty: Option<String>,
    mana_cost: String,
    name: String,
    object: String,
    oracle_id: Option<UUID>,
    oracle_text: Option<String>,
    power: Option<String>,
    printed_name: Option<String>,
    printed_text: Option<String>,
    printed_type_line: Option<String>,
    toughness: Option<String>,
    type_line: Option<String>,
    watermark: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum LegalState {
    #[serde(alias = "legal")]
    Legal,
    #[serde(alias = "not_legal")]
    NotLegal,
    #[serde(alias = "banned")]
    Banned,
    #[serde(alias = "restricted")]
    Restricted,
}

#[derive(Deserialize, Debug)]
pub struct Legalities {
    standard: LegalState,
    future: LegalState,
    historic: LegalState,
    timeless: LegalState,
    gladiator: LegalState,
    pioneer: LegalState,
    explorer: LegalState,
    modern: LegalState,
    legacy: LegalState,
    pauper: LegalState,
    vintage: LegalState,
    penny: LegalState,
    commander: LegalState,
    oathbreaker: LegalState,
    brawl: LegalState,
    historicbrawl: LegalState,
    alchemy: LegalState,
    paupercommander: LegalState,
    duel: LegalState,
    oldschool: LegalState,
    premodern: LegalState,
    predh: LegalState,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Finish {
    Foil,
    NonFoil,
    Etched,
}

#[derive(Deserialize, Debug)]
pub enum Language {
    #[serde(alias = "en")]
    English,
    #[serde(alias = "es")]
    Spanish,
    #[serde(alias = "fr")]
    French,
    #[serde(alias = "de")]
    German,
    #[serde(alias = "it")]
    Italian,
    #[serde(alias = "pt")]
    Portugese,
    #[serde(alias = "jp")]
    Japanese,
    #[serde(alias = "ko")]
    Korean,
    #[serde(alias = "ru")]
    Russian,
    #[serde(alias = "zhs")]
    SimplifiedChinese,
    #[serde(alias = "zht")]
    TraditionalChinese,
    #[serde(alias = "he")]
    Hebrew,
    #[serde(alias = "la")]
    Latin,
    #[serde(alias = "grc")]
    AncientGreek,
    #[serde(alias = "ar")]
    Arabic,
    #[serde(alias = "sa")]
    Sanskrit,
    #[serde(alias = "ph")]
    Phyrexian,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum ManaColor {
    #[serde(alias = "W")]
    White,
    #[serde(alias = "U")]
    Blue,
    #[serde(alias = "B")]
    Black,
    #[serde(alias = "R")]
    Red,
    #[serde(alias = "G")]
    Green,
    #[serde(alias = "C")]
    Colorless,
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct ManaColors {
    pub colors: Vec<ManaColor>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BorderColor {
    Black,
    White,
    Silver,
    Gold,
    Borderless,
}

#[derive(Deserialize, Debug)]
pub struct Images {
    png: URI<String>,
    border_crop: URI<String>,
    art_crop: URI<String>,
    large: URI<String>,
    normal: URI<String>,
    small: URI<String>,
}

#[derive(Deserialize, Debug)]
pub struct Prices {
    usd: Option<String>,
    usd_foil: Option<String>,
    usd_etched: Option<String>,
    eur: Option<String>,
    eur_foil: Option<String>,
    tix: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PurchaseURIs {
    tcgplayer: Option<URI<String>>,
    cardmarket: Option<URI<String>>,
    cardhoarder: Option<URI<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Special,
    Mythic,
    Bonus,
}

#[cfg(test)]
mod tests {
    use crate::{cards::{Card, ManaColor}, uri::URI};

    #[test]
    fn fetch_card_lightning_bolt() {
        let uri: URI<Card> = URI::new("https://api.scryfall.com/cards/f29ba16f-c8fb-42fe-aabf-87089cb214a7");
        let card = uri.fetch().unwrap();
        assert_eq!(card.id.0, "f29ba16f-c8fb-42fe-aabf-87089cb214a7");
        assert_eq!(card.name, "Lightning Bolt");
        assert_eq!(card.color_identity.colors, vec![ManaColor::Red]);
    }
}
