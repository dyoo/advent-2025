use std::io::{BufRead, stdin};
use std::cmp::max;

#[derive(Debug)]
struct Bank {
    voltages: Vec<u8>,
}

fn read_input(buf_read: impl BufRead) -> Vec<Bank> {
    buf_read
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| Bank { voltages: l.as_bytes().into_iter().map(|b| b - b'0').collect() })
        .collect()
}


impl Bank {
    fn max_voltage(&self) -> u8 {
        let mut best = self.voltages[0] * 10 + self.voltages[1];
        for i in 0..self.voltages.len() - 1 {
            for j in i+1..self.voltages.len() {
                let candidate = self.voltages[i] * 10 + self.voltages[j];
                best = max(best, candidate);
            }
        }
        best
    }
}

#[test]
fn test_max_voltage() {
    let bank = Bank{voltages: vec![9,8,7,6,5,4,3,2,1,1,1,1,1]};
    assert_eq!(bank.max_voltage(), 98);

    let bank = Bank{voltages: vec![8, 1, 1, 9]};
    assert_eq!(bank.max_voltage(), 89);
}

fn part_1(input: &[Bank]) -> u64 {
    input.iter().map(|bank| bank.max_voltage() as u64).sum()
}


fn main() {
    let input = read_input(stdin().lock());
    println!("Part 1: {:?}", part_1(&input));
}
