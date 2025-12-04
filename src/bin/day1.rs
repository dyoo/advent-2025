use std::io::{stdin};

fn read_input() -> Vec<i32> {
    stdin().lines().filter_map(|s| s.ok()).map(|s| {
        (if s.starts_with("L") {
            -1
        } else {
            1
        }) * &s[1..].parse::<i32>().expect("parse error")
    }).collect()
}

fn main() {
    println!("Day 1");
    let mut count_zeros = 0;
    let mut dial = 50;
    for direction in read_input() {
        dial = (dial + direction).rem_euclid(100);
        if dial == 0 {
            count_zeros += 1;
        }
    }
    println!("count_zeros: {}", count_zeros);
}
