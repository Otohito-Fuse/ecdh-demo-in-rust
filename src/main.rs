const P: u64 = 863; // P must be 'prime' and '3 mod 4' and '>= 7'.
                    // Default value is 863 = 2^5 * 3^3 - 1.

pub mod characteristic;
pub mod complexification;
pub mod identities;
pub mod inverse;
pub mod modint;
pub mod polynomial;
pub mod rational_point;

use crate::complexification::Complex;
use crate::identities::{Identity, Zero};
use crate::modint::ModInt;
use crate::polynomial::Polynomial;
use crate::rational_point::RationalPoint;

use rand::seq::SliceRandom;
use rand::thread_rng;

/// Primality test
fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    if n == 0 || n == 1 {
        return false;
    }
    for i in 0..n {
        if n != 3 + 2 * i && n % (3 + 2 * i) == 0 {
            return false;
        }
        if (3 + 2 * i) * (3 + 2 * i) >= n {
            break;
        }
    }
    true
}

fn main() {
    if !is_prime(P) {
        println!("p = {} is not prime.", P);
        return;
    }

    if P < 7 || P % 4 == 1 {
        println!("Please set p as '3 mod 4'-type prime >= 7.");
        return;
    }

    println!("\nDemonstration of ECDH (Elliptic curve Diffie–Hellman key exchange).\n");

    let a;
    let b;
    let mut rng = thread_rng();
    let v: Vec<u64> = (1..P).collect();
    loop {
        let &i = v.choose(&mut rng).unwrap();
        let &j = v.choose(&mut rng).unwrap();
        if ModInt::<P>::new(4) * ModInt::<P>::new(i).power(3)
            + ModInt::<P>::new(27) * ModInt::<P>::new(j).power(2)
            != ModInt::<P>::new(0)
        {
            a = i;
            b = j;
            break;
        }
    }

    println!(
        "We consider the elliptic curve\ny^2 = x^3 + {0}x + {1}\nover F_({2}^2) = F_{2}[x]/(x^2 + 1) = F_{2}(i).\n",
        a, b, P
    );

    let f_v: Vec<Complex<ModInt<P>>> = vec![
        Complex::<ModInt<P>>::new(ModInt::<P>::new(b), ModInt::<P>::new(0)),
        Complex::<ModInt<P>>::new(ModInt::<P>::new(a), ModInt::<P>::new(0)),
        Complex::<ModInt<P>>::zero(),
        Complex::<ModInt<P>>::identity(),
    ];
    let f: Polynomial<Complex<ModInt<P>>> = Polynomial::new(&f_v);

    /* use std::collections::HashSet;
    let mut q_r: HashSet<u64> = HashSet::new();
    for i in 1..P {
        q_r.insert((i * i) % P);
    }
    let q_r: Vec<u64> = q_r.into_iter().collect(); */

    let point;
    loop {
        let &i = v.choose(&mut rng).unwrap();
        let &j = v.choose(&mut rng).unwrap();
        let &k = v.choose(&mut rng).unwrap();
        let &l = v.choose(&mut rng).unwrap();
        let x = Complex::<ModInt<P>>::new(ModInt::<P>::new(i), ModInt::<P>::new(j));
        let y = Complex::<ModInt<P>>::new(ModInt::<P>::new(k), ModInt::<P>::new(l));
        if y * y == Polynomial::evaluate(&f, x) {
            point = RationalPoint::Point(x, y);
            break;
        }
    }

    println!("We start up with the rational point G = {}.\n", point);

    let mut point_tmp = point.clone();

    let mut flag = false;
    let max = std::cmp::max(P * P, 1000000);
    let mut ord = max;
    for i in 2..=max {
        point_tmp = point_tmp.add_rational_points(
            &point,
            Complex::<ModInt<P>>::new(ModInt::<P>::new(a), ModInt::<P>::zero()),
        );
        if point_tmp == RationalPoint::O {
            ord = i;
            flag = true;
            break;
        }
    }

    if flag {
        println!("The order of G is {}.\n", ord);
    } else {
        println!("The order of G is greater than p^2.\n");
    }

    /* point_tmp = point.clone();
    for i in 2..=ord {
        point_tmp = point_tmp.add_rational_points(
            &point,
            Complex::<ModInt<P>>::new(ModInt::<P>::new(a), ModInt::<P>::zero()),
        );
        println!("{:10}P = {}", i, point_tmp);
    } */

    if flag {
        assert_eq!(
            RationalPoint::O,
            point.multiply_rational_point(
                Complex::<ModInt<P>>::new(ModInt::<P>::new(a), ModInt::<P>::zero()),
                ord
            )
        );
    }

    let w: Vec<u64> = (1..ord).collect();

    let &d_a = w.choose(&mut rng).unwrap();

    let point_a = point.multiply_rational_point(
        Complex::<ModInt<P>>::new(ModInt::<P>::new(a), ModInt::<P>::zero()),
        d_a,
    );

    println!(
        "1a. Alice chooses d_a = {} randomly and computes Q_a = d_a G = {}.\n",
        d_a, point_a
    );

    let &d_b = w.choose(&mut rng).unwrap();

    let point_b = point.multiply_rational_point(
        Complex::<ModInt<P>>::new(ModInt::<P>::new(a), ModInt::<P>::zero()),
        d_b,
    );

    println!(
        "1b. Bob chooses d_b = {} randomly and computes Q_b = d_b G = {}.\n",
        d_b, point_b
    );

    println!("2. Alice sends Q_a to Bob while Bob sends Q_b to Alice.\n");

    let point_ba = point_b.multiply_rational_point(
        Complex::<ModInt<P>>::new(ModInt::<P>::new(a), ModInt::<P>::zero()),
        d_a,
    );

    let point_ab = point_a.multiply_rational_point(
        Complex::<ModInt<P>>::new(ModInt::<P>::new(a), ModInt::<P>::zero()),
        d_b,
    );

    assert_eq!(point_ab, point_ba);

    println!("3a. Alice computes d_a Q_b = {}.\n", point_ba);

    println!("3b. Bob computes d_b Q_a = {}.\n", point_ab);

    println!("They coincide and can be used as a shared key.\n")
}
