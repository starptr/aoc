use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let fin = File::open("./inputs/01.in")?;
    let reader = BufReader::new(fin);
    let mut window = LinkedList::new();
    let mut ct = 0;
    for line in reader.lines() {
        let cur = line.unwrap().parse::<i32>().unwrap();
        window.push_back(cur);
        if window.len() == 4 {
            if window.back() > window.front() {
                ct += 1;
            }
            window.pop_front();
        }
    }
    println!("{}", ct);
    Ok(())
}
