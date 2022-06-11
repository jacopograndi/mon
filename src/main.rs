mod gamestate;
mod search;

use crate::{
    gamestate::GameState, 
    search::*
};


fn main() {
    let gst = GameState::new(2);
    gst.show();
    println!("board value: {}", gst.eval());
    let value = minimax(&gst, 3, true);
    println!("best move value: {}", value);
}
