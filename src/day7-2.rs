use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn compute_fuel(crab_locs: &[i32], pos: usize) -> i32 {
    let dist_to_fuel = |dist: i32| dist * (dist + 1) / 2;
    crab_locs
        .iter()
        .fold(0, |acc, &cur| acc + dist_to_fuel((cur - pos as i32).abs()))
}

fn bsearch(crab_locs: &[i32], left: usize, right: usize) -> i32 {
    if left >= right {
        return compute_fuel(crab_locs, left);
    }
    let mid = (left + right) / 2;
    let left_fuel = compute_fuel(crab_locs, left);
    let right_fuel = compute_fuel(crab_locs, right);
    let mid_fuel = compute_fuel(crab_locs, mid);
    if mid_fuel < left_fuel && mid_fuel < right_fuel {
        mid_fuel
    } else if mid_fuel < left_fuel {
        bsearch(crab_locs, mid + 1, right)
    } else if mid_fuel < right_fuel {
        bsearch(crab_locs, left, mid)
    } else {
        panic!("Unhandled case");
    }
}

fn main() -> Result<()> {
    let fin = File::open("./inputs/07.in")?;
    let mut reader = BufReader::new(fin);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let crab_locs: Vec<_> = line
        .trim()
        .split(',')
        .map(|s: &str| i32::from_str_radix(s, 10).unwrap())
        .collect();

    println!("{}", bsearch(&crab_locs, 0, crab_locs.len()));
    Ok(())
}
