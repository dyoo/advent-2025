use std::io;
use std::io::BufRead;

type Point = (u64, u64, u64);

fn parse(buf: impl BufRead) -> Vec<Point> {
    buf.lines()
        .map_while(|l| l.ok())
        .map(|l| {
            let mut chunks = l.split(',');
            let x = chunks.next().expect("x").parse().expect("number");
            let y = chunks.next().expect("y").parse().expect("number");
            let z = chunks.next().expect("z").parse().expect("number");
            (x, y, z)
        })
        .collect()
}

fn dist_sq(a: Point, b: Point) -> u64 {
    a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2)
}

// Compute the dist_sq metric between points.  i < j, returning a list of them in increasing metric.
fn metric_between_pairs(points: &[Point]) -> Vec<(u64, usize, usize)> {
    let mut result: Vec<(u64, usize, usize)> = (0..points.len())
        .flat_map(|i| (i + 1..points.len()).map(move |j| (dist_sq(points[i], points[j]), i, j)))
        .collect();

    result.sort();
    result
}

struct UnionSet {
    up: Vec<usize>,
}

impl UnionSet {
    fn new(n: usize) -> Self {
        let up = (0..n).collect();
        Self { up }
    }

    fn merge(&mut self, i: usize, j: usize) {
        let i_id = self.id(i);
        let j_id = self.id(j);
        self.up[i_id] = j_id;
    }

    fn id(&mut self, i: usize) -> usize {
        if self.up[i] == i {
            return i;
        }

        let mut to_change = vec![i];
        let mut i = self.up[i];
        loop {
            let up = self.up[i];
            if up == i {
                break;
            }
            to_change.push(i);
            i = up;
        }

        // Path compression
        for item in to_change {
            self.up[item] = i;
        }

        i
    }
}

fn part_1(points: &[Point], n_to_pair: usize, n_biggest: usize) -> usize {
    let n = points.len();

    let metrics = metric_between_pairs(points);

    let mut union_set = UnionSet::new(n);
    for metric in &metrics[0..n_to_pair] {
        union_set.merge(metric.1, metric.2);
    }

    let mut sizes = vec![0; n];
    for i in 0..n {
        let index = union_set.id(i);
        sizes[index] += 1;
    }
    sizes.sort();
    sizes[sizes.len() - n_biggest..].iter().product()
}

fn part_2(points: &[Point]) -> u64 {
    let n = points.len();

    let metrics = metric_between_pairs(points);

    let mut union_set = UnionSet::new(n);
    for metric in metrics {
        union_set.merge(metric.1, metric.2);

        // Check the size of the component that 0 is in.  If it's n,
        // we're fully connected and can stop.
        let mut sizes = vec![0; n];
        for i in 0..n {
            let index = union_set.id(i);
            sizes[index] += 1;
        }
        if sizes[union_set.id(0)] == n {
            return points[metric.1].0 * points[metric.2].0;
        }
    }

    // Defensive; we should never get here.
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_metric_between_pairs() {
        let points = parse(DATA.as_bytes());
        let metrics = metric_between_pairs(&points);
        assert_eq!(metrics[0].1, 0); // 162,817,812
        assert_eq!(metrics[0].2, 19); // 425,690,689

        assert_eq!(metrics[1].1, 0); // 162,817,812
        assert_eq!(metrics[1].2, 7); // 431,825,988
    }

    #[test]
    fn test_part_1() {
        let points = parse(DATA.as_bytes());
        assert_eq!(part_1(&points, 10, 3), 40);
    }

    #[test]
    fn test_part_2() {
        let points = parse(DATA.as_bytes());
        assert_eq!(part_2(&points), 25272);
    }
}

fn main() {
    let problem = parse(io::stdin().lock());
    println!("{:?}", part_1(&problem, 1000, 3));
    println!("{:?}", part_2(&problem));
}
