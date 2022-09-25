use rand::{distributions::Uniform, thread_rng, Rng};
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, strum::EnumCount, strum::EnumIter,
)]
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, strum::EnumCount, strum::EnumIter)]
pub(crate) enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub(crate) struct Card {
    pub(crate) suit: Suit,
    pub(crate) rank: Rank,
}

impl Card {
    pub(crate) fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

pub(crate) struct Deck {
    cards: Vec<Card>,
    rng: Box<dyn rand::RngCore>,
}

impl Deck {
    pub(crate) fn new() -> Self {
        Self {
            cards: Self::generate_deck(),
            rng: Box::new(rand::thread_rng()),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub(crate) fn reset_deck(&mut self) {
        self.cards = Self::generate_deck();
    }

    pub(crate) fn pull_card(&mut self) -> Card {
        if self.cards.is_empty() {
            panic!("the deck has been depleted");
        }
        let distribution = rand::distributions::Uniform::new(0, self.cards.len());
        self.cards.swap_remove(self.rng.sample(distribution))
    }

    fn generate_deck() -> Vec<Card> {
        let mut cards = Vec::with_capacity(Suit::COUNT * Rank::COUNT);
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit, rank));
            }
        }
        cards
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
            assert!(
                !encountered_cards.contains(&card),
                "somehow able to pull the same card from a deck twice"
            );
            encountered_cards.insert(card);
        }
    }
}
