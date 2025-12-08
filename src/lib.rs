use std::io::BufRead;
use std::ops::RangeInclusive;

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

#[derive(Debug)]
pub struct Grid {
    lines: Vec<Vec<u8>>,
}

const NEIGHBOR_DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Grid {
    pub fn new(buf_read: impl BufRead) -> Self {
        let lines = buf_read
            .lines()
            .filter_map(|l| l.ok())
            .map(|l| l.as_bytes().into_iter().copied().collect())
            .collect();
        Self { lines }
    }

    pub fn width(&self) -> usize {
        self.lines[0].len()
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn at(&self, (x, y): (usize, usize)) -> u8 {
        self.lines[y][x]
    }

    pub fn set(&mut self, (x, y): (usize, usize), val: u8) {
        self.lines[y][x] = val;
    }

    pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height()).flat_map(|y| (0..self.width()).map(move |x| (x, y)))
    }

    pub fn pos_plus(&self, p1: (usize, usize), delta: (isize, isize)) -> Option<(usize, usize)> {
        let x = p1.0.checked_add_signed(delta.0)?;
        if x >= self.width() {
            return None;
        }

        let y = p1.1.checked_add_signed(delta.1)?;

        if y >= self.height() {
            return None;
        }

        Some((x, y))
    }

    pub fn neighbors(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        NEIGHBOR_DELTAS
            .into_iter()
            .filter_map(move |delta| self.pos_plus(pos, delta))
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for line in &self.lines {
            write!(f, "{}\n", std::str::from_utf8(&line).expect("utf8"))?;
        }
        Ok(())
    }
}

pub trait Overlaps {
    fn overlaps(&self, other: &Self) -> bool;
}

impl Overlaps for RangeInclusive<u64> {
    fn overlaps(&self, other: &Self) -> bool {
        self.end() >= other.start() && self.start() <= other.end()
    }
}

#[test]
fn test_overlaps() {
    assert!((1..=1).overlaps(&(1..=100)));
    assert!((1..=1).overlaps(&(1..=1)));
    assert!((1..=1).overlaps(&(2..=2)) == false);
}
