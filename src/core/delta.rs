use std::ops::{Add, Mul, Neg, Sub};

/// A 2D vector representing movement on the chess board.
/// Used for piece movements, directions, and offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Delta {
    pub dx: i8,
    pub dy: i8,
}

impl Delta {
    /// Creates a new Delta.
    #[inline]
    pub const fn new(dx: i8, dy: i8) -> Self {
        Self { dx, dy }
    }

    /// Returns the zero vector.
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0, 0)
    }

    /// Returns the Manhattan distance (|dx| + |dy|).
    #[inline]
    pub fn manhattan_distance(self) -> u8 {
        self.dx.unsigned_abs() + self.dy.unsigned_abs()
    }

    /// Returns the Chebyshev distance (max(|dx|, |dy|)).
    #[inline]
    pub fn chebyshev_distance(self) -> u8 {
        self.dx.unsigned_abs().max(self.dy.unsigned_abs())
    }

    /// Returns true if this vector is on the same diagonal, file, or rank as another.
    pub fn is_collinear_with(self, other: Delta) -> bool {
        if self.dx == 0 && other.dx == 0 {
            return true; // Same file
        }
        if self.dy == 0 && other.dy == 0 {
            return true; // Same rank
        }
        if self.dx.abs() == self.dy.abs() && other.dx.abs() == other.dy.abs() {
            // Both on diagonals - check if same diagonal direction
            return self.dx.signum() * other.dx.signum() == self.dy.signum() * other.dy.signum()
                && self.dx.signum() == other.dx.signum();
        }
        false
    }

    /// Returns a normalized direction vector.
    /// For diagonal/orthogonal moves, returns (-1, 0, 1) for each component.
    pub fn normalize(self) -> Self {
        Self::new(self.dx.signum(), self.dy.signum())
    }
}

impl Add for Delta {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.dx + rhs.dx, self.dy + rhs.dy)
    }
}

impl Sub for Delta {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.dx - rhs.dx, self.dy - rhs.dy)
    }
}

impl Neg for Delta {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.dx, -self.dy)
    }
}

impl Mul<i8> for Delta {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i8) -> Self::Output {
        Self::new(self.dx * rhs, self.dy * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(Delta::new(3, 4).manhattan_distance(), 7);
        assert_eq!(Delta::new(-2, 5).manhattan_distance(), 7);
    }

    #[test]
    fn test_chebyshev_distance() {
        assert_eq!(Delta::new(3, 4).chebyshev_distance(), 4);
        assert_eq!(Delta::new(-2, 5).chebyshev_distance(), 5);
    }

    #[test]
    fn test_normalize() {
        assert_eq!(Delta::new(3, 0).normalize(), Delta::new(1, 0));
        assert_eq!(Delta::new(-5, 5).normalize(), Delta::new(-1, 1));
        assert_eq!(Delta::new(0, 0).normalize(), Delta::new(0, 0));
    }

    #[test]
    fn test_is_collinear() {
        // Same file (dx = 0)
        assert!(Delta::new(0, 3).is_collinear_with(Delta::new(0, 5)));

        // Same rank (dy = 0)
        assert!(Delta::new(3, 0).is_collinear_with(Delta::new(5, 0)));

        // Same diagonal
        assert!(Delta::new(2, 2).is_collinear_with(Delta::new(3, 3)));

        // Not collinear
        assert!(!Delta::new(1, 2).is_collinear_with(Delta::new(2, 1)));
    }

    #[test]
    fn test_arithmetic() {
        let a = Delta::new(1, 2);
        let b = Delta::new(3, 4);

        assert_eq!(a + b, Delta::new(4, 6));
        assert_eq!(b - a, Delta::new(2, 2));
        assert_eq!(-a, Delta::new(-1, -2));
        assert_eq!(a * 3, Delta::new(3, 6));
    }
}
