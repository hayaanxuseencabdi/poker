use crate::{card::Card, player::Player};

pub(crate) struct Table {
    community_cards: [Option<Card>; 3],
    players: Vec<Player>,
}


// Construct every permutation of hands and choose the winner from them
// 5 * 4 * 3 = 60 possible hands in total
impl Table {
    pub(crate) fn flop() {}

    pub(crate) fn turn() {}

    pub(crate) fn river() {}
}
