use log::{info, debug};
use crate::cards::CardInfos;
use crate::player::PlayerBoard;
use crate::buy_store::BuyStore;

#[derive(Debug)]
pub enum TurnPhase {
    Action,
    Buying,
    Cleaning,
}

#[derive(Debug)]
pub struct CurrentTurn {
    player_name: String,
    phase: TurnPhase,
}

#[derive(Debug)]
pub struct GameBoard {
    card_infos: CardInfos,
    players: Vec<PlayerBoard>,
    buy_store: BuyStore,
    current_turn: CurrentTurn,
    nb_turn: u8,
}

impl GameBoard {
    pub fn new(player_names: Vec<String>) -> Self {
        let card_infos = CardInfos::init_default();
        // should be cleaner with map collect
        let mut players = vec![];
        for name in player_names.iter() {
            let new_player = PlayerBoard::new(name.clone(), &card_infos);
            players.push(new_player);
        }
        let buy_store = BuyStore::new(&card_infos);
        let current_turn = CurrentTurn {
            player_name: player_names.get(0).unwrap().clone(),
            phase: TurnPhase::Action,
        };
        GameBoard {
            card_infos,
            players,
            buy_store,
            current_turn,
            nb_turn: 1
        }
    }

    fn is_game_ended(&self) -> bool {
        debug!("Checking if game ended");
        let province_id = "province".to_string();
        let nb_province = self.buy_store.get_number_at_pile(&province_id);
        if nb_province == 0 {
            return true
        }
        let empties = self.buy_store.get_empty_piles();
        if empties.len() >= 3 {
            return true
        }
        // todo: remove this
        if self.nb_turn > 10 {
            return true
        }
        false
    }

    fn get_next_player_name(&self) -> String {
        debug!("Looking for next player");
        let current_name = self.current_turn.player_name.clone();
        let order_player: Vec<String> = self.players
            .iter()
            .map(|p| { p.name.clone() })
            .collect();
        if order_player.ends_with(&[current_name.clone()]) {
            return order_player.get(0).unwrap().clone()
        }
        let current_index = order_player
            .iter()
            .position(|pn| { pn.clone() == current_name } )
            .unwrap();
        order_player.get(current_index + 1).unwrap().clone()
    }

    fn run_player_turn(&mut self, player_name: String) {
        info!("Starting turn of {:?}", player_name);
    }

    pub fn start_game(&mut self) {
        info!("Starting game");
        let mut next_player = self.current_turn.player_name.clone();
        while !self.is_game_ended() {
            self.run_player_turn(next_player);
            next_player = self.get_next_player_name();
            self.nb_turn += 1;
        }
        info!("End of game");
    }
}