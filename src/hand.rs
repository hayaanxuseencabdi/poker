use crate::card::{self, Card};

#[derive(Debug)]
enum Ranking {
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

#[derive(Debug)]
pub(crate) struct Hand {
    cards: [Card; 5],
    ranking: Ranking,
}

impl Hand {
    pub(crate) fn new(cards: [Card; 5], ranking: Ranking) -> Self {
        Self {
            cards,
            ranking: Self::compute_best_ranking(&cards),
        }
    }

    fn compute_best_ranking(cards: &[Card; 5]) -> Ranking {
        Ranking::Flush { rank: card::Rank::Ace }
    }

}
