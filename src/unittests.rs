#[cfg(test)]
pub mod u_tests {
    use crate::parse_bits;

    #[test]
    fn test_bitmask() {
        let mask = parse_bits("4:2").unwrap();
        assert_eq!(mask, 0b011100);
    }

}