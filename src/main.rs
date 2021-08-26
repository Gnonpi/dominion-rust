/*
The turn phases are:
* Action - play action cards from your hand
* Buy - from the common store
* Clean - discard un+played cards and draw 5

Each player has:
* a hand
* a deck
* a pile of discarded card
* currently played card

Each card is either:
* a Treasure
* a Victory
* a Kingdom (actions)
* a Curse

All card have a cost to buy.

Each turn, the player gets one Action and one Buy.
At the beginning of the game,
each player gets a deck of 3 Copper and 3 Estate.

In common, they have:
* the store to buy cards from

The game ends:
* the pile of province is empty
* 3 piles of supply cards are empty

*/

use std::collections::HashMap;

type CardId = String;

enum ActionType {
    None,
    Attack,
    Reaction,
}

enum CardType {
    Victory,
    Treasure,
    Kingdom(ActionType),
    Curse,
}

struct Card {
    card_id: CardId,
    card_type: CardType,
    name: String,
    human_text: String,
    store_cost: u8
}

struct CardInfos {
    cards: HashMap<CardId, Card>
}

struct PlayerBoard {
    remaining_actions: u8,
    remaining_buys: u8,
    name: String,
    deck: Vec<CardId>,
    hand: Vec<CardId>,
    discarded_card: Vec<CardId>,
    currently_played_card: Option<CardId>
}

struct BuyStore {
    piles: HashMap<CardId, u8>
}

enum TurnPhase {
    Action,
    Buying,
    Cleaning,
}

struct CurrentTurn {
    player_name: String,
    phase: TurnPhase,
}

struct GameBoard {
    card_store: CardInfos,
    players: Vec<PlayerBoard>,
    buy_store: BuyStore
}

fn main() {
    println!("Hello, world!");
}
