use crate::Seq;

pub type Seq32 = Seq<u32>;

#[cfg(test)]
mod tests {
    use crate::seq32::Seq32;

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
        let b = a.add(1);
        assert_eq!(b.value(), &0);
    }

    #[test]
    fn add_no_wraparound() {
        let a = Seq32::new(0);
        let b = a.add(1);
        assert_eq!(b.value(), &1);
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
