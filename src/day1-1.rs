use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let fin = File::open("./inputs/01.in")?;
    let reader = BufReader::new(fin);
    let mut ct = 0;
    let mut prev = None::<i32>;
    for line in reader.lines() {
        let cur = line.unwrap().parse::<i32>().unwrap();
        if let Some(prev) = prev {
            if prev < cur {
                ct += 1;
            }
        }
        prev = Some(cur);
    }
    println!("{}", ct);
    Ok(())
}
