use crate::cards::{CardId, CardInfos};

#[derive(Debug)]
pub struct PlayerBoard {
    remaining_actions: u8,
    remaining_buys: u8,
    pub name: String,
    deck: Vec<CardId>,
    hand: Vec<CardId>,
    discarded_card: Vec<CardId>,
    currently_played_card: Option<CardId>
}

impl PlayerBoard {
    pub fn new(name: String, card_infos: &CardInfos) -> Self {
        PlayerBoard {
            remaining_actions: 0,
            remaining_buys: 0,
            name,
            deck: card_infos.initial_deck.clone(),
            hand: vec![],
            discarded_card: vec![],
            currently_played_card: None,
        }
    }

    pub fn shuffle_deck(&mut self) {

    }

    pub fn draw_one_card(&mut self) {

    }

    pub fn draw_n_cards(&mut self) {

    }
}