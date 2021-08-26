use std::collections::HashMap;
use std::default::Default;

pub type CardId = String;


#[derive(Copy, Clone, Debug)]
pub enum ActionType {
    None,
    Attack,
    Reaction,
}

#[derive(Copy, Clone, Debug)]
pub enum CardType {
    Victory,
    Treasure,
    Kingdom(ActionType),
    Curse,
}

#[derive(Default, Clone, Debug)]
pub struct CardGain {
    coin_gain: Option<u8>,
    victory_gain: Option<i8>,
    action_gain: Option<u8>,
    card_gain: Option<u8>,
    other_effects: String  // todo: inject particular functions here
}

#[derive(Clone, Debug)]
pub struct Card {
    card_id: CardId,
    card_type: CardType,
    name: String,
    human_text: String,
    store_cost: u8,
    card_gain: CardGain
}

#[derive(Debug)]
pub struct CardInfos {
    cards: HashMap<CardId, Card>,
    pub initial_store: HashMap<CardId, u8>,
    pub initial_deck: Vec<CardId>,
}

fn load_default() -> Vec<Card> {
    vec![
        Card {
            card_id: "copper".to_string(),
            name: "Copper".to_string(),
            card_type: CardType::Treasure,
            human_text: "Gain 1 coin".to_string(),
            store_cost: 0,
            card_gain: CardGain {
                coin_gain: Some(1),
                ..Default::default()
            }
        },
        Card {
            card_id: "silver".to_string(),
            name: "Silver".to_string(),
            card_type: CardType::Treasure,
            human_text: "Gain 2 coin".to_string(),
            store_cost: 3,
            card_gain: CardGain {
                coin_gain: Some(2),
                ..Default::default()
            }
        },

        Card {
            card_id: "gold".to_string(),
            name: "Gold".to_string(),
            card_type: CardType::Treasure,
            human_text: "Gain 3 coin".to_string(),
            store_cost: 6,
            card_gain: CardGain {
                coin_gain: Some(3),
                ..Default::default()
            }
        },
        Card {
            card_id: "estate".to_string(),
            name: "Estate".to_string(),
            card_type: CardType::Treasure,
            human_text: "Secure 1 Victory".to_string(),
            store_cost: 2,
            card_gain: CardGain {
                victory_gain: Some(1),
                ..Default::default()
            }
        },
        Card {
            card_id: "duchy".to_string(),
            name: "Duchy".to_string(),
            card_type: CardType::Treasure,
            human_text: "Secure 3 Victory".to_string(),
            store_cost: 5,
            card_gain: CardGain {
                victory_gain: Some(3),
                ..Default::default()
            }
        },
        Card {
            card_id: "province".to_string(),
            name: "Province".to_string(),
            card_type: CardType::Treasure,
            human_text: "Secure 6 Victory".to_string(),
            store_cost: 8,
            card_gain: CardGain {
                victory_gain: Some(6),
                ..Default::default()
            }
        },
        Card {
            card_id: "curse".to_string(),
            name: "Curse".to_string(),
            card_type: CardType::Treasure,
            human_text: "Lose 1 Victory".to_string(),
            store_cost: 0,
            card_gain: CardGain {
                victory_gain: Some(-1),
                ..Default::default()
            }
        },
    ]
}


impl CardInfos {
    // todo: implement passing number of players
    pub fn init_default() -> Self {
        let default_cards = load_default();

        let mut default_store = HashMap::new();
        default_store.insert("copper".to_string(), 60);
        default_store.insert("silver".to_string(), 40);
        default_store.insert("gold".to_string(), 30);
        default_store.insert("estate".to_string(), 12);
        default_store.insert("duchy".to_string(), 12);
        default_store.insert("province".to_string(), 12);

        let mut default_deck = vec![];
        for _ in 0..7 {
            default_deck.push("copper".to_string());
        }
        for _ in 0..3 {
            default_deck.push("estate".to_string());
        }
        let mut default_cards_ids = HashMap::new();

        for card in default_cards.iter() {
            // not sure about those clone, but it's easier
            default_cards_ids.insert(card.card_id.clone(), card.clone());
        }

        CardInfos {
            cards: default_cards_ids,
            initial_store: default_store,
            initial_deck: default_deck,
        }
    }

    fn validate_setup(&self) -> bool {
        // check that initial_store contains cards from .cards
        // check that initial_deck contains cards from .cards
        true
    }
}