use std::collections::{HashSet, HashMap};
use crate::chess::chess_piece::{Pos, ChessPiece};
use crate::chess::pawn::Pawn;
use crate::chess::knight::Knight;
use crate::chess::king::King;
use crate::chess::bishop::Bishop;
use crate::chess::queen::Queen;
use crate::chess::rook::Rook;

pub struct Board {
    pieces: Vec<Box<dyn ChessPiece>>,
    wanted: Vec<String>
}

impl Board {
    pub fn new(w: &Vec<String>) -> Self {
        Self {
            pieces: Vec::new(),
            wanted: w.clone()
        }
    }

    pub fn pos_used(&self, p: &Pos) -> bool {
        false
    }

    pub fn add_piece(&mut self, p: Pos, ptype: String) {
        if self.pos_used(&p) || p.0 < 0 || p.0 > 7 || p.1 < 0 || p.1 > 7{
            return
        }

        let v: Box<dyn ChessPiece>;
        match ptype.as_str() {
            "p" => {
                v = Box::new(Pawn::new(p))
            }, "K" => {
                v = Box::new(King::new(p))
            }, "k" => {
                v = Box::new(Knight::new(p))
            }, "b" => {
                v = Box::new(Bishop::new(p))
            }, "r" => {
                v = Box::new(Rook::new(p))
            }, "Q" => {
                v = Box::new(Queen::new(p))
            }, _ => return
        }

        self.pieces.push(v);
    }

    pub fn remove_piece(&mut self, p: Pos) {
        for i in 0..self.pieces.len() {
            if self.pieces[i].get_pos() == p {
                self.pieces.remove(i);
                break;
            }
        }
    }

    fn is_legal(&self) -> bool {
        let mut set: HashSet<Pos> = HashSet::new();
        for piece in self.pieces.iter() {
            for p in (piece.get_occupied()).iter() {
                if set.contains(p) {
                    return false;
                }
                set.insert(*p);
            }
        }
        true
    }

    fn all_pieces_used(&self) -> bool {
        let mut mp: HashMap<&str, i32> = HashMap::new();
        for x in self.wanted.iter() {
            mp.insert(x.as_str().clone(), 0);
        }

        for x in self.wanted.iter() {
            *mp.get_mut(x.as_str()).unwrap() += 1;
        }

        for x in self.pieces.iter() {
            println!("Str: {}", x.get_str());
            if mp.contains_key(x.get_str().as_str()) {
                *mp.get_mut(x.get_str().as_str()).unwrap() -= 1;
            } else {
                return false;
            }
        }

        for (x, cnt) in &mp {
            if *cnt != 0 {
                return false;
            }
        }
        true
    }

    pub fn game_over(&self) -> bool {
        let il = self.is_legal();
        let apu = self.all_pieces_used();
        println!("Legal: {}, All pieces used: {}", il, apu);
        il && apu
    }

    pub fn print_board(&self) {
        let mut x: Vec<Vec<&str>> = Vec::new();
        x.resize(8, vec![".", ".", ".", ".", ".", ".", ".", "."]);
        for p in self.pieces.iter() {
            x[(p.get_pos()).0 as usize][(p.get_pos()).1 as usize] = "P";
        }
        for i in 0..8 {
            println!("{:?}", x[i]);
        }
    }
}