use std::io::{BufRead, stdin};

type Bank = Vec<u8>;

fn read_input(buf_read: impl BufRead) -> Vec<Bank> {
    buf_read
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.as_bytes().into_iter().map(|b| b - b'0').collect())
        .collect()
}

fn main() {
    let input = read_input(stdin().lock());
    println!("{:?}", input);
}
