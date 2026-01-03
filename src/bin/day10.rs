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

fn main() {
    let problems: Vec<Problem> = stdin()
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .flat_map(|line| parse_line(&line))
        .collect();
    println!("{:?}", problems);
}
