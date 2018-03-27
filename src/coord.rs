use std::ops::{Add, AddAssign};
use std::fmt;

/// ベクトルを表現する構造体
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coord(pub i8, pub i8);

/// `+` 演算子のオーバーロード
impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let Coord(x1, y1) = self;
        let Coord(x2, y2) = rhs;
        Coord(x1 + x2, y1 + y2)
    }
}
/// `+=` 演算子のオーバーロード
impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        let &mut Coord(ref mut x1, ref mut y1) = self;
        let Coord(x2, y2) = rhs;
        *x1 += x2;
        *y1 += y2;
    }
}
const X_AXIS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", X_AXIS[self.0 as usize], self.1 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_add() {
        let a = Coord(5, 5);
        let b = Coord(-2, 3);
        let actual = a + b;
        let expected = Coord(3, 8);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_coord_add_assign() {
        let mut actual = Coord(5, 5);
        actual += Coord(-2, 3);
        let expected = Coord(3, 8);
        assert_eq!(expected, actual);
    }
}
