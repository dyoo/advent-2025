use std::collections::HashMap;
use std::io::BufRead;
use std::io::stdin;

type Graph = HashMap<String, Vec<String>>;

fn parse_graph(buf: impl BufRead) -> Graph {
    let edges: HashMap<String, Vec<String>> = buf
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            let mut chunk = line.split_whitespace();
            let name = chunk.next()?;
            let name = &name[0..name.len() - 1];
            Some((name.to_string(), chunk.map(|str| str.to_string()).collect()))
        })
        .collect();
    edges
}

fn count_paths_to_out(graph: &Graph, path: &[&String]) -> usize {
    let at = path.last().copied().expect("empty path");

    if at == "out" {
        return 1;
    }

    let empty = vec![];
    let children = graph
        .get(at)
        .unwrap_or(&empty)
        .iter()
        .filter(|child| !path.contains(child));

    children
        .map(|child| {
            let mut new_path = path.to_vec();
            new_path.push(child);
            count_paths_to_out(graph, &new_path)
        })
        .sum()
}

#[test]
fn test_count_paths() {
    let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
        .as_bytes();
    let graph = parse_graph(input);
    assert_eq!(count_paths_to_out(&graph, &[&"you".to_string()]), 5);
}

fn part_2(graph: &Graph, path: &[&String], mut visited_fft: bool, mut visited_dac: bool) -> usize {
    let at = path.last().copied().expect("empty path");
    if at == "out" {
        return if visited_fft && visited_dac { 1 } else { 0 };
    } else if at == "fft" {
        visited_fft = true;
    } else if at == "dac" {
        visited_dac = true;
    }

    let empty = vec![];
    let children = graph
        .get(at)
        .unwrap_or(&empty)
        .iter()
        .filter(|child| !path.contains(child));

    children
        .map(|child| {
            let mut new_path = path.to_vec();
            new_path.push(child);
            part_2(graph, &new_path, visited_fft, visited_dac)
        })
        .sum()
}

#[test]
fn test_part_2() {
    let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
        .as_bytes();
    let graph = parse_graph(input);
    assert_eq!(part_2(&graph, &[&"svr".to_string()], false, false), 2);
}

fn main() {
    let graph = parse_graph(stdin().lock());
    println!(
        "Part 1: {}",
        count_paths_to_out(&graph, &[&"you".to_string()])
    );

    println!(
        "Part 2: {}",
        part_2(&graph, &[&"svr".to_string()], false, false)
    );
}
