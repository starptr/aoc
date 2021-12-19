use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const SIZE: usize = 1000;

fn main() -> Result<()> {
    let fin = File::open("./inputs/05.in")?;
    let reader = BufReader::new(fin);

    let mut grid = [[0; SIZE]; SIZE];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split(" -> ");
        let start = line.next().unwrap();
        let end = line.next().unwrap();
        let mut start = start.split(',');
        let mut end = end.split(',');
        let (x1, y1) = (start.next().unwrap(), start.next().unwrap());
        let (x2, y2) = (end.next().unwrap(), end.next().unwrap());
        let x1 = usize::from_str_radix(x1, 10).unwrap();
        let y1 = usize::from_str_radix(y1, 10).unwrap();
        let x2 = usize::from_str_radix(x2, 10).unwrap();
        let y2 = usize::from_str_radix(y2, 10).unwrap();

        if x1 == x2 {
            for y in min(y1, y2)..(max(y1, y2) + 1) {
                grid[x1][y] += 1;
            }
        } else if y1 == y2 {
            for x in min(x1, x2)..(max(x1, x2) + 1) {
                grid[x][y1] += 1;
            }
        } else {
            // force x1 to be less than x2
            let mut x1 = x1;
            let mut x2 = x2;
            let mut y1 = y1;
            let mut y2 = y2;
            if x1 > x2 {
                let tmp = x1;
                x1 = x2;
                x2 = tmp;
                let tmp = y1;
                y1 = y2;
                y2 = tmp;
            }
            if y1 < y2 {
                for delta in 0..(x2 - x1 + 1) {
                    grid[x1 + delta][y1 + delta] += 1;
                }
            } else {
                for delta in 0..(x2 - x1 + 1) {
                    grid[x1 + delta][y1 - delta] += 1;
                }
            }
        }
    }

    let mut ct = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] >= 2 {
                ct += 1;
            }
        }
    }
    println!("{}", ct);

    Ok(())
}
