use std::io::BufRead;

#[derive(Debug, PartialEq)]
enum Op {
    Mul,
    Add,
}

impl Op {
    fn zero(&self) -> u64 {
        match self {
            Op::Mul => 1,
            Op::Add => 0,
        }
    }

    fn f(&self, x: u64, y: u64) -> u64 {
        match self {
            Op::Mul => x * y,
            Op::Add => x + y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Problem {
    columns: Vec<Vec<Option<u64>>>,
    ops: Vec<Op>,
}

fn parse(buf: impl BufRead) -> Problem {
    let lines: Vec<Vec<u8>> = buf
        .lines()
        .filter_map(|l| l.ok().map(|s| s.to_string().into_bytes()))
        .collect();

    let number_lines = &lines[0..lines.len() - 1];
    let operator_line = &lines[lines.len() - 1];

    let mut indexes: Option<(usize, usize)> = None;
    let mut operator = Op::Mul;

    let mut columns = Vec::new();
    let mut ops = Vec::new();

    let extract_column = |start, end| {
        number_lines
            .iter()
            .map(move |l| std::str::from_utf8(&l[start..end]).expect("utf8"))
            .map(|s| s.trim().parse::<u64>().ok())
            .collect::<Vec<Option<u64>>>()
    };

    for (index, ch) in operator_line.iter().enumerate() {
        if *ch == b'*' || *ch == b'+' {
            if let Some((start, _)) = indexes {
                columns.push(extract_column(start, index - 1));
                ops.push(operator);

                operator = if *ch == b'*' { Op::Mul } else { Op::Add };
                indexes = Some((index, index));
            } else {
                indexes = Some((index, index));
                operator = if *ch == b'*' { Op::Mul } else { Op::Add };
            }
        } else {
            if let Some((start, _)) = indexes {
                indexes = Some((start, index));
            } else {
                indexes = Some((index, index));
            }
        }
    }
    // Handle last column.
    if let Some((start, _)) = indexes {
        columns.push(extract_column(start, operator_line.len()));
        ops.push(operator);
    }

    Problem { columns, ops }
}

#[test]
fn test_parse() {
    let data = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    assert_eq!(
        parse(data.as_bytes()),
        Problem {
            columns: vec![
                vec![Some(123), Some(45), Some(6)],
                vec![Some(328), Some(64), Some(98)],
                vec![Some(51), Some(387), Some(215)],
                vec![Some(64), Some(23), Some(314)]
            ],
            ops: vec![Op::Mul, Op::Add, Op::Mul, Op::Add]
        }
    );
}

fn part_1(problem: &Problem) -> u64 {
    (0..problem.ops.len())
        .map(|i| {
            let op = &problem.ops[i];
            let zero = op.zero();
            let column = &problem.columns[i];

            column
                .iter()
                .fold(op.zero(), move |x, y| op.f(x, y.unwrap_or(op.zero())))
        })
        .sum()
}

#[test]
fn test_part_1() {
    let data = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    let problem = parse(data.as_bytes());
    assert_eq!(part_1(&problem), 4277556);
}

fn main() {
    let problem = parse(std::io::stdin().lock());
    println!("Part 1: {}", part_1(&problem))
}
