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
    println!("{:?}", read_input());
    //let mut dial = 50;
}
