use std::collections::HashSet;

pub type Pos = (i8, i8);

pub trait ChessPiece {
    fn get_occupied (&self) -> HashSet<Pos>;
    fn get_pos (&self) -> Pos;
    fn get_str(&self) -> String;
}

