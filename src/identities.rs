/// Zero element. i.e. ```0``` in the ring of integers.
pub trait Zero {
    fn zero() -> Self;
}

/// Identity element. i.e. ```1``` in the ring of integers.
pub trait Identity {
    fn identity() -> Self;
}
