#[cfg(test)]
pub mod u_tests {
    use crate::parse_bits;

    #[test]
    fn test_bitmask() {
        let (mask, lo) = parse_bits("4:2").unwrap();
        assert_eq!(mask, 0b011100);
        assert_eq!(lo, 2);
        
        let (mask, lo) = parse_bits("3:3").unwrap() ;
        assert_eq!(mask, 0b01000) ;
        assert_eq!(lo, 3);
    }
    
}