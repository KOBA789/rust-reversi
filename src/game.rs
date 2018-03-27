use piece::Piece;
use board::{Board, Move};

pub struct Game<P1, P2> {
    board: Board,
    turn: Piece,
    black: P1,
    white: P2,
    is_passed: bool,
}

impl<P1, P2> Game<P1, P2>
where
    P1: Play,
    P2: Play,
{
    pub fn new(black: P1, white: P2) -> Self {
        let board = Board::new();
        Game {
            board,
            turn: Piece::Black,
            black,
            white,
            is_passed: false,
        }
    }

    pub fn step(&mut self) -> bool {
        let mov = match self.turn {
            Piece::Black => self.black.play(self.turn, &self.board),
            Piece::White => self.white.play(self.turn, &self.board),
        };
        match mov {
            Some(mov) => {
                self.board.do_move(self.turn, &mov);
                self.is_passed = false;
            },
            None => {
                if self.is_passed {
                    // game is over
                    return false;
                }
                self.is_passed = true;
            }
        }
        self.turn = self.turn.opponent();
        true
    }

    pub fn print(&self) {
        println!("{}", self.board);
        println!("Turn: {:?}", self.turn);
    }
}

pub trait Play {
    fn play(&mut self, piece: Piece, board: &Board) -> Option<Move>;
}
