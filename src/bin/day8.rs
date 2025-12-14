use std::io::BufRead;
use std::io;

fn parse(buf: impl BufRead) -> Vec<(u32, u32, u32)> {
    buf.lines().filter_map(|l|l.ok())
        .map(|l| {
            let mut chunks = l.split(',');
            let x: u32 = chunks.next().expect("x").parse().expect("number");
            let y: u32 = chunks.next().expect("y").parse().expect("number");
            let z: u32 = chunks.next().expect("z").parse().expect("number");
            (x, y, z)
        })
        .collect()
}

fn main()  {
    let problem = parse(io::stdin().lock());
    println!("{:?}", problem);
}
