use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let fin = File::open("./inputs/02.in")?;
    let reader = BufReader::new(fin);
    let mut pos = (0, 0);
    let mut aim = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let tokens = line.split(' ').collect::<Vec<&str>>();
        let cmd = tokens[0];
        let offset = tokens[1].parse::<i32>().unwrap();
        match cmd {
            "forward" => {
                pos.0 += offset;
                pos.1 += aim * offset;
            }
            "up" => aim -= offset,
            "down" => aim += offset,
            _ => panic!("Invalid command"),
        }
    }
    println!("{}", pos.0 * pos.1);
    Ok(())
}
