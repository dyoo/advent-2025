use std::io::{BufRead, stdin};

#[derive(Debug)]
struct Grid {
    lines: Vec<Vec<u8>>,
}

impl Grid {
    fn width(&self) -> usize {
        self.lines[0].len()
    }

    fn height(&self) -> usize {
        self.lines.len()
    }

    fn at(&self, x: usize, y: usize) -> u8 {
        self.lines[y][x]
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

fn read_input(buf_read: impl BufRead) -> Grid {
    let lines = buf_read
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.as_bytes().into_iter().copied().collect())
        .collect();
    Grid { lines }
}

fn main() {
    let input = read_input(stdin().lock());
    println!("{} {}", input.width(), input.height());
    println!("{}", input);
}
