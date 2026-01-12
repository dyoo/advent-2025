use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, default_solver, variable,
};
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
    let indicator_start = line.find('[')?;
    let indicator_end = line.find(']')?;
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
    let indicator = parse_indicator(chunks[0])?;
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
        .iter()
        .map(find_fewest_presses_for_indicators)
        .sum()
}

fn part_2(problems: &[Problem]) -> usize {
    problems.iter().map(find_fewest_presses_for_joltage).sum()
}

fn find_fewest_presses_for_joltage(p: &Problem) -> usize {
    let mut problem_variables = ProblemVariables::new();
    let button_presses: Vec<Variable> =
        problem_variables.add_all(vec![variable().min(0).integer(); p.schematics.len()]);
    let goal = button_presses.iter().sum::<Expression>();
    let problem = problem_variables.minimise(&goal).using(default_solver);

    let constraints = p
        .joltage
        .iter()
        .enumerate()
        .map(|(index, &goal)| {
            button_presses
                .iter()
                .zip(p.schematics.iter())
                .filter_map(|(button_press, schematic)| {
                    if schematic.contains(&index) {
                        Some(button_press)
                    } else {
                        None
                    }
                })
                .sum::<Expression>()
                .eq(goal as u32)
        })
        .collect::<Vec<_>>();

    let solution = problem
        .with_all(constraints)
        .solve()
        .expect("solver failed");
    solution.eval(goal) as usize
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
        .map_while(|line| line.ok())
        .flat_map(|line| parse_line(&line))
        .collect();
    println!("Part 1: {:?}", part_1(&problems));
    println!("Part 2: {:?}", part_2(&problems));
}
