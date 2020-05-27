use crate::chess::board::Board;
use text_io::read;

pub mod chess;

// Все фигуры белые
fn main() {

    println!("Input n: ");

    let mut wanted: Vec<String> = Vec::new();
    let n = read!();
    for _i in 0..n {
        let s: String = read!();
        wanted.push(s);
    }
    let mut board = Board::new(&wanted);


    // Формат такой:
    // 0 x y piece  - добавить
    // 1 x y        - удалить

    board.print_board();
    loop {
        let action: u8 = read!();
        let y: i8 = read!();
        let x: i8 = read!();
        if action == 0 {
            let piece: String = read!();
            board.add_piece((x, y), piece);
        } else {
            board.remove_piece((x, y));
        }
        let res = board.game_over();
        println!("Game over: {}", res);
        board.print_board();
        if res {
            break;
        }
    }
}
