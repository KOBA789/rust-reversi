use rayon::prelude::*;
use piece::Piece;
use board::{Board, Move};
use game::Play;

pub struct NegaMaxPlayer {
    depth: usize,
}
impl NegaMaxPlayer {
    pub fn new(depth: usize) -> Self {
        NegaMaxPlayer { depth }
    }

    fn evaluate(&self, piece: Piece, board: &Board) -> i8 {
        match piece {
            Piece::Black => board.black as i8 - board.white as i8,
            Piece::White => board.white as i8 - board.black as i8,
        }
    }

    fn negamax(&self, piece: Piece, board: &Board, depth: usize) -> (i8, Option<Move>) {
        if depth == 0 {
            return (self.evaluate(piece, &board), None);
        }
        let moves = board.moves(piece);
        moves.into_iter().map(|mov| {
            let mut board = board.clone();
            board.do_move(piece, &mov);
            let (score, _) = self.negamax(piece.opponent(), &board, depth - 1);
            (-score, Some(mov))
        }).max_by_key(|&(score, _)| score).unwrap_or((-127, None))
    }

    fn negamax_mt(&self, piece: Piece, board: &Board, depth: usize) -> (i8, Option<Move>) {
        if depth == 0 {
            return (self.evaluate(piece, &board), None);
        }
        let moves = board.moves(piece);
        let (score, mov) = moves.into_par_iter().map(|mov| {
            let mut board = board.clone();
            board.do_move(piece, &mov);
            let (score, _) = self.negamax(piece.opponent(), &board, depth - 1);
            (-score, Some(mov))
        }).max_by_key(|&(score, _)| score).unwrap_or((-127, None));
        (score, mov.map(Clone::clone))
    }
}
impl Play for NegaMaxPlayer {
    fn play(&mut self, piece: Piece, board: &Board) -> Option<Move> {
        let depth = self.depth;
        let (_, mov) = self.negamax_mt(piece, &board, depth);
        mov
    }
}
