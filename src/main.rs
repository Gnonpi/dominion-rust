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
each player gets a deck of 7 Copper and 3 Estate.

In common, they have:
* the store to buy cards from

The game ends:
* the pile of province is empty
* 3 piles of supply cards are empty

*/
use log::{info};
use dominion_rust::GameBoard;


fn main() {
    env_logger::init();
    let player_names = vec!["Joel", "Hector"]
        .iter()
        .map(|x| { x.to_string() })
        .collect();
    let mut game = GameBoard::new(player_names);
    info!("Hello, this is");
    info!("{:?}", game);
    game.start_game();
}
