use aoc2025::{count_digits, implode_digits};
use std::cmp::max;
use std::collections::{HashMap, hash_map::Entry};
use std::io::{BufRead, stdin};

#[derive(Debug)]
struct Bank {
    voltages: Vec<u8>,
}

fn read_input(buf_read: impl BufRead) -> Vec<Bank> {
    buf_read
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| Bank {
            voltages: l.as_bytes().into_iter().map(|b| b - b'0').collect(),
        })
        .collect()
}

impl Bank {
    fn max_voltage(&self) -> u8 {
        let mut best = self.voltages[0] * 10 + self.voltages[1];
        for i in 0..self.voltages.len() - 1 {
            for j in i + 1..self.voltages.len() {
                let candidate = self.voltages[i] * 10 + self.voltages[j];
                best = max(best, candidate);
            }
        }
        best
    }
}

#[test]
fn test_max_voltage() {
    let bank = Bank {
        voltages: vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1],
    };
    assert_eq!(bank.max_voltage(), 98);

    let bank = Bank {
        voltages: vec![8, 1, 1, 9],
    };
    assert_eq!(bank.max_voltage(), 89);
}

fn part_1(input: &[Bank]) -> u64 {
    input.iter().map(|bank| bank.max_voltage() as u64).sum()
}

fn max_voltage2(voltages: &[u8]) -> u64 {
    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();
    search(voltages, 0, 12, &mut cache)
}

fn search(
    voltages: &[u8],
    start: usize,
    capacity: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Entry::Occupied(o) = cache.entry((start, capacity)) {
        return *o.get();
    }

    let result = {
        if capacity > voltages[start..].len() {
            0
        } else if capacity == voltages[start..].len() {
            implode_digits(&voltages[start..])
        } else if capacity == 1 {
            *voltages[start..].iter().max().expect("empty voltages") as u64
        } else {
            let picking_first = search(voltages, start + 1, capacity - 1, cache);
            let choice = 10u64.pow(count_digits(picking_first)) as u64 * voltages[start] as u64
                + picking_first;

            max(choice, search(&voltages, start + 1, capacity, cache))
        }
    };

    cache.insert((start, capacity), result);
    result
}

#[test]
fn test_max_voltage2() {
    assert_eq!(
        max_voltage2(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
        987654321111
    );
    assert_eq!(
        max_voltage2(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
        434234234278
    );
    assert_eq!(
        max_voltage2(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
        888911112111
    );
}

fn part_2(input: &[Bank]) -> u64 {
    input
        .iter()
        .map(|bank| max_voltage2(&bank.voltages) as u64)
        .sum()
}

fn main() {
    let input = read_input(stdin().lock());
    println!("Part 1: {:?}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
}
