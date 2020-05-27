use std::collections::HashSet;
use crate::chess::chess_piece::{Pos, ChessPiece};

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
        if self.pos_used(&p) {
            return
        }

        let v: Box<dyn ChessPiece>;
        match ptype.as_str() {
            "p" => {

            }, "K" => {

            }, "k" => {

            }, "b" => {

            }, "r" => {

            }, "Q" => {

            }, _ => ()
        }

    }

    pub fn remove_piece(&mut self, p: Pos) {

    }

    fn is_legal(&self) -> bool {
        false
    }

    fn all_pieces_used(&self) -> bool {
        false
    }

    pub fn game_over(&self) -> bool {
        self.is_legal() && self.all_pieces_used()
    }

    pub fn print_board(&self) {

    }
}