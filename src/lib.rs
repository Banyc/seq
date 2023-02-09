use std::{
    cmp::Ordering,
    ops::{Add, Div, Sub},
};

use num_traits::{Bounded, One, WrappingAdd, WrappingSub, Zero};

mod seq32;

pub use seq32::*;

pub type Seq8 = Seq<u8>;
pub type Seq16 = Seq<u16>;
pub type Seq64 = Seq<u64>;

pub trait SeqTrait:
    Copy
    + Eq
    + Ord
    + Bounded
    + Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + WrappingAdd
    + WrappingSub
{
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Seq<T> {
    v: T,
}

impl<T> Seq<T> {
    pub fn new(v: T) -> Self {
        Self { v }
    }

    pub fn value(&self) -> &T {
        &self.v
    }
}

impl<T> Seq<T>
where
    T: WrappingAdd,
{
    pub fn add(&self, n: T) -> Self {
        let v = self.v.wrapping_add(&n);
        Self { v }
    }
}

impl<T> Seq<T>
where
    T: WrappingSub,
{
    pub fn sub(&self, n: T) -> Self {
        let v = self.v.wrapping_sub(&n);
        Self { v }
    }
}

impl<T> Seq<T>
where
    T: Ord + WrappingSub + Zero + Copy + Bounded + One + Div<Output = T>,
{
    pub fn dist(a: &Self, b: &Self) -> T {
        match Self::cmp(&a, &b) {
            Ordering::Less => b.v.wrapping_sub(&a.v),
            Ordering::Greater => a.v.wrapping_sub(&b.v),
            Ordering::Equal => T::zero(),
        }
    }
}

impl<T> Seq<T>
where
    T: Zero,
{
    pub fn zero() -> Self {
        Self { v: T::zero() }
    }
}

impl<T> PartialOrd for Seq<T>
where
    T: Eq
        + Sub<Output = T>
        + PartialOrd
        + Ord
        + Copy
        + Bounded
        + One
        + Add<Output = T>
        + Div<Output = T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Seq<T>
where
    T: Eq
        + Sub<Output = T>
        + PartialOrd
        + Ord
        + Copy
        + Bounded
        + One
        + Add<Output = T>
        + Div<Output = T>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = match self.v.partial_cmp(&other.v).unwrap() {
            Ordering::Less => {
                let diff = other.v - self.v;
                match diff <= T::max_value() / (T::one() + T::one()) {
                    true => Ordering::Less,
                    false => Ordering::Greater,
                }
            }
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => {
                let diff = self.v - other.v;
                match diff <= T::max_value() / (T::one() + T::one()) {
                    true => Ordering::Greater,
                    false => Ordering::Less,
                }
            }
        };
        ord
    }
}
