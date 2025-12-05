use std::io::{stdin, BufRead};
use std::ops::{RangeInclusive};

use aoc2025::count_digits;

fn read_input(buf_read: impl BufRead) -> Vec<RangeInclusive<u64>> {
    buf_read.lines()
        .filter_map(|s| s.ok())
        .flat_map(|s| s.split(",").map(|s| s.to_string()).collect::<Vec<String>>())
        .filter(|s| !s.is_empty())
        .map(|s: String| {
            let mut ids = s.split("-").map(|s| s.parse::<u64>().expect("parse error"));
            let from = ids.next().expect("missing from");
            let to = ids.next().expect("missing to");
            from..=to
        }).collect()
}

fn is_bad(n: u64) -> bool {
    let digits = crate::count_digits(n);
    if digits < 2 || digits % 2 != 0 {
        return false;
    }
    
    let (left_half, right_half) = {
        (n / 10u64.pow(digits / 2),
            n % 10u64.pow(digits / 2))
    };
    left_half == right_half
}

#[test]
fn test_is_bad() {
    assert!(is_bad(1312) == false);
    assert!(is_bad(1313));

    assert!(is_bad(345345));
    assert!(is_bad(345678) == false);
}

fn part1(input: &[RangeInclusive<u64>]) -> u64 {
    input.iter().flat_map(|r| r.clone().filter(|&v|is_bad(v))).sum()
}

#[test]
fn test_part1() {
    let input = read_input("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124".as_bytes());
    assert_eq!(part1(&input), 1227775554);
}

fn main() {
    let input = read_input(stdin().lock());
    println!("Part 1: {:?}", part1(&input));
}
