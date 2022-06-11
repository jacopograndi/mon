mod gamestate;
mod search;

use crate::{
    gamestate::GameState, 
    search::*
};


fn main() {
    let gst = GameState::new(2);
    gst.show();
    println!("board value: {}\n", gst.value( 0));
    let nexts = gst.next();
    println!("nexts len: {}", nexts.len());
    search(&gst);
}
