#[derive(Debug, Hash, PartialEq, PartialOrd)]
enum Rank {
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
enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

#[derive(Debug, Hash)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }
}

// Construct every permutation of hands and choose the winner from them
// 5 * 4 * 3 = 60 possible hands in total
#[derive(Debug, Hash)]
struct Hand {
    hole_cards: [Card; 2],
    community_cards: [Option<Card>; 5],
}

impl Hand {
    fn generate_all_possible_hands() -> Vec<[Card; 5]> {
        vec![]
    }
}

#[derive(Debug, Hash)]
enum HandRanking {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

fn main() {
    let hand = Hand {
        hole_cards: [
            Card::new(Rank::Ace, Suit::Club),
            Card::new(Rank::King, Suit::Club),
        ],
        community_cards: [
            Some(Card::new(Rank::Ace, Suit::Spade)),
            Some(Card::new(Rank::King, Suit::Spade)),
            Some(Card::new(Rank::Queen, Suit::Spade)),
            Some(Card::new(Rank::Jack, Suit::Spade)),
            Some(Card::new(Rank::Ten, Suit::Spade)),
        ],
    };
    println!("{:#?}", hand);
    println!("Ace > 10? {:#?}", Rank::Ace > Rank::Ten);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
