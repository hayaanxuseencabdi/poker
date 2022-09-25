use crate::card::{self, Card};

#[derive(Debug, PartialEq)]
pub(crate) enum Ranking {
    HighCard {
        cards: [card::Rank; 5],
    },
    Pair {
        rank: card::Rank,
        kickers: [card::Rank; 3],
    },
    TwoPair {
        higher_pair: card::Rank,
        lower_pair: card::Rank,
        kicker: card::Rank,
    },
    ThreeOfAKind {
        rank: card::Rank,
        kickers: [card::Rank; 2],
    },
    Straight {
        rank: card::Rank,
    },
    Flush {
        rank: card::Rank,
    },
    FullHouse {
        three_of_a_kind: card::Rank,
        pair: card::Rank,
    },
    FourOfAKind {
        rank: card::Rank,
        kicker: card::Rank,
    },
    StraightFlush {
        rank: card::Rank,
    },
    RoyalFlush,
}

impl PartialOrd for Ranking {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct Hand {
    cards: [Card; 5],
    ranking: Ranking,
}

impl Hand {
    pub(crate) fn new(mut cards: [Card; 5]) -> Self {
        cards.sort();
        Self {
            cards,
            ranking: Self::compute_best_ranking(&cards),
        }
    }

    pub(crate) fn beats(&self, other: &Self) -> bool {
        self.ranking < other.ranking
    }

    fn compute_best_ranking(cards: &[Card; 5]) -> Ranking {
        Ranking::RoyalFlush
    }

    pub(crate) fn contains(&self, rank: card::Rank) -> bool {
        self.cards.iter().any(|card| card.rank == rank)
    }

    pub(crate) fn have_the_same_suit(&self) -> bool {
        let hands_suit = self.cards.first().unwrap().suit;
        self.cards.iter().all(|card| card.suit == hands_suit)
    }

    pub(crate) fn is_flush(&self) -> bool {
        todo!()
    }

    pub(crate) fn is_straight(&self) -> bool {
        let ranks = self.cards.map(|card| card.rank);
        let highest_rank = ranks.last().unwrap();
        let lowest_rank = ranks.first().unwrap();
        match highest_rank {
            card::Rank::Ace => match lowest_rank {
                card::Rank::Two => {
                    ranks
                        == [
                            card::Rank::Two,
                            card::Rank::Three,
                            card::Rank::Four,
                            card::Rank::Five,
                            card::Rank::Ace,
                        ]
                }
                card::Rank::Ten => {
                    ranks
                        == [
                            card::Rank::Ten,
                            card::Rank::Jack,
                            card::Rank::Queen,
                            card::Rank::King,
                            card::Rank::Ace,
                        ]
                }
                card::Rank::Three
                | card::Rank::Four
                | card::Rank::Five
                | card::Rank::Six
                | card::Rank::Seven
                | card::Rank::Eight
                | card::Rank::Nine
                | card::Rank::Jack
                | card::Rank::Queen
                | card::Rank::King
                | card::Rank::Ace => false,
            },
            _ => match lowest_rank {
                card::Rank::Two => {
                    ranks
                        == [
                            card::Rank::Two,
                            card::Rank::Three,
                            card::Rank::Four,
                            card::Rank::Five,
                            card::Rank::Six,
                        ]
                }
                card::Rank::Three => {
                    ranks
                        == [
                            card::Rank::Three,
                            card::Rank::Four,
                            card::Rank::Five,
                            card::Rank::Six,
                            card::Rank::Seven,
                        ]
                }
                card::Rank::Four => {
                    ranks
                        == [
                            card::Rank::Four,
                            card::Rank::Five,
                            card::Rank::Six,
                            card::Rank::Seven,
                            card::Rank::Eight,
                        ]
                }
                card::Rank::Five => {
                    ranks
                        == [
                            card::Rank::Five,
                            card::Rank::Six,
                            card::Rank::Seven,
                            card::Rank::Eight,
                            card::Rank::Nine,
                        ]
                }
                card::Rank::Six => {
                    ranks
                        == [
                            card::Rank::Six,
                            card::Rank::Seven,
                            card::Rank::Eight,
                            card::Rank::Nine,
                            card::Rank::Ten,
                        ]
                }
                card::Rank::Seven => {
                    ranks
                        == [
                            card::Rank::Seven,
                            card::Rank::Eight,
                            card::Rank::Nine,
                            card::Rank::Ten,
                            card::Rank::Jack,
                        ]
                }
                card::Rank::Eight => {
                    ranks
                        == [
                            card::Rank::Eight,
                            card::Rank::Nine,
                            card::Rank::Ten,
                            card::Rank::Jack,
                            card::Rank::Queen,
                        ]
                }
                card::Rank::Nine => {
                    ranks
                        == [
                            card::Rank::Nine,
                            card::Rank::Ten,
                            card::Rank::Jack,
                            card::Rank::Queen,
                            card::Rank::King,
                        ]
                }
                card::Rank::Ten | card::Rank::Jack | card::Rank::Queen | card::Rank::King => false,
                card::Rank::Ace => {
                    unreachable!("impossible provided the hand is sorted w.r.t. rank")
                }
            },
        }
    }

    pub(crate) fn is_wheel(&self) -> bool {
        self.is_straight() && self.contains(card::Rank::Ace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

    #[test]
    fn detect_wheel_as_straight() {
        let wheel = Hand::new([
            Card::new(Suit::Club, Rank::Ace),
            Card::new(Suit::Heart, Rank::Four),
            Card::new(Suit::Diamond, Rank::Two),
            Card::new(Suit::Club, Rank::Five),
            Card::new(Suit::Club, Rank::Three),
        ]);
        assert!(wheel.is_straight());
    }

    #[test]
    fn wheel_straight_should_lose_to_other_straights() {
        let wheel = Hand::new([
            Card::new(Suit::Club, Rank::Ace),
            Card::new(Suit::Heart, Rank::Four),
            Card::new(Suit::Diamond, Rank::Two),
            Card::new(Suit::Club, Rank::Five),
            Card::new(Suit::Club, Rank::Three),
        ]);

        let other_flush = Hand::new([
            Card::new(Suit::Club, Rank::Five),
            Card::new(Suit::Club, Rank::Three),
            Card::new(Suit::Club, Rank::Six),
            Card::new(Suit::Spade, Rank::Four),
            Card::new(Suit::Heart, Rank::Seven),
        ]);
        assert!(other_flush.beats(&wheel));
    }
}
