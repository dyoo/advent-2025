use std::io::stdin;

#[derive(Debug)]
struct IdRange {
    from: u64,
    to: u64,
}

fn read_input() -> Vec<IdRange> {
    stdin().lines()
        .filter_map(|s| s.ok())
        .flat_map(|s| s.split(",").map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|s: String| {
            let mut ids = s.split("-").map(|s| s.parse::<u64>().expect("parse error"));
            let from = ids.next().expect("missing from");
            let to = ids.next().expect("missing to");
            IdRange {from, to}
        }).collect()
}

fn main() {
    println!("{:?}", read_input())
}
