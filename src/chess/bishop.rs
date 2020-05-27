use crate::chess::chess_piece::Pos;
use crate::chess::chess_piece::ChessPiece;
use std::collections::HashSet;

pub struct Bishop {
    pos: Pos
}

impl Bishop {
    pub fn new(p: Pos) -> Self {
        Self {
            pos: p
        }
    }
}

impl ChessPiece for Bishop {

    fn get_occupied(&self) -> HashSet<Pos> {
        let mut occ: HashSet<Pos> = HashSet::new();

        for x in 0..8 {
            occ.insert((x, x - self.pos.0 + self.pos.1));
        }

        for y in 0..8 {
            occ.insert((y - self.pos.1 + self.pos.0, y));
        }

        return occ;
    }

    fn get_pos(&self) -> Pos {
        self.pos
    }

    fn get_str(&self) -> String {
        "b".to_string()
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_go() {

    }
}