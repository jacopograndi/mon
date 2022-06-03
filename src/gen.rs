use crate::gamestate::*;


pub fn ply (state : GameState) -> Vec<GameState> {
    let mut nexts = Vec::<GameState>::new();
    let own_index = state.get_current_player();
    let own : i32 = state.get_current_player().try_into().unwrap();

    for card in state.hands[own_index].iter() {
        if state.can_play(own, card.num) {
            let mut next = state.clone();
            next.play(own, card.num);
            next.turn += 1;
            nexts.push(next);
        }

        let mut next = state.clone();
        next.discard(own, card.num);
        next.turn += 1;
        nexts.push(next);
        
        if state.can_draw(own) {
            let mut next = state.clone();
            next.discard(own, card.num);
            next.draw(own);
            next.turn += 1;
            nexts.push(next);
        }
    }
    nexts
}
