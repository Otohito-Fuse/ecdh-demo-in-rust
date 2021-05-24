use crate::characteristic::Characteristic;
use crate::identities::{Identity, Zero};
use crate::inverse::Inverse;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Elements of ```Z / (MOD)Z```.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ModInt<const MOD: u64> {
    representative: u64,
}

impl<const MOD: u64> ModInt<MOD> {
    /// Constructor.
    pub fn new(n: u64) -> Self {
        ModInt {
            representative: n % MOD,
        }
    }

    /// Make it ```u64``` type.
    pub fn to_int(&self) -> u64 {
        self.representative
    }

    /// Culculate exponentiation by repeated squaring.
    pub fn power(&self, n: u64) -> Self {
        let mut res = 1;
        let mut a = self.representative;
        let mut m = n;
        loop {
            if m == 0 {
                break;
            }
            if m % 2 == 1 {
                res = (res * a) % MOD;
            }
            a = (a * a) % MOD;
            m = m / 2;
        }
        ModInt {
            representative: res,
        }
    }
}

/// Implementation of ```Display```.
impl<const MOD: u64> fmt::Display for ModInt<MOD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.representative)
    }
}

/// Implementation of ```Add```.
impl<const MOD: u64> Add for ModInt<MOD> {
    type Output = Self;
    /// Overloading the operator ```+```.
    fn add(self, rhs: Self) -> Self::Output {
        ModInt {
            representative: (self.representative + rhs.representative) % MOD,
        }
    }
}

/// Implementation of ```AddAssign```.
impl<const MOD: u64> AddAssign for ModInt<MOD> {
    /// Overloading the operator ```+=```.
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            representative: (self.representative + other.representative) % MOD,
        };
    }
}

/// Implementation of ```Sub```.
impl<const MOD: u64> Sub for ModInt<MOD> {
    type Output = Self;
    /// Overloading the operator ```-```.
    fn sub(self, rhs: Self) -> Self::Output {
        ModInt {
            representative: (self.representative + MOD - rhs.representative) % MOD,
        }
    }
}

/// Implementation of ```SubAssign```.
impl<const MOD: u64> SubAssign for ModInt<MOD> {
    /// Overloading the operator ```-=```.
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            representative: (self.representative + MOD - other.representative) % MOD,
        };
    }
}

/// Implementation of ```Mul```.
impl<const MOD: u64> Mul for ModInt<MOD> {
    type Output = Self;
    /// Overloading the operator ```*```.
    fn mul(self, rhs: Self) -> Self::Output {
        ModInt {
            representative: (self.representative * rhs.representative) % MOD,
        }
    }
}

/// Implementation of ```MulAssign```.
impl<const MOD: u64> MulAssign for ModInt<MOD> {
    /// Overloading the operator ```*=```.
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            representative: (self.representative * other.representative) % MOD,
        };
    }
}

/// Implementation of ```Neg```.
impl<const MOD: u64> Neg for ModInt<MOD> {
    type Output = Self;
    /// Overloading the operator ```-```.
    fn neg(self) -> Self {
        ModInt::<MOD>::new(MOD - self.representative)
    }
}

/// Implementation of ```Zero``` defined in ```identities.rs```.
impl<const MOD: u64> Zero for ModInt<MOD> {
    /// A function that returns an object corresponding to ```0``` in ```Z / (MOD)Z```.
    fn zero() -> Self {
        ModInt::new(0)
    }
}

/// Implementation of ```Identity``` defined in ```identities.rs```.
impl<const MOD: u64> Identity for ModInt<MOD> {
    /// A function that returns an object corresponding to ```1``` in ```Z / (MOD)Z```.
    fn identity() -> Self {
        ModInt::new(1)
    }
}

/// Implementation of ```Inverse``` defined in ```inverse.rs```.
impl<const MOD: u64> Inverse for ModInt<MOD> {
    /// A function that returns an object corresponding to ```x^(-1)``` in ```Z / (MOD)Z```.
    /// This function works well when ```MOD``` is prime.
    fn inverse(self) -> Option<ModInt<MOD>> {
        let n = self.to_int();
        if num::Integer::gcd(&n, &MOD) != 1 {
            None
        } else {
            let ret = self.power(MOD - 2);
            Some(ret)
        }
    }
}

/// Implementation of ```Characteristic``` defined in ```characteristic.rs```.
impl<const MOD: u64> Characteristic for ModInt<MOD> {
    /// A function that returns the characteristic of the fields dealing with.
    fn characteristic() -> u64 {
        MOD
    }
}
