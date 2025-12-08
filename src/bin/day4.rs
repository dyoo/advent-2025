use aoc2025::Grid;
use std::io::stdin;

fn part_1(grid: &Grid) -> usize {
    let paper_positions = grid.positions().filter(|pos| grid.at(*pos) == b'@');
    let fewer_than_four = paper_positions
        .filter(|pos| grid.neighbors(*pos).filter(|p| grid.at(*p) == b'@').count() < 4);

    fewer_than_four.count()
}

#[test]
fn test_part1() {
    let data = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
        .as_bytes();
    let input = Grid::new(data);
    assert_eq!(part_1(&input), 13);
}

fn main() {
    let input = Grid::new(stdin().lock());
    println!("Part 1: {}", part_1(&input));
}
