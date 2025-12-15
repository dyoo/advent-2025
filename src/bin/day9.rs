use std::io;
use std::io::BufRead;

type Point = (u64, u64);

fn parse(buf: impl BufRead) -> Vec<Point> {
    buf.lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            let mut chunks = l.split(',');
            let x: u64 = chunks.next().expect("first").parse().expect("number");
            let y: u64 = chunks.next().expect("second").parse().expect("number");
            (x, y)
        })
        .collect()
}

fn pairs(points: &[Point]) -> impl Iterator<Item = (Point, Point)> {
    (0..points.len()).flat_map(move |x| (x + 1..points.len()).map(move |y| (points[x], points[y])))
}

fn area(p1: Point, p2: Point) -> u64 {
    (1 + p1.0.abs_diff(p2.0)) * (1 + p1.1.abs_diff(p2.1))
}

#[test]
fn test_area() {
    assert_eq!(area((2, 5), (9, 7)), 24);
}

fn part_1(points: &[Point]) -> Option<u64> {
    pairs(points).map(|pair| area(pair.0, pair.1)).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part_1() {
        let points = parse(DATA.as_bytes());
        assert_eq!(part_1(&points), Some(50));
    }
}

fn main() {
    let points = parse(io::stdin().lock());
    println!("Part 1: {:?}", part_1(&points));
}
