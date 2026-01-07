use std::cmp::Ord;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io::BufRead;
use std::io::stdin;

#[derive(Debug)]
struct Problem {
    indicator: Vec<bool>,
    schematics: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn parse_indicator(line: &str) -> Option<Vec<bool>> {
    let Some(indicator_start) = line.find('[') else {
        return None;
    };
    let Some(indicator_end) = line.find(']') else {
        return None;
    };
    Some(
        line.as_bytes()[indicator_start + 1..indicator_end]
            .iter()
            .map(|ch| *ch == b'#')
            .collect(),
    )
}

fn parse_comma_separated(s: &str) -> Vec<usize> {
    s[1..s.len() - 1]
        .split(',')
        .flat_map(|n| n.parse::<usize>().ok())
        .collect()
}

fn parse_line(line: &str) -> Option<Problem> {
    let chunks: Vec<&str> = line.split(" ").collect();
    let Some(indicator) = parse_indicator(chunks[0]) else {
        return None;
    };
    let schematics = chunks[1..chunks.len() - 1]
        .iter()
        .map(|&chunk| parse_comma_separated(chunk))
        .collect();
    let joltage = parse_comma_separated(chunks[chunks.len() - 1]);
    Some(Problem {
        indicator,
        schematics,
        joltage,
    })
}

fn push_schematic(v: &[bool], schematics: &[usize]) -> Vec<bool> {
    let mut result = Vec::from(v);
    for b in schematics {
        result[*b] = !result[*b];
    }
    result
}

fn find_fewest_presses_for_indicators(p: &Problem) -> usize {
    let mut seen: HashSet<Vec<bool>> = HashSet::new();
    let mut count = 0;
    let mut fringe: HashSet<Vec<bool>> = [vec![false; p.indicator.len()]].into_iter().collect();

    // Breadth first traveral.
    while !fringe.is_empty() {
        seen.extend(fringe.clone());
        let new_buttons: HashSet<Vec<bool>> = fringe
            .into_iter()
            .flat_map(|buttons| {
                p.schematics
                    .iter()
                    .map(move |schematic| push_schematic(&buttons, schematic))
            })
            .collect();
        count += 1;

        if new_buttons.iter().any(|buttons| *buttons == p.indicator) {
            return count;
        }

        fringe = new_buttons
            .into_iter()
            .filter(|button| !seen.contains(button))
            .collect();
    }

    panic!("Unexpected escape");
}

#[test]
fn test_find_fewest_presses_for_indicators_1() {
    let problem = parse_line("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").expect("problem");
    assert_eq!(find_fewest_presses_for_indicators(&problem), 2);
}

#[test]
fn test_find_fewest_presses_for_indicators_2() {
    let problem = parse_line("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
        .expect("problem");
    assert_eq!(find_fewest_presses_for_indicators(&problem), 3);
}

#[test]
fn test_find_fewest_presses_for_indicators_3() {
    let problem = parse_line("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
        .expect("problem");
    assert_eq!(find_fewest_presses_for_indicators(&problem), 2);
}

fn part_1(problems: &[Problem]) -> usize {
    problems
        .into_iter()
        .map(find_fewest_presses_for_indicators)
        .sum()
}


fn part_2(problems: &[Problem]) -> usize {
    problems
        .into_iter()
        .map(find_fewest_presses_for_joltage)
        .sum()
}


fn increment_joltage(joltage: &[usize], schematics: &[usize]) -> Vec<usize> {
    let mut result = Vec::from(joltage);
    for b in schematics {
        result[*b] += 1;
    }
    result
}

#[test]
fn test_increment_joltage() {
    assert_eq!(increment_joltage(&[0, 1, 2, 3], &[1, 3]), vec![0, 2, 2, 4]);
}

/// Check that all the values in x are <= those in y.
fn joltage_le(x: &[usize], y: &[usize]) -> bool {
    x.iter().zip(y).all(|(x, y)| x <= y)
}

// We approximate the "distance" between x and y as the max componentwise distance.
fn joltage_distance_approximation(x: &[usize], y: &[usize]) -> usize {
    x.iter()
        .zip(y)
        .map(|(x, y)| x.abs_diff(*y))
        .max()
        .expect("non-empty joltage")
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct Element {
    estimate_to_goal: usize,
    clicks: usize,
    joltage: Vec<usize>,
}

fn find_fewest_presses_for_joltage(p: &Problem) -> usize {
    // We'll do an A* approach, using joltage_distance_approximation
    // as our heuristic guiding us through the search space.

    let initial_joltage = vec![0usize; p.indicator.len()];
    let goal_joltage = p.joltage.clone();

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(Element {
        estimate_to_goal: joltage_distance_approximation(&initial_joltage, &goal_joltage),
        clicks: 0,
        joltage: initial_joltage,
    }));

    while !priority_queue.is_empty() {
        let Some(Reverse(element)) = priority_queue.pop() else {
            panic!("impossible");
        };
        if element.joltage == goal_joltage {
            return element.clicks;
        }

        for schematic in &p.schematics {
            let child_joltage = increment_joltage(&element.joltage, schematic);
            if !joltage_le(&child_joltage, &goal_joltage) {
                continue;
            }

            priority_queue.push(Reverse(Element {
                estimate_to_goal: element.clicks
                    + joltage_distance_approximation(&child_joltage, &goal_joltage),
                clicks: element.clicks + 1,
                joltage: child_joltage,
            }));
        }
    }

    panic!("Unable to find fewest presses for joltage!");
}

#[test]
fn test_find_fewest_presses_for_joltage_1() {
    let problem = parse_line("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").expect("problem");
    assert_eq!(find_fewest_presses_for_joltage(&problem), 10);
}

#[test]
fn test_find_fewest_presses_for_joltage_2() {
    let problem = parse_line("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
        .expect("problem");
    assert_eq!(find_fewest_presses_for_joltage(&problem), 12);
}

#[test]
fn test_find_fewest_presses_for_joltage_3() {
    let problem = parse_line("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
        .expect("problem");
    println!("{:?}", problem);
    assert_eq!(find_fewest_presses_for_joltage(&problem), 11);
}

// Notes: we'll want to look into
// https://docs.rs/good_lp/latest/good_lp/index.html to do a linear
// programming solver approach, following the sketch of
// https://medium.com/@sergey.chelak/my-point-on-advent-of-code-2025-days-10-12-2d9232940b04.

fn main() {
    let problems: Vec<Problem> = stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .flat_map(|line| parse_line(&line))
        .collect();
    println!("Part 1: {:?}", part_1(&problems));
    println!("Part 2: {:?}", part_2(&problems));
}
