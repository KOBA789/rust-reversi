use std::cmp;
use piece::Piece;
use board::{Board, Move};
use game::Play;

pub struct AlphaBetaPlayer {
    depth: usize,
}
impl AlphaBetaPlayer {
    pub fn new(depth: usize) -> Self {
        AlphaBetaPlayer { depth }
    }

    fn evaluate(&self, piece: Piece, board: &Board) -> i8 {
        match piece {
            Piece::Black => board.black as i8 - board.white as i8,
            Piece::White => board.white as i8 - board.black as i8,
        }
    }

    fn alphabeta(&self, piece: Piece, board: &Board, mut al: i8, be: i8, depth: usize) -> (i8, Option<Move>) {
        if depth == 0 {
            return (self.evaluate(piece, &board), None);
        }
        let moves = board.moves(piece);
        if moves.len() == 0 {
            return (self.evaluate(piece, &board), None);
        }
        let mut best = (-127, None);
        for mov in moves {
            let mut board = board.clone();
            board.do_move(piece, &mov);
            let (score, _) = self.alphabeta(piece.opponent(), &board, -be, -al, depth - 1);
            if -score > best.0 {
                best = (-score, Some(mov));
            }
            al = cmp::max(al, -score);
            if al >= be {
                break;
            }
        }
        best
    }
}
impl Play for AlphaBetaPlayer {
    fn play(&mut self, piece: Piece, board: &Board) -> Option<Move> {
        let depth = self.depth;
        let (_, mov) = self.alphabeta(piece, &board, -127, 127, depth);
        mov
    }
}
