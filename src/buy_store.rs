use crate::cards::{CardInfos, CardId};
use std::collections::HashMap;

#[derive(Debug)]
pub struct BuyStore {
    piles: HashMap<CardId, u8>
}

impl BuyStore {
    pub fn new(card_infos: &CardInfos) -> Self {
        BuyStore {
            piles: card_infos.initial_store.clone()
        }
    }

    pub fn get_number_at_pile(&self, card_id: &CardId) -> u8 {
        // if you forgot province in the init
        if !self.piles.contains_key(card_id) {
            panic!("Add province cards to store to allow end game");
        }
        self.piles.get(card_id).unwrap().clone()
    }

    pub fn get_empty_piles(&self) -> Vec<CardId> {
        let mut empties = vec![];
        for (key, val) in self.piles.iter() {
            if *val == 0 {
                empties.push(key.clone());
            }
        }
        empties
    }
}
