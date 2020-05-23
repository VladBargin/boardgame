use cargo::chess::chess_piece::Pos;
use cargo::chess::chess_piece::ChessPiece;

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

        occ.push((self.p.0, self.p.1));
        occ.push((self.p.0 + 1, self.p.1 + 1));
        occ.push((self.p.0 - 1, self.p.1 + 1));

        return occ;
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_op() {
    }
}