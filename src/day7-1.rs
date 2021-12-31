use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let fin = File::open("./inputs/07.in")?;
    let mut reader = BufReader::new(fin);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let mut crab_locs: Vec<_> = line
        .trim()
        .split(',')
        .map(|s: &str| i32::from_str_radix(s, 10).unwrap())
        .collect();

    crab_locs.sort_unstable();
    let best_pos = crab_locs[crab_locs.len() / 2];
    let mut total_fuel = 0;
    for crab in crab_locs {
        total_fuel += (crab - best_pos).abs();
    }
    println!("{}", total_fuel);
    Ok(())
}
