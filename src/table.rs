use std::default;

use crate::{
    card::{Card, Deck},
    player::Player,
};

pub(crate) struct Table {
    players: Vec<Player>,
    deck: Deck,
    community_cards: [Option<Card>; 3],
}

// Construct every permutation of hands and choose the winner from them
// 5 * 4 * 3 = 60 possible hands in total
impl Table {
    pub(crate) fn new(number_of_players: usize) -> Self {
        Self {
            community_cards: [None, None, None],
            deck: Deck::new(),
            players: Vec::with_capacity(number_of_players),
        }
    }

    fn reset_hand(&mut self) {
        self.community_cards = [None, None, None];
    }

    fn deal_cards(&mut self) {
        for player in self.players.iter_mut() {
            // player.assign_card(deck.);
            // player.assign_card();
        }
    }
}
