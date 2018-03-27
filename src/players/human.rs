use std::io;
use piece::Piece;
use board::{Board, Move};
use game::Play;

pub struct Human;
impl Play for Human {
    fn play(&mut self, piece: Piece, board: &Board) -> Option<Move> {
        let moves = board.moves(piece);
        if moves.len() == 0 {
            return None;
        }
        for (idx, mov) in moves.iter().enumerate() {
            println!("{}) {}", idx, mov.pos);
        }
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let idx: Result<usize, _> = input.trim().parse();
            if let Ok(idx) = idx {
                if idx < moves.len() {
                    return Some(moves[idx].clone());
                }
            }
        }
    }
}
