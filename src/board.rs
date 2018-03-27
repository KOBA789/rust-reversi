use std::fmt;
use std::ops::{Index, IndexMut};
use smallvec::SmallVec;
use piece::*;
use coord::Coord;

/// 盤面の1辺の長さの定数
const MATRIX_SIZE: usize = 8;

/// 盤面の石の状態のみを保持する構造体
///
/// インデックスによってアクセスできる。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix([[Option<Piece>; MATRIX_SIZE]; MATRIX_SIZE]);

impl Matrix {
    /// 新しい Matrix を生成する
    ///
    /// 盤面の初期状態を返す。
    pub fn new() -> Self {
        Matrix([
            [N, N, N, N, N, N, N, N],
            [N, N, N, N, N, N, N, N],
            [N, N, N, N, N, N, N, N],
            [N, N, N, W, B, N, N, N],
            [N, N, N, B, W, N, N, N],
            [N, N, N, N, N, N, N, N],
            [N, N, N, N, N, N, N, N],
            [N, N, N, N, N, N, N, N],
        ])
    }

    /// 第一引数に与えられた座標が盤面のサイズに収まっているかどうかを判定する
    pub fn is_in_range(&self, pos: Coord) -> bool {
        let Coord(x, y) = pos;
        0 <= x && x < self.size() as i8 && 0 <= y && y < self.size() as i8
    }

    /// 盤面の1辺の長さを返す
    pub fn size(&self) -> usize {
        MATRIX_SIZE
    }
}
/// `[]` 演算子のオーバーロード
impl Index<Coord> for Matrix {
    type Output = Option<Piece>;
    /// 第一引数に与えられた座標の状態を返す
    ///
    /// 座標が盤面の範囲外であった場合は None が返る。
    fn index(&self, index: Coord) -> &Self::Output {
        if !self.is_in_range(index) {
            return &None;
        }
        let Coord(x, y) = index;
        &self.0[y as usize][x as usize]
    }
}
/// `[]=` 演算子のオーバーロード
impl IndexMut<Coord> for Matrix {
    /// 第一引数に与えられた座標の状態へのミュータブルな参照を返す
    ///
    /// 座標が盤面の範囲外であった場合の挙動は未定義
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let Coord(x, y) = index;
        &mut self.0[y as usize][x as usize]
    }
}
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "  a b c d e f g h")?;
        writeln!(f, " +-+-+-+-+-+-+-+-+")?;
        for y in 0..MATRIX_SIZE {
            write!(f, "{}|", y + 1)?;
            for x in 0..MATRIX_SIZE {
                let cell = self[Coord(x as i8, y as i8)];
                match cell {
                    B => write!(f, "X|")?,
                    W => write!(f, "O|")?,
                    _ => write!(f, " |")?,
                }
            }
            writeln!(f)?;
            writeln!(f, " +-+-+-+-+-+-+-+-+")?;
        }
        Ok(())
    }
}

/// 1つもひっくり返すことのできない手の flips の値
const ZERO_FLIP: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

/// 「手」を表す構造体
///
/// 手を打つ座標と、8方向それぞれについて、そこに手を打った際にひっくり返すことができる石の数を保持している
#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    /// 手を打つ座標
    pub pos: Coord,
    /// 8方向それぞれの、pos に打った際にひっくり返すことができる石の数
    ///
    /// DIRECTIONS と同じ順番で並んでいる
    flips: [u8; 8],
}
impl Move {
    /// その手が合法=1以上の石をひっくり返せるかどうかを返す
    pub fn is_legal(&self) -> bool {
        self.flips != ZERO_FLIP
    }
}

/// 候補手のリスト
pub type Moves<'a> = SmallVec<[Move; MATRIX_SIZE * MATRIX_SIZE]>;

/// 隣接する8マスを指すそれぞれのベクトル
///
/// ```
/// +---+---+---+
/// | 0 | 1 | 2 |
/// +---+---+---+
/// | 3 | X | 4 |
/// +---+---+---+
/// | 5 | 6 | 7 |
/// +---+---+---+
/// ```
const DIRECTIONS: [Coord; 8] = [
    Coord(-1, -1), //左上
    Coord(0, -1),  //真上
    Coord(1, -1),  //右上
    Coord(-1, 0),  //真左
    Coord(1, 0),   //真右
    Coord(-1, 1),  //左下
    Coord(0, 1),   //真下
    Coord(1, 1),   //右下
];

/// 盤面にリバーシの操作を実装したもの
#[derive(Clone)]
pub struct Board {
    matrix: Matrix,
    pub black: u8,
    pub white: u8,
}

impl Board {
    pub fn new() -> Self {
        Board {
            matrix: Matrix::new(),
            black: 2,
            white: 2,
        }
    }

