/// 石の色を表す列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Black,
    White,
}
impl Piece {
    /// 逆の色を返す
    pub fn opponent(&self) -> Piece {
        match *self {
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        }
    }
}

pub const N: Option<Piece> = None;
pub const B: Option<Piece> = Some(Piece::Black);
pub const W: Option<Piece> = Some(Piece::White);
