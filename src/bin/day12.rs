use std::io;
use std::io::BufRead;
use std::io::stdin;

use aoc2025::Grid;

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    quantity: Vec<usize>,
}

#[derive(Debug)]
struct Problem {
    shapes: Vec<Grid>,
    regions: Vec<Region>,
}

fn parse_shape(mut buf_read: impl BufRead) -> Option<Grid> {
    // Skip the first line
    buf_read.read_line(&mut String::new()).expect("index");
    Some(Grid::new(buf_read))
}

fn parse_region(s: &str) -> Region {
    let chunks: Vec<&str> = s.split_whitespace().collect();
    let size_chunk = chunks[0];
    let mut width_height = size_chunk[0..size_chunk.len() - 1].split('x');
    let width = width_height
        .next()
        .expect("width")
        .parse()
        .expect("width parse");
    let height = width_height
        .next()
        .expect("height")
        .parse()
        .expect("height parse");
    let quantity = chunks[1..]
        .iter()
        .map(|chunk| chunk.parse::<usize>().expect("quantity"))
        .collect();

    Region {
        width,
        height,
        quantity,
    }
}

fn parse_problem(buf: impl BufRead) -> Option<Problem> {
    let content = io::read_to_string(buf).ok()?;
    let chunks: Vec<String> = content.split("\n\n").map(|s| s.to_string()).collect();
    let shapes: Vec<Grid> = chunks[0..chunks.len() - 1]
        .iter()
        .map(|chunk: &String| parse_shape(chunk.as_bytes()))
        .collect::<Option<_>>()?;
    let regions: Vec<Region> = chunks[chunks.len() - 1].lines().map(parse_region).collect();

    Some(Problem { shapes, regions })
}

fn main() {
    let problem = parse_problem(stdin().lock()).expect("problem");
}
