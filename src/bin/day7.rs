use aoc2025::{Grid, Pos};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;
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
    fn count_paths_from(&self, pos: Pos, cache: &mut HashMap<Pos, usize>) -> usize;
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

    fn count_paths_from(&self, pos: Pos, cache: &mut HashMap<Pos, usize>) -> usize {
        if let Entry::Occupied(count) = cache.entry(pos) {
            return *count.get();
        }

        let result = {
            if let Some(below) = self.pos_plus(pos, (0, 1)) {
                if self.is_split_below(pos) {
                    self.pos_plus(below, (-1, 0))
                        .map(|below_left| self.count_paths_from(below_left, cache))
                        .unwrap_or(0)
                        + self
                            .pos_plus(below, (1, 0))
                            .map(|below_right| self.count_paths_from(below_right, cache))
                            .unwrap_or(0)
                } else {
                    self.count_paths_from(below, cache)
                }
            } else {
                1
            }
        };

        cache.insert(pos, result);
        result
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

fn part_2(grid: &Grid) -> usize {
    let Some(start) = grid.start() else {
        return 0;
    };
    grid.count_paths_from(start, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = ".......S.......
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
        let grid = parse(DATA.as_bytes());
        assert_eq!(grid.beam_next((7, 0)), vec![(7, 1)]);
        assert_eq!(grid.beam_next((7, 1)), vec![(6, 2), (8, 2)]);
    }

    #[test]
    fn test_start() {
        let grid = parse(DATA.as_bytes());
        assert_eq!(grid.start(), Some((7, 0)));
    }

    #[test]
    fn test_part_1() {
        let grid = parse(DATA.as_bytes());
        assert_eq!(part_1(&grid), 21);
    }

    #[test]
    fn test_part_2() {
        let grid = parse(DATA.as_bytes());
        assert_eq!(part_2(&grid), 40);
    }
}

fn main() {
    let grid = parse(io::stdin().lock());
    println!("Part 1: {:?}", part_1(&grid));
    println!("Part 2: {:?}", part_2(&grid));
}
