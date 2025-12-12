use aoc2025::{Grid, Pos};
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn parse(s: impl BufRead) -> Grid {
    Grid::new(s)
}

trait Beam {
    // Given a position, compute the next position.
    fn beam_next(&self, pos: Pos) -> Vec<Pos>;
    fn is_split_below(&self, pos: Pos) -> bool;
    fn start(&self) -> Option<Pos>;
}

impl Beam for Grid {
    fn beam_next(&self, pos: Pos) -> Vec<Pos> {
        let Some(below) = self.pos_plus(pos, (0, 1)) else {
            return vec![];
        };
        if self.at(below) == b'^' {
            self.pos_plus(below, (-1, 0))
                .into_iter()
                .chain(self.pos_plus(below, (1, 0)))
                .collect()
        } else {
            vec![below]
        }
    }

    fn is_split_below(&self, pos: Pos) -> bool {
        let Some(below) = self.pos_plus(pos, (0, 1)) else {
            return false;
        };

        self.at(below) == b'^'
    }

    fn start(&self) -> Option<Pos> {
        for i in 0..self.width() {
            if self.at((i, 0)) == b'S' {
                return Some((i, 0));
            }
        }
        None
    }
}

// Compute the number of splits.
fn part_1(grid: &Grid) -> usize {
    let mut count_splits = 0;
    let Some(start) = grid.start() else {
        return 0;
    };

    let mut fringe = HashSet::from([start]);
    loop {
        count_splits += fringe
            .iter()
            .filter(|&&pos| grid.is_split_below(pos))
            .count();
        fringe = fringe
            .into_iter()
            .flat_map(|pos| grid.beam_next(pos))
            .collect();
        if fringe.is_empty() {
            break;
        }
    }

    count_splits
}

#[cfg(test)]
mod tests {
    use super::*;
    const data: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_beam_next() {
        let grid = parse(data.as_bytes());
        assert_eq!(grid.beam_next((7, 0)), vec![(7, 1)]);
        assert_eq!(grid.beam_next((7, 1)), vec![(6, 2), (8, 2)]);
    }

    #[test]
    fn test_start() {
        let grid = parse(data.as_bytes());
        assert_eq!(grid.start(), Some((7, 0)));
    }

    #[test]
    fn test_part_1() {
        let grid = parse(data.as_bytes());
        assert_eq!(part_1(&grid), 21);
    }
}

fn main() {
    let grid = parse(io::stdin().lock());
    println!("Part 1: {:?}", part_1(&grid));
}
