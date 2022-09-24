use crate::card::Card;

pub(crate) struct Player {
    cards: [Card; 2],
}

impl Player {
    pub(crate) fn new(cards: [Card; 2]) -> Self {
        Self { cards }
    }
}
