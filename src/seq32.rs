use std::{cmp::Ordering, num::Wrapping};

use crate::Seq;

#[derive(Debug, PartialEq, Eq)]
pub struct Seq32 {
    v: u32,
}

impl Seq32 {
    pub fn new(v: u32) -> Self {
        Self { v }
    }

    pub fn as_u32(&self) -> u32 {
        self.v
    }
}

impl Seq for Seq32 {
    fn add_usize(&self, n: usize) -> Self {
        let s = Wrapping(self.v) + Wrapping(n as u32);
        Self { v: s.0 }
    }

    fn sub_usize(&self, n: usize) -> Self {
        let s = Wrapping(self.v) - Wrapping(n as u32);
        Self { v: s.0 }
    }

    fn dist(a: &Self, b: &Self) -> usize {
        let a_v = Wrapping(a.v);
        let b_v = Wrapping(b.v);
        match a.cmp(&b) {
            Ordering::Less => (b_v - a_v).0 as usize,
            Ordering::Greater => (a_v - b_v).0 as usize,
            Ordering::Equal => 0,
        }
    }

    fn zero() -> Self {
        Self { v: 0 }
    }
}

impl PartialOrd for Seq32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Seq32 {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = match self.v.partial_cmp(&other.v).unwrap() {
            Ordering::Less => {
                let diff = other.v - self.v;
                match diff <= u32::MAX / 2 {
                    true => Ordering::Less,
                    false => Ordering::Greater,
                }
            }
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => {
                let diff = self.v - other.v;
                match diff <= u32::MAX / 2 {
                    true => Ordering::Greater,
                    false => Ordering::Less,
                }
            }
        };
        ord
    }
}

#[cfg(test)]
mod tests {
    use crate::Seq;

    use super::Seq32;

    #[test]
    fn cmp_wraparound() {
        let a = Seq32::new(u32::MAX);
        let b = Seq32::new(u32::MIN);
        assert!(a < b);
    }

    #[test]
    fn cmp_no_wraparound() {
        let a = Seq32::new(0);
        let b = Seq32::new(1);
        assert!(a < b);
    }

    #[test]
    fn cmp_far() {
        let a = Seq32::new(0);
        let b = Seq32::new(i32::MAX as u32);
        let c = Seq32::new(i32::MAX as u32 + 1);
        assert!(a < b);
        assert!(c < a);
    }

    #[test]
    fn add_wraparound() {
        let a = Seq32::new(u32::MAX);
        let b = a.add_usize(1);
        assert_eq!(b.as_u32(), 0);
    }

    #[test]
    fn add_no_wraparound() {
        let a = Seq32::new(0);
        let b = a.add_usize(1);
        assert_eq!(b.as_u32(), 1);
    }

    #[test]
    fn sub_wraparound() {
        let a = Seq32::new(0);
        let b = Seq32::new(u32::MAX);
        assert_eq!(Seq32::dist(&a, &b), 1);
    }

    #[test]
    fn sub_zero() {
        let a = Seq32::new(1);
        let b = Seq32::new(1);
        assert_eq!(Seq32::dist(&a, &b), 0);
    }

    #[test]
    fn sub_no_wraparound() {
        let a = Seq32::new(3);
        let b = Seq32::new(1);
        assert_eq!(Seq32::dist(&a, &b), 2);
    }
}
