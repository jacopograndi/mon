use rand::{ thread_rng, seq::SliceRandom };
use crate::search::State;


#[derive(Clone)]
pub struct Card {
    pub num: i32
}

impl Card {
    pub fn value(&self) -> i32 { (self.num+6-1) / 6 }
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

    pub fn can_draw (&self) -> bool {
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

impl State for GameState {
    fn next (&self) -> Vec<GameState> {
        let mut nexts = Vec::<GameState>::new();
        let own_index = self.get_current_player();
        let own : i32 = self.get_current_player().try_into().unwrap();

        for card in self.hands[own_index].iter() {
            if self.can_play(own, card.num) {
                let mut next = self.clone();
                next.play(own, card.num);
                next.turn += 1;
                nexts.push(next);
            }

            let mut next = self.clone();
            next.discard(own, card.num);
            next.turn += 1;
            nexts.push(next);
            
            if self.can_draw() {
                let mut next = self.clone();
                next.discard(own, card.num);
                next.draw(own);
                next.turn += 1;
                nexts.push(next);
            }
        }
        nexts
    }

    fn eval(&self) -> i32 {
        let mut others : Vec<i32> = vec![0; self.hands.len()];
        for line in &self.board {
            for played in line {
                let index_from_own : usize = played.own.try_into().unwrap();
                others[index_from_own] += played.card.value();
            }
        }
        let cur_player = (self.turn%self.hands.len()).try_into().unwrap();
        let cur_points = others[cur_player];
        others.remove(cur_player);
        let max = *others.iter().max().unwrap();
        cur_points - max
    }
} 
