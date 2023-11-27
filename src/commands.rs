use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Search(SearchArgs),
    Exact(NamedArgs),
    Fuzzy(NamedArgs),
    Sets,
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    /// Search pattern
    pub q: String,
    /// Order of cards
    #[arg(value_enum, default_value_t = Order::Name)]
    pub order: Order,
    /// Direction for sorting cards
    #[arg(value_enum, default_value_t = Direction::Auto)]
    pub direction: Direction,
}

#[derive(Args, Debug)]
pub struct NamedArgs {
    pub name: String,
    pub set: Option<String>
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Order {
    Name,
    Set,
    Released,
    Rarity,
    Color,
    Usd,
    Tix,
    Eur,
    Cmc,
    Power,
    Toughness,
    Edhrec,
    Penny,
    Artist,
    Review,
}

impl Order {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Set => "set",
            Self::Released => "released",
            Self::Rarity => "rarity",
            Self::Color => "color",
            Self::Usd => "usd",
            Self::Tix => "tix",
            Self::Eur => "eur",
            Self::Cmc => "cmc",
            Self::Power => "power",
            Self::Toughness => "toughness",
            Self::Edhrec => "edhrec",
            Self::Penny => "penny",
            Self::Artist => "artist",
            Self::Review => "review",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Direction {
    Auto,
    Ascending,
    Descending,
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Ascending => "asc",
            Self::Descending => "desc",
        }
    }
}
