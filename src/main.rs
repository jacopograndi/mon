use rand::{ thread_rng, seq::SliceRandom };

struct Card {
    num: i32
}

impl Card {
    fn value(card: &Card) -> i32 { (card.num+6-1) / 6 }
}


struct PlayedCard {
    card: Card,
    own: i32
}

struct GameState {
    board: Vec<Vec<PlayedCard>>,
    hands: Vec<Vec<Card>>,
    deck: Vec<Card>
}

impl GameState {
    fn new(players: i32) -> GameState {
        let mut gst = GameState {
            board: vec![vec![], vec![], vec![]],
            hands: Vec::new(),
            deck: (0..54).map(|x| Card {num: x}).collect()
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

    fn draw (&mut self, own: i32) {
        let own_index : usize = own.try_into().unwrap(); 
        let card = self.deck.remove(0);
        self.hands[own_index].push(card);
    }

    fn play (&mut self, own: i32, num: i32) {
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

    fn lohi (&mut self, line: usize) -> (i32, i32) {
        let mut lo : i32 = 0;
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

    fn discard (&mut self, own: i32, num: i32) {
        let own_index : usize = own.try_into().unwrap(); 
        self.hands[own_index].retain(|card| card.num != num);
    }

    fn value(&mut self, own: i32) -> i32 {
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

    fn show(&self) {
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


fn main() {
    let mut gst = GameState::new(2);
    println!("board value: {}", gst.value( 0));
    gst.show();
    let num = gst.hands[0][0].num;
    gst.play(0, num);
    let num = gst.hands[0][0].num;
    gst.play(0, num);
    let num = gst.hands[1][0].num;
    gst.play(1, num);
    gst.show();
}
