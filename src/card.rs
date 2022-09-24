use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, EnumCount, EnumIter)]
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EnumCount, EnumIter)]
pub(crate) enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub(crate) struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub(crate) fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

#[derive(Debug, Hash)]
pub(crate) struct Deck {
    cards: Vec<Card>,
    // rng:
}

impl Deck {
    pub(crate) fn new() -> Self {
        let mut cards = Vec::with_capacity(Suit::COUNT * Rank::COUNT);
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit, rank));
            }
        }
        Self { cards }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub(crate) fn shuffle_deck(&mut self) {
        // TODO
    }

    pub(crate) fn pull_card(&mut self) -> Card {
        if self.cards.is_empty() {
            panic!("the deck has been depleted");
        }
        // TODO: replace with randomly generated index in range [0, self.cards.size)
        self.cards.swap_remove(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unable_to_draw_duplicate_cards() {
        let mut deck = Deck::new();
        let mut encountered_cards = std::collections::HashSet::<Card>::new();
        while !deck.is_empty() {
            let card = deck.pull_card();
            assert!(!encountered_cards.contains(&card), "able to pull the same card from a deck twice");
            encountered_cards.insert(card);
        }
    }

    #[test]
    fn it_works() {
    }
}
