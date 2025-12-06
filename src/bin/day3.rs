use std::io::{BufRead, stdin};
use std::cmp::max;
use aoc2025::{count_digits, implode_digits};

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

impl Bank {
    fn max_voltage2(&self) -> u64 {
        search(&self.voltages, 0, 12)
    }
}

fn search(voltages: &[u8], start: usize, capacity: usize) -> u64 {
    if capacity > voltages[start..].len() {
        return 0;
    } else if capacity == voltages[start..].len() {
        return implode_digits(&voltages[start..]);
    } else if capacity == 1 {
        return *voltages[start..].iter().max().expect("empty voltages") as u64;
    } 

    let picking_first = search(voltages, start + 1, capacity-1);
    let choice =
        10u64.pow(count_digits(picking_first)) as u64 * voltages[start] as u64 + picking_first;

    let best_choice = max(choice, search(&voltages, start + 1, capacity));
    best_choice
}

#[test]
fn test_search() {
    assert_eq!(search(&vec![1,2,3,4], 0, 5), 0);
    assert_eq!(search(&vec![1,2,3,4], 0, 4), 1234);
    assert_eq!(search(&vec![1,2,3,4], 0, 3), 234);
    assert_eq!(search(&vec![1,2,3,4], 0, 2), 34);
    assert_eq!(search(&vec![1,2,3,4], 0, 1), 4);

    assert_eq!(search(&vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 0, 12),
        987654321111);
    assert_eq!(search(&vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], 0, 12),
        434234234278);
    assert_eq!(search(&vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 0, 12),
        888911112111);
}


fn part_2(input: &[Bank]) -> u64 {
    input.iter().map(|bank| bank.max_voltage2() as u64).sum()
}


fn main() {
    let input = read_input(stdin().lock());
    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}
