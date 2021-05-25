use crate::identities::Identity;
use crate::inverse::Inverse;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// This type is intended to be treated as the type representing the rational points on some plane curves.
///
/// More precisely, let S is the set of type ```T``` object (S may be a field),
/// then ```RationalPoint::Point(x,y)``` is corresponding to (x,y) in S^2 and
/// ```RationalPoint::O``` is corresponding to the unique point at infinity.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum RationalPoint<T> {
    Point(T, T),
    O,
}

impl<T> RationalPoint<T> {
    /// Constructor.
    pub fn new(x: T, y: T) -> Self {
        RationalPoint::Point(x, y)
    }
}

/// Implementation of ```Display```.
impl<T: fmt::Display> fmt::Display for RationalPoint<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RationalPoint::O => write!(f, "O"),
            RationalPoint::Point(x, y) => write!(f, "({}, {})", x, y),
        }
    }
}

impl<
        T: Add<Output = T>
            + Mul<Output = T>
            + Sub<Output = T>
            + Copy
            + Eq
            + Inverse
            + Identity
            + Neg<Output = T>,
    > RationalPoint<T>
{
    /// An addition of rational points on an elliptic curve.
    pub fn add_rational_points(&self, rhs: &Self, a: T) -> Self {
        match *self {
            RationalPoint::O => *rhs,
            RationalPoint::Point(x1, y1) => match *rhs {
                RationalPoint::O => RationalPoint::Point(x1, y1),
                RationalPoint::Point(x2, y2) => {
                    if x1 == x2 {
                        if y1 == -y2 {
                            RationalPoint::O
                        } else {
                            let id = T::identity();
                            let m = ((id + id + id) * x1 * x1 + a)
                                * ((id + id) * y1).inverse().unwrap();
                            RationalPoint::Point(m * m - x1 - x1, m * (x1 - m * m + x1 + x1) - y1)
                        }
                    } else {
                        let m = (y2 - y1) * ((x2 - x1).inverse().unwrap());
                        RationalPoint::Point(m * m - x1 - x2, m * (x1 - m * m + x1 + x2) - y1)
                    }
                }
            },
        }
    }
}

impl<
        T: Add<Output = T>
            + Mul<Output = T>
            + Sub<Output = T>
            + Copy
            + Eq
            + Inverse
            + Identity
            + Neg<Output = T>,
    > RationalPoint<T>
{
    /// Calculate nP by repeated squaring
    /// where n is a positive integer and P is a rational point on an elliptic curve.
    pub fn multiply_rational_point(&self, a: T, n: u64) -> Self {
        match *self {
            RationalPoint::O => RationalPoint::O,
            RationalPoint::Point(_, _) => {
                let mut res = RationalPoint::O;
                let mut now = *self;
                let mut m = n;
                loop {
                    if m == 0 {
                        break;
                    }
                    if m % 2 == 1 {
                        res = res.add_rational_points(&now, a);
                    }
                    now = now.add_rational_points(&now, a);
                    m = m / 2;
                }
                res
            }
        }
    }
}
