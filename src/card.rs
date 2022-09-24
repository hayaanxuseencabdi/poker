#[derive(Debug, Hash, PartialEq, PartialOrd)]
pub(crate) enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Hash)]
pub(crate) enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

#[derive(Debug, Hash)]
pub(crate) struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub(crate) fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}
