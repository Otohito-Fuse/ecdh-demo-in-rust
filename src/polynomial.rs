use crate::identities::{Identity, Zero};
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Polynomials (in 1 variable).
///
/// ```coefficients``` is an array of coefficients starting with 0-th coefficient.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Polynomial<T> {
    coefficients: Vec<T>,
    degree: usize,
}

impl<T> Polynomial<T> {
    /// Return the degree of a polynomial. In this function, set deg 0 = 0.
    pub fn deg(&self) -> usize {
        self.degree
    }

    /// Make constant from an element in the coefficient ring.
    pub fn new_constant(t: T) -> Self {
        Self {
            coefficients: vec![t],
            degree: 0,
        }
    }
}

impl<T: Zero + Eq> Polynomial<T> {
    /// Degree of a polynomial. If f(x) != 0, it returns ```Some(deg f)```, otherwise ```None```.
    pub fn strict_deg(&self) -> Option<usize> {
        if self.coefficients.len() == 1 && self.coefficients[0] == T::zero() {
            None
        } else {
            Some(self.degree)
        }
    }
}

impl<T: Zero + Eq + Copy> Polynomial<T> {
    /// Constructor.
    ///
    /// If the coefficient of highest degree is equal to zero, remove it recursively.
    /// To realize this, ```Zero``` must be implemented for ```T```.
    pub fn new(v: &Vec<T>) -> Self {
        let mut f: Vec<T> = Vec::new();
        if v.len() == 0 {
            f.push(T::zero());
            Self {
                coefficients: f,
                degree: 0,
            }
        } else {
            f.push(v[0]);
            for &t in &v[1..] {
                f.push(t.clone());
            }
            while f.len() > 1 {
                if let Some(&t) = f.last() {
                    if t == T::zero() {
                        f.pop();
                    } else {
                        break;
                    }
                }
            }
            let d = f.len() - 1;
            Self {
                coefficients: f,
                degree: d,
            }
        }
    }
}

impl<T: Zero + Identity + Mul<Output = T> + MulAssign + AddAssign + Copy + Eq> Polynomial<T> {
    /// The evaluate function.
    pub fn evaluate(f: &Self, t: T) -> T {
        let mut t_pow = T::identity();
        let mut ans = T::zero();
        for &c in &f.coefficients {
            ans += c * t_pow;
            t_pow *= t;
        }
        ans
    }
}

/// Implementation of ```Display```.
impl<T: fmt::Display + Zero + Identity + Eq> fmt::Display for Polynomial<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::new();
        let mut flag = false;
        if self.degree == 0 || self.coefficients[0] != T::zero() {
            s.push_str(&self.coefficients[0].to_string())
        } else {
            flag = true;
        }
        if self.degree > 0 && self.coefficients[1] != T::zero() {
            if !flag {
                s.push_str(&" + ");
            } else {
                flag = false;
            }
            if self.coefficients[1] != T::identity() {
                s.push_str(&self.coefficients[1].to_string());
            }
            s.push_str(&"x");
        }
        if self.degree > 1 {
            for i in 2..=self.degree {
                if self.coefficients[i] == T::zero() {
                    continue;
                }
                if !flag {
                    s.push_str(&" + ");
                } else {
                    flag = false;
                }
                if self.coefficients[i] != T::identity() {
                    s.push_str(&self.coefficients[i].to_string());
                }
                s.push_str(&"x^");
                s.push_str(&i.to_string());
            }
        }
        write!(f, "{}", s)
    }
}

impl<T: fmt::Display + Zero + Identity + Eq> Polynomial<T> {
    /// Display as a function of ```x```.
    pub fn print_f_of_x(&self, x: char) -> String {
        let mut s: String = String::new();
        let mut flag = false;
        if self.degree == 0 || self.coefficients[0] != T::zero() {
            s.push_str(&self.coefficients[0].to_string())
        } else {
            flag = true;
        }
        if self.degree > 0 && self.coefficients[1] != T::zero() {
            if !flag {
                s.push_str(&" + ");
            } else {
                flag = false;
            }
            if self.coefficients[1] != T::identity() {
                s.push_str(&self.coefficients[1].to_string());
            }
            s.push_str(&x.to_string());
        }
        if self.degree > 1 {
            for i in 2..=self.degree {
                if self.coefficients[i] == T::zero() {
                    continue;
                }
                if !flag {
                    s.push_str(&" + ");
                } else {
                    flag = false;
                }
                if self.coefficients[i] != T::identity() {
                    s.push_str(&self.coefficients[i].to_string());
                }
                s.push_str(&x.to_string());
                s.push_str(&"^");
                s.push_str(&i.to_string());
            }
        }
        s
    }
}

/// Implementation of ```Add```.
impl<T: Copy + Add<Output = T> + Zero + Eq> Add for Polynomial<T> {
    type Output = Self;
    /// Overloading the operator ```+```.
    fn add(self, rhs: Self) -> Self {
        let mut v: Vec<T> = Vec::new();
        v.push(self.coefficients[0] + rhs.coefficients[0]);
        if self.degree < rhs.degree {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] + rhs.coefficients[i]);
            }
            for i in (self.degree + 1)..=rhs.degree {
                v.push(rhs.coefficients[i]);
            }
        } else if self.degree > rhs.degree {
            for i in 1..=rhs.degree {
                v.push(self.coefficients[i] + rhs.coefficients[i]);
            }
            for i in (rhs.degree + 1)..=self.degree {
                v.push(self.coefficients[i]);
            }
        } else {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] + rhs.coefficients[i]);
            }
        }
        Polynomial::new(&v)
    }
}

