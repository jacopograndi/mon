mod gen;
mod gamestate;

use crate::{
    gamestate::GameState, 
    gen::ply
};


fn main() {
    let mut gst = GameState::new(2);
    gst.show();
    println!("board value: {}\n", gst.value( 0));
    let nexts = ply(gst);
    println!("nexts len: {}", nexts.len());
}
