pub fn count_digits(n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
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

// Explodes number into its digits
pub fn get_digits(mut n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }

    let mut result = Vec::new();
    while n != 0 {
        result.push((n % 10) as u8);
        n /= 10;
    }
    result.reverse();
    result
}

#[test]
fn test_get_digits() {
    assert_eq!(get_digits(31337), vec![3, 1, 3, 3, 7]);
}

pub fn implode_digits(digits: &[u8]) -> u64 {
    let mut result = 0;
    for d in digits {
        result = result * 10 + (*d as u64);
    }
    result
}

#[test]
fn test_implode_digits() {
    assert_eq!(implode_digits(&[1, 2, 3, 4]), 1234);
}