/// Implementation of ```AddAssign```.
impl<T: Copy + Add<Output = T> + Zero + Eq> AddAssign for Polynomial<T> {
    /// Overloading the operator ```+=```.
    fn add_assign(&mut self, rhs: Self) {
        let mut v: Vec<T> = Vec::new();
        v.push(self.coefficients[0] + rhs.coefficients[0]);
        if self.degree < rhs.degree {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] + rhs.coefficients[i]);
            }
            for i in (self.degree + 1)..=rhs.degree {
                v.push(rhs.coefficients[i]);
            }
        } else if self.degree > rhs.degree {
            for i in 1..=rhs.degree {
                v.push(self.coefficients[i] + rhs.coefficients[i]);
            }
            for i in (rhs.degree + 1)..=self.degree {
                v.push(self.coefficients[i]);
            }
        } else {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] + rhs.coefficients[i]);
            }
        }
        *self = Polynomial::new(&v)
    }
}

/// Implementation of ```Sub```.
impl<T: Copy + Sub<Output = T> + Zero + Eq> Sub for Polynomial<T> {
    type Output = Self;
    /// Overloading the operator ```-```.
    fn sub(self, rhs: Self) -> Self {
        let mut v: Vec<T> = Vec::new();
        v.push(self.coefficients[0] - rhs.coefficients[0]);
        if self.degree < rhs.degree {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] - rhs.coefficients[i]);
            }
            for i in (self.degree + 1)..=rhs.degree {
                v.push(T::zero() - rhs.coefficients[i]);
            }
        } else if self.degree > rhs.degree {
            for i in 1..=rhs.degree {
                v.push(self.coefficients[i] - rhs.coefficients[i]);
            }
            for i in (rhs.degree + 1)..=self.degree {
                v.push(self.coefficients[i]);
            }
        } else {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] - rhs.coefficients[i]);
            }
        }
        Polynomial::new(&v)
    }
}

/// Implementation of ```SubAssign```.
impl<T: Copy + Sub<Output = T> + Zero + Eq> SubAssign for Polynomial<T> {
    /// Overloading the operator ```-=```.
    fn sub_assign(&mut self, rhs: Self) {
        let mut v: Vec<T> = Vec::new();
        v.push(self.coefficients[0] - rhs.coefficients[0]);
        if self.degree < rhs.degree {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] - rhs.coefficients[i]);
            }
            for i in (self.degree + 1)..=rhs.degree {
                v.push(T::zero() - rhs.coefficients[i]);
            }
        } else if self.degree > rhs.degree {
            for i in 1..=rhs.degree {
                v.push(self.coefficients[i] - rhs.coefficients[i]);
            }
            for i in (rhs.degree + 1)..=self.degree {
                v.push(self.coefficients[i]);
            }
        } else {
            for i in 1..=self.degree {
                v.push(self.coefficients[i] - rhs.coefficients[i]);
            }
        }
        *self = Polynomial::new(&v)
    }
}

/// Implementation of ```Mul```.
impl<T: Copy + Add<Output = T> + AddAssign<T> + Mul<Output = T> + Zero + Eq> Mul for Polynomial<T> {
    type Output = Self;
    /// Overloading the operator ```*```.
    fn mul(self, rhs: Self) -> Self {
        let mut v: Vec<T> = vec![T::zero(); self.degree * rhs.degree + 1];
        for i in 0..=(self.degree * rhs.degree) {
            for j in 0..=i {
                if i - j <= rhs.degree && j <= self.degree {
                    v[i] += self.coefficients[j] * rhs.coefficients[i - j];
                }
            }
        }
        Polynomial::new(&v)
    }
}

/// Implementation of ```MulAssign```.
impl<T: Copy + Add<Output = T> + AddAssign<T> + Mul<Output = T> + Zero + Eq> MulAssign
    for Polynomial<T>
{
    /// Overloading the operator ```*=```.
    fn mul_assign(&mut self, rhs: Self) {
        let mut v: Vec<T> = vec![T::zero(); self.degree * rhs.degree + 1];
        for i in 0..=(self.degree * rhs.degree) {
            for j in 0..=i {
                if i - j <= rhs.degree && j <= self.degree {
                    v[i] += self.coefficients[j] * rhs.coefficients[i - j];
                }
            }
        }
        *self = Polynomial::new(&v)
    }
}

/// Implementation of ```Neg```.
impl<T: Zero + Eq + Copy + Neg<Output = T>> Neg for Polynomial<T> {
    type Output = Self;
    /// Overloading the operator ```-```.
    fn neg(self) -> Self {
        let mut v: Vec<T> = Vec::new();
        for c in self.coefficients.clone() {
            v.push(-c);
        }
        Polynomial::new(&v)
    }
}

/// Implementation of ```Zero``` defined in ```identities.rs```.
impl<T: Zero + Copy> Zero for Polynomial<T> {
    /// A function that returns an object corresponding to ```0``` in the ring of polynomials.
    fn zero() -> Self {
        Self {
            coefficients: vec![T::zero(); 1],
            degree: 0,
        }
    }
}

/// Implementation of ```Identity``` defined in ```identities.rs```.
impl<T: Identity + Copy> Identity for Polynomial<T> {
    /// A function that returns an object corresponding to ```1``` in the ring of polynomials.
    fn identity() -> Self {
        Self {
            coefficients: vec![T::identity(); 1],
            degree: 0,
        }
    }
}
