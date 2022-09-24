#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod card;
mod hand;
mod player;
mod table;

use crate::card::*;
use crate::hand::*;
use crate::player::*;
use crate::table::*;

fn main() {
    let hand = Hand::new([
        Card::new(Rank::Ace, Suit::Spade),
        Card::new(Rank::King, Suit::Spade),
        Card::new(Rank::Queen, Suit::Spade),
        Card::new(Rank::Jack, Suit::Spade),
        Card::new(Rank::Ten, Suit::Spade),
    ]);
    println!("{:#?}", hand);
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