    /// 指定の色の石を指定の位置に置いたとき、指定の方向へひっくり返せる石の数を返す
    ///
    /// * `piece` - 置く石の色
    /// * `pos` - 石を置く位置
    /// * `dir` - ひっくり返せる石を探す方向。`DIRECTIONS` の要素のいずれかが渡される
    fn get_flip(&self, piece: Piece, mut pos: Coord, dir: Coord) -> u8 {
        let me = Some(piece);
        let mut count = 0;
        loop {
            pos += dir;
            let cell = self.matrix[pos];
            if cell == N {
                return 0;
            }
            if cell == me {
                return count;
            }
            count += 1;
        }
    }

    /// 指定の色の石を指定の位置に置いたときの `Move` を返す
    ///
    /// 戻り値の `Move` には8方向分の `get_flip` の結果が含まれる
    fn get_move(&self, piece: Piece, pos: Coord) -> Move {
        if self.matrix[pos] != N {
            return Move {
                pos,
                flips: ZERO_FLIP,
            };
        }
        let mut flips = [0; 8];
        for (dir, flip) in DIRECTIONS.iter().zip(flips.iter_mut()) {
            *flip = self.get_flip(piece, pos, *dir);
        }
        Move { pos, flips }
    }

    /// 合法な Move のリストを返す
    ///
    /// 盤面の左上から右下まで走査して、合法手を探し出す
    pub fn moves(&self, piece: Piece) -> Moves {
        let mut moves = SmallVec::new();
        for y in 0..self.matrix.size() {
            for x in 0..self.matrix.size() {
                let mov = self.get_move(piece, Coord(x as i8, y as i8));
                if mov.is_legal() {
                    moves.push(mov);
                }
            }
        }
        moves
    }

    /// 指定の色のカウンタへのミュータブルな参照を返す
    fn count_mut(&mut self, piece: Piece) -> &mut u8 {
        unimplemented!();
    }

    /// 石を指定の位置から指定の方向へ指定の数だけ指定の色にひっくり返す
    ///
    /// * `piece` - ひっくり返したあとの色
    /// * `pos` - 石を置く位置
    /// * `dir` - ひっくり返す方向。`DIRECTIONS` の要素のいずれかが渡される
    /// * `flip` - ひっくり返す枚数
    ///
    /// ひっくり返した分だけ `black`/`white` の数を増減させる必要がある
    fn do_flip(&mut self, piece: Piece, mut pos: Coord, dir: Coord, flip: u8) {
        unimplemented!();
    }

    /// 指定の色で指定の「手」を打つ
    pub fn do_move(&mut self, piece: Piece, mov: &Move) {
        unimplemented!();
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.matrix)?;
        write!(f, "B {} - {} W", self.black, self.white)
    }
}

#[cfg(test)]
mod tests {
    use piece::*;
    use super::*;

    const SAMPLE_MATRIX: Matrix = Matrix([
        [N, N, W, W, N, B, N, N],
        [N, N, N, W, W, B, N, B],
        [B, W, W, W, B, W, W, B],
        [N, B, W, B, B, W, W, B],
        [B, B, B, W, W, B, W, B],
        [W, N, B, W, W, W, B, B],
        [N, N, B, W, W, W, N, B],
        [N, B, B, B, B, B, N, N],
    ]);
    const SAMPLE_BOARD: Board = Board {
        matrix: SAMPLE_MATRIX,
        black: 25,
        white: 22,
    };

    #[test]
    fn test_matrix_is_in_range() {
        let matrix = Matrix::new();
        assert_eq!(true, matrix.is_in_range(Coord(3, 3)));
        assert_eq!(true, matrix.is_in_range(Coord(0, 0)));
        assert_eq!(true, matrix.is_in_range(Coord(7, 7)));
        assert_eq!(false, matrix.is_in_range(Coord(-1, 0)));
        assert_eq!(false, matrix.is_in_range(Coord(0, -1)));
        assert_eq!(false, matrix.is_in_range(Coord(8, 0)));
        assert_eq!(false, matrix.is_in_range(Coord(0, 8)));
    }

    #[test]
    fn test_matrix_index() {
        let matrix = Matrix::new();
        assert_eq!(N, matrix[Coord(0, 0)]);
        assert_eq!(W, matrix[Coord(3, 3)]);
        assert_eq!(B, matrix[Coord(4, 3)]);
        assert_eq!(N, matrix[Coord(8, 0)]);
        assert_eq!(N, matrix[Coord(0, -1)]);
    }

    #[test]
    fn test_matrix_index_mut() {
        let mut matrix = Matrix::new();
        assert_eq!(N, matrix[Coord(0, 0)]);
        matrix[Coord(0, 0)] = B;
        assert_eq!(B, matrix[Coord(0, 0)]);
    }

