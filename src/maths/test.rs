#[cfg(test)]
mod test {
    use crate::maths;

    #[test]
    fn test_sqrt() {
        assert_eq!(maths::sqrt(-10.0), None);
        assert_eq!(maths::sqrt(-0.123), None);
        assert_eq!(maths::sqrt(0.0), Some(0.0));
        assert_eq!(maths::sqrt(4.0), Some(2.0));
        assert_eq!(maths::sqrt(25.0), Some(5.0));
        assert_eq!(maths::sqrt(10.0), Some(3.162277660168379));
        assert_eq!(maths::sqrt(8.789), Some(2.964624765463582));
        assert_eq!(maths::sqrt(1.0), Some(1.0));
        assert_eq!(maths::sqrt(0.5), Some(0.7071067811865475));
        assert_eq!(maths::sqrt(0.0625), Some(0.25));
        assert_eq!(maths::sqrt(0.1), Some(0.31622776601683794));
        assert_eq!(maths::sqrt(70484.5874), Some(265.4893357556947));
    }

    #[test]
    fn test_abs() {
        assert_eq!(maths::abs(1.0), 1.0);
        assert_eq!(maths::abs(-1.0), 1.0);
        assert_eq!(maths::abs(-134.7), 134.7);
        assert_eq!(maths::abs(0.34), 0.34);
        assert_eq!(maths::abs(0.0), 0.0);
    }
}
