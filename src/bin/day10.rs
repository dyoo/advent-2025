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

fn find_fewest_presses(p: &Problem) -> usize {
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
fn test_find_fewest_presses_1() {
    let problem = parse_line("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").expect("problem");
    assert_eq!(find_fewest_presses(&problem), 2);
}

#[test]
fn test_find_fewest_presses_2() {
    let problem = parse_line("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
        .expect("problem");
    println!("{:?}", problem);
    assert_eq!(find_fewest_presses(&problem), 3);
}

#[test]
fn test_find_fewest_presses_3() {
    let problem = parse_line("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
        .expect("problem");
    println!("{:?}", problem);
    assert_eq!(find_fewest_presses(&problem), 2);
}

fn part_1(problems: &[Problem]) -> usize {
    problems.into_iter().map(find_fewest_presses).sum()
}

fn main() {
    let problems: Vec<Problem> = stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .flat_map(|line| parse_line(&line))
        .collect();
    println!("Part 1: {:?}", part_1(&problems));
}