    #[test]
    fn test_move_is_legal() {
        let b = Board::new();
        assert_eq!(
            false,
            b.get_move(Piece::Black, Coord(0, 0)).is_legal(),
            "black on upper left corner",
        );
        assert_eq!(
            true,
            b.get_move(Piece::Black, Coord(3, 2)).is_legal(),
            "black on top of upper left white",
        );
        assert_eq!(
            false,
            b.get_move(Piece::White, Coord(3, 2)).is_legal(),
            "white on top of upper left white",
        );
        assert_eq!(
            true,
            b.get_move(Piece::White, Coord(4, 2)).is_legal(),
            "white on right of upper right black",
        );
    }

    #[test]
    fn test_board_get_flip() {
        let b = Board::new();
        let right = b.get_flip(Piece::Black, Coord(2, 3), Coord(1, 0));
        assert_eq!(1, right, "right");
        let left = b.get_flip(Piece::Black, Coord(2, 3), Coord(-1, 0));
        assert_eq!(0, left, "left");
        let bottom = b.get_flip(Piece::Black, Coord(2, 3), Coord(0, 1));
        assert_eq!(0, bottom, "bottom");
    }

    #[test]
    fn test_board_get_move() {
        let b = SAMPLE_BOARD.clone();
        let actual = b.get_move(Piece::White, Coord(0, 3));
        let expected = Move {
            pos: Coord(0, 3),
            flips: [
                0, 0, 0,
                0,    1,
                0, 1, 2,
            ],
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_board_moves() {
        let b = SAMPLE_BOARD.clone();
        let actual = b.moves(Piece::White);
        let candidates = vec![
            Move {
                pos: Coord(4, 0),
                flips: [
                    0, 0, 0,
                    0,    0,
                    0, 0, 1,
                ],
            },
            Move {
                pos: Coord(6, 1),
                flips: [
                    0, 0, 0,
                    1,    0,
                    0, 0, 0,
                ],
            },
            Move {
                pos: Coord(0, 3),
                flips: [
                    0, 0, 0,
                    0,    1,
                    0, 1, 2,
                ],
            },
            Move {
                pos: Coord(1, 5),
                flips: [
                    0, 2, 0,
                    0,    1,
                    0, 0, 0,
                ],
            },
            Move {
                pos: Coord(1, 6),
                flips: [
                    0, 0, 1,
                    0,    1,
                    0, 0, 0,
                ],
            },
            Move {
                pos: Coord(6, 6),
                flips: [
                    0, 1, 0,
                    0,    0,
                    0, 0, 0,
                ],
            },
        ];
        assert_eq!(6, actual.len());
        assert!(actual.contains(&candidates[0]));
        assert!(actual.contains(&candidates[1]));
        assert!(actual.contains(&candidates[2]));
        assert!(actual.contains(&candidates[3]));
        assert!(actual.contains(&candidates[4]));
        assert!(actual.contains(&candidates[5]));
    }

    #[test]
    fn test_board_count_mut() {
        let mut b = Board::new();
        assert_eq!(2, *b.count_mut(Piece::White));
        *b.count_mut(Piece::White) = 5;
        assert_eq!(5, *b.count_mut(Piece::White));
    }

    #[test]
    fn test_do_flip() {
        let mut b = SAMPLE_BOARD.clone();
        b.do_flip(Piece::White, Coord(0, 3), Coord(1, 1), 2);
        assert_eq!(24, b.white);
        assert_eq!(23, b.black);
        assert_eq!(
            Matrix([
                [N, N, W, W, N, B, N, N],
                [N, N, N, W, W, B, N, B],
                [B, W, W, W, B, W, W, B],
                [N, B, W, B, B, W, W, B],
                [B, W, B, W, W, B, W, B],
                [W, N, W, W, W, W, B, B],
                [N, N, B, W, W, W, N, B],
                [N, B, B, B, B, B, N, N],
            ]),
            b.matrix,
        );
    }

    #[test]
    fn test_board_do_move() {
        let mut b = SAMPLE_BOARD.clone();
        b.do_move(
            Piece::White,
            &Move {
                pos: Coord(0, 3),
                flips: [
                    0, 0, 0,
                    0,    1,
                    0, 1, 2,
                ],
            },
        );
        assert_eq!(27, b.white);
        assert_eq!(21, b.black);
        assert_eq!(
            Matrix([
                [N, N, W, W, N, B, N, N],
                [N, N, N, W, W, B, N, B],
                [B, W, W, W, B, W, W, B],
                [W, W, W, B, B, W, W, B],
                [W, W, B, W, W, B, W, B],
                [W, N, W, W, W, W, B, B],
                [N, N, B, W, W, W, N, B],
                [N, B, B, B, B, B, N, N],
            ]),
            b.matrix,
        );
    }
}
