use std::io::stdin;

fn read_directions() -> Vec<i32> {
    stdin()
        .lines()
        .map_while(|s| s.ok())
        .map(|s| {
            (if s.starts_with("L") { -1 } else { 1 }) * s[1..].parse::<i32>().expect("parse error")
        })
        .collect()
}

fn main() {
    println!("Day 1");
    let directions = read_directions();

    // Part 1
    {
        let mut count_zeros = 0;
        let mut dial = 50;
        for direction in &directions {
            dial = (dial + direction).rem_euclid(100);
            if dial == 0 {
                count_zeros += 1;
            }
        }
        println!("Part 1: count_zeros: {}", count_zeros);
    }

    // Part 2
    {
        let mut dial = 50;
        let mut count_zeros = 0;
        for direction in directions {
            let unscaled_dial = dial + direction;

            if unscaled_dial == 0 {
                count_zeros += 1;
            } else if unscaled_dial < 0 {
                count_zeros += if dial == 0 { 0 } else { 1 } + (-unscaled_dial) / 100;
            } else if unscaled_dial / 100 > 0 {
                count_zeros += unscaled_dial / 100;
            }

            dial = unscaled_dial.rem_euclid(100);
        }
        println!("Part 2: count_zeros: {}", count_zeros);
    }
}
