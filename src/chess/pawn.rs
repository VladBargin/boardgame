use crate::chess::chess_piece::ChessPiece;
use crate::chess::chess_piece::Pos;

pub struct Pawn {
    pos: Pos
}

impl ChessPiece for Pawn {

    fn new(p: Pos) -> Self {
        Self {
            pos: p
        }
    }

    fn get_occupied(&self) -> Vec<Pos> {
        let mut occ: Vec<Pos> = Vec::new();

        occ.push((self.pos.0, self.pos.1));
        occ.push((self.pos.0 + 1, self.pos.1 + 1));
        occ.push((self.pos.0 - 1, self.pos.1 + 1));

        return occ;
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_op() {
    }
}