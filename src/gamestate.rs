use rand::{ thread_rng, seq::SliceRandom };


#[derive(Clone)]
pub struct Card {
    pub num: i32
}

impl Card {
    pub fn value(card: &Card) -> i32 { (card.num+6-1) / 6 }
}


#[derive(Clone)]
pub struct PlayedCard {
    pub card: Card,
    pub own: i32
}


#[derive(Clone)]
pub struct GameState {
    pub board: Vec<Vec<PlayedCard>>,
    pub hands: Vec<Vec<Card>>,
    pub deck: Vec<Card>,
    pub turn: usize
}

impl GameState {
    pub fn new(players: i32) -> GameState {
        let mut gst = GameState {
            board: vec![vec![], vec![], vec![]],
            hands: Vec::new(),
            deck: (0..54).map(|x| Card {num: x}).collect(),
            turn: 0
        };
        gst.deck.shuffle(&mut thread_rng());
        for j in 0..players { 
            gst.hands.push(Vec::new());
            for _i in 0..13 {
                GameState::draw(&mut gst, j);
            }
        }
        gst
    }

    pub fn get_current_player (&self) -> usize { self.turn % self.hands.len() }

    pub fn draw (&mut self, own: i32) {
        let own_index : usize = own.try_into().unwrap(); 
        let card = self.deck.remove(0);
        self.hands[own_index].push(card);
    }

    pub fn discard (&mut self, own: i32, num: i32) {
        let own_index : usize = own.try_into().unwrap(); 
        self.hands[own_index].retain(|card| card.num != num);
    }

    pub fn play (&mut self, own: i32, num: i32) {
        self.discard(own, num);
        let line_index : usize = (num/18).try_into().unwrap(); 
        for (_i, mut pc) in self.board[line_index]
                .iter_mut().rev().enumerate() {
            if pc.card.num > num {
               pc.own = own;
            }
            else { break; }
        }
        self.board[line_index].push(PlayedCard {
            card: Card { num },
            own: own
        });
    }

    pub fn can_play (&self, own: i32, num: i32) -> bool {
        let line_index : usize = (num/18).try_into().unwrap();
        if let Some(last) = self.board[line_index].last() {
            let (lo, hi) = self.lohi(line_index);
            if num > hi {
                true
            }
            else if num > lo {
                last.own != own
            } 
            else { false }
        }
        else { true }
    }

    pub fn can_draw (&self, own: i32) -> bool {
        self.deck.len() > 0
    }

    pub fn lohi (&self, line: usize) -> (i32, i32) {
        let mut lo : i32 = -1;
        let hi : i32 = match self.board[line].last() {
            Some(pc) => pc.card.num,
            None => 0
        };
        for (_i, pc) in self.board[line].iter().rev().enumerate() {
            if pc.card.num < hi {
               lo = pc.card.num;
               break;
            }
        }
        (lo, hi)
    }

    pub fn value(&self, own: i32) -> i32 {
        let mut tot = 0;
        for line in &self.board {
            for played in line {
                if played.own == own {
                    tot += Card::value(&played.card);
                }
            }
        }
        tot
    }

    pub fn show(&self) {
        print!("deck: [");
        for (i, card) in self.deck.iter().enumerate() {
            print!("{}", card.num);
            if i < self.deck.len()-1 { print!(", "); }
        }
        println!("]");

        println!("board: ");
        for (_i, line) in self.board.iter().enumerate() {
            print!("  line: [");
            for (j, playedcard) in line.iter().enumerate() {
                print!("({}:{})", playedcard.card.num, playedcard.own);
                if j < line.len()-1 { print!(", "); }
            }
            println!("]");
        }

        println!("hands: ");
        for (i, hand) in self.hands.iter().enumerate() {
            print!("  hand player {}: [", i);
            for (j, card) in hand.iter().enumerate() {
                print!("{}", card.num);
                if j < hand.len()-1 { print!(", "); }
            }
            println!("]");
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_play_forward() {
        let mut gst = GameState {
            board: vec![vec![], vec![], vec![]],
            hands: vec![vec![Card { num: 0 }]],
            deck: vec![],
            turn: 0
        };
        assert![gst.can_play(0, 0)];
    }

    #[test]
    fn can_play_backward() {
        let mut gst = GameState {
            board: vec![vec![
                PlayedCard { card: Card { num: 1 }, own: 1 }], 
                vec![], vec![]],
            hands: vec![vec![Card { num: 0 }]],
            deck: vec![],
            turn: 0
        };
        assert![gst.can_play(0, 0)];
    }

    #[test]
    fn cannot_play_backward() {
        let mut gst = GameState {
            board: vec![vec![
                PlayedCard { card: Card { num: 1 }, own: 0 }], 
                vec![], vec![]],
            hands: vec![vec![Card { num: 0 }]],
            deck: vec![],
            turn: 0
        };
        assert![!gst.can_play(0, 0)];
    }
}
