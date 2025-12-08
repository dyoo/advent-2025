use std::io::{BufRead, stdin};
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Problem {
    fresh: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

fn parse(buf: impl BufRead) -> Problem {
    let mut lines = buf.lines().filter_map(|l| l.ok());
    let mut fresh = Vec::new();

    loop {
        let Some(line) = lines.next() else {
            break;
        };
        if line == "" {
            break;
        }
        let mut range_elements = line.split('-');
        let from = range_elements
            .next()
            .expect("first component")
            .parse::<u64>()
            .expect("parse first");
        let to = range_elements
            .next()
            .expect("second component")
            .parse::<u64>()
            .expect("parse second");
        fresh.push(from..=to);
    }
    let ingredients = lines.map(|l| l.parse::<u64>().expect("u64")).collect();

    Problem { fresh, ingredients }
}

fn main() {
    let problem = parse(stdin().lock());
    println!("{:?}", problem);
}
