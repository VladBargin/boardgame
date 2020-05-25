use crate::chess::chess_piece::Pos;
use crate::chess::chess_piece::ChessPiece;
use std::collections::HashSet;

pub struct Rook {
    pos: Pos
}

impl ChessPiece for Rook {

    fn new(p: Pos) -> Self {
        Self {
            pos: p
        }
    }

    fn get_occupied(&self) -> HashSet<Pos> {
        let mut occ: HashSet<Pos> = HashSet::new();

        for x in 0..8 {
            occ.insert((x, self.pos.1))
        }

        for y in 0..8 {
            occ.insert((self.pos.0, y))
        }

        return occ;
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_go() {

    }
}