use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const DAYS: usize = 256;
const TIMER_STATES: usize = 9;

fn main() -> Result<()> {
    let fin = File::open("./inputs/06.in")?;
    let mut reader = BufReader::new(fin);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let starting_fish = line
        .trim()
        .split(',')
        .map(|s: &str| i32::from_str_radix(s, 10).unwrap());

    let mut counts: Vec<i64> = vec![0; TIMER_STATES];

    for timer in starting_fish {
        counts[timer as usize] += 1;
    }

    for _ in 0..DAYS {
        let baby_ct = counts[0];
        for i in 0..(TIMER_STATES - 1) {
            counts[i] = counts[i + 1];
        }
        counts[6] += baby_ct;
        *counts.last_mut().unwrap() = baby_ct;
    }
    println!("{}", counts.iter().sum::<i64>());
    Ok(())
}
