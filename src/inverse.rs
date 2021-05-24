/// Inverse element. If ```x``` has its inverse, this function returns ```Some(x^(-1))```, otherwise ```None```.
pub trait Inverse
where
    Self: std::marker::Sized,
{
    fn inverse(self) -> Option<Self>;
}
