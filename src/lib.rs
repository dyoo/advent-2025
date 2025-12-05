pub fn count_digits(n: u64) -> u32 {
    if n == 0 { return 1; }
    n.ilog10() + 1
}


#[test]
fn test_count_digits() {
    assert_eq!(count_digits(0), 1);
    assert_eq!(count_digits(1), 1);
    assert_eq!(count_digits(9), 1);
    assert_eq!(count_digits(10), 2);
    assert_eq!(count_digits(11), 2);
    assert_eq!(count_digits(99), 2);
    assert_eq!(count_digits(100), 3);
}
