use std::collections::HashMap;
use std::io::BufRead;
use std::io::stdin;

#[derive(Debug)]
pub struct Graph {
    pub edges: HashMap<String, Vec<String>>,
}

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
    Graph { edges }
}

fn main() {
    let graph = parse_graph(stdin().lock());
    println!("{:?}", graph);
}
