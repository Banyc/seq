mod seq32;

pub use seq32::Seq32;

pub trait Seq: PartialOrd + Ord {
    fn add_usize(&self, n: usize) -> Self;
    fn sub_usize(&self, n: usize) -> Self;
    fn dist(a: &Self, b: &Self) -> usize;
    fn zero() -> Self;
}
