use crate::characteristic::Characteristic;
use crate::identities::{Identity, Zero};
use crate::inverse::Inverse;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Elements in ```R\[x\]/(x^2 + 1)```
/// where ```R``` is a ring consisting of the objects of type ```T```.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Complex<T> {
    real: T,
    imaginary: T,
}

impl<T> Complex<T> {
    /// Constructor.
    pub fn new(real: T, imaginary: T) -> Self {
        Self {
            real: real,
            imaginary: imaginary,
        }
    }
}

/// Implementation of ```Display```.
impl<T: fmt::Display + Zero + Eq> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imaginary == T::zero() {
            write!(f, "{}", self.real)
        } else if self.real == T::zero() {
            write!(f, "{}i", self.imaginary)
        } else {
            write!(f, "({} + {}i)", self.real, self.imaginary)
        }
    }
}

/// Implementation of ```Add```.
impl<T: Copy + Add<Output = T> + Eq> Add for Complex<T> {
    type Output = Self;
    /// Overloading the operator ```+```.
    fn add(self, rhs: Self) -> Self {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

/// Implementation of ```AddAssign```.
impl<T: Copy + Add<Output = T> + Eq> AddAssign for Complex<T> {
    /// Overloading the operator ```+=```.
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

/// Implementation of ```Sub```.
impl<T: Copy + Sub<Output = T> + Eq> Sub for Complex<T> {
    type Output = Self;
    /// Overloading the operator ```-```.
    fn sub(self, rhs: Self) -> Self {
        Self {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

/// Implementation of ```SubAssign```.
impl<T: Copy + Sub<Output = T> + Eq> SubAssign for Complex<T> {
    /// Overloading the operator ```-=```.
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }
}

/// Implementation of ```Mul```.
impl<T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Eq> Mul for Complex<T> {
    type Output = Self;
    /// Overloading the operator ```*```.
    fn mul(self, rhs: Self) -> Self {
        Self {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.imaginary * rhs.real + self.real * rhs.imaginary,
        }
    }
}

/// Implementation of ```MulAssign```.
impl<T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Eq> MulAssign for Complex<T> {
    /// Overloading the operator ```*=```.
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.imaginary * other.real + self.real * other.imaginary,
        }
    }
}

/// Implementation of ```Neg```.
impl<T: Neg<Output = T>> Neg for Complex<T> {
    type Output = Self;
    /// Overloading the operator ```-```.
    fn neg(self) -> Self {
        Self {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

/// Implementation of ```Zero``` defined in ```identities.rs```.
impl<T: Copy + Zero> Zero for Complex<T> {
    /// A function that returns an object corresponding to ```[0+0x]``` in ```R\[x\]/(x^2 + 1)```.
    fn zero() -> Self {
        Self {
            real: T::zero(),
            imaginary: T::zero(),
        }
    }
}

/// Implementation of ```Identity``` defined in ```identities.rs```.
impl<T: Copy + Zero + Identity> Identity for Complex<T> {
    /// A function that returns an object corresponding to ```[1+0x]``` in ```R\[x\]/(x^2 + 1)```.
    fn identity() -> Self {
        Self {
            real: T::identity(),
            imaginary: T::zero(),
        }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Eq + Zero + Identity>
    Complex<T>
{
    /// Culculate exponentiation by repeated squaring.
    pub fn modpow(&self, n: u64) -> Self {
        let mut res_r = T::identity();
        let mut res_i = T::zero();
        let mut a = self.real;
        let mut b = self.imaginary;
        let mut m = n;
        loop {
            if m == 0 {
                break;
            }
            if m % 2 == 1 {
                let tmp_r = res_r;
                let tmp_i = res_i;
                res_r = tmp_r * a - tmp_i * b;
                res_i = tmp_r * b + a * tmp_i;
            }
            let tmp_a = a;
            let tmp_b = b;
            a = tmp_a * tmp_a - tmp_b * tmp_b;
            b = tmp_a * tmp_b + tmp_b * tmp_a;
            m = m / 2;
        }
        Self {
            real: res_r,
            imaginary: res_i,
        }
    }
}

/// Implementation of ```Characteristic``` defined in ```characteristic.rs```.
impl<T: Characteristic> Characteristic for Complex<T> {
    fn characteristic() -> u64 {
        T::characteristic()
    }
}

impl<
        T: Characteristic
            + Copy
            + Add<Output = T>
            + Mul<Output = T>
            + Sub<Output = T>
            + Eq
            + Zero
            + Identity,
    > Inverse for Complex<T>
{
    /// A function that returns an object corresponding to ```x^(-1)``` in ```R\[x\]/(x^2 + 1)```.
    /// This function works well if ```R``` is ```F_p``` where ```p``` is prime and ```-1``` is quadratic non-residue in ```F_p```.
    fn inverse(self) -> Option<Complex<T>> {
        if self.real == T::zero() && self.imaginary == T::zero() {
            None
        } else {
            Some(self.modpow(T::characteristic() * T::characteristic() - 2))
        }
    }
}
