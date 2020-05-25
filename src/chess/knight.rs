use crate::chess::chess_piece::Pos;
use crate::chess::chess_piece::ChessPiece;
use std::collections::HashSet;

pub struct Knight {
    pos: Pos
}

impl ChessPiece for Knight {

    fn new(p: Pos) -> Self {
        Self {
            pos: p
        }
    }

    fn get_occupied(&self) -> HashSet<Pos> {
        let mut occ: HashSet<Pos> = HashSet::new();

        occ.insert((self.pos.0, self.pos.1));
        occ.insert((self.pos.0 + 2, self.pos.1 + 1));
        occ.insert((self.pos.0 + 2, self.pos.1 - 1));
        occ.insert((self.pos.0 + 1, self.pos.1 - 2));
        occ.insert((self.pos.0 - 1, self.pos.1 - 2));
        occ.insert((self.pos.0 - 2, self.pos.1 - 1));
        occ.insert((self.pos.0 - 2, self.pos.1 + 1));
        occ.insert((self.pos.0 - 1, self.pos.1 + 2));
        occ.insert((self.pos.0 + 1, self.pos.1 + 2));

        return occ;
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_go() {

    }
}