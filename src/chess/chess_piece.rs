use std::collections::HashSet;

pub type Pos = (u8, u8);

pub trait ChessPiece {
    fn get_occupied (&self) -> HashSet<Pos>;
}

