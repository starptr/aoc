use std::fs::File;
use std::io::{self, BufRead, BufReader};

const BITWIDTH: usize = 12;
const BITMASK: u32 = 0xFFF;

fn main() -> io::Result<()> {
    let fin = File::open("./inputs/03.in")?;
    let reader = BufReader::new(fin);

    let mut ct = vec![0; BITWIDTH];
    let mut line_ct = 0;
    for line in reader.lines() {
        line_ct += 1;

        let mut num = u16::from_str_radix(&line.unwrap(), 2).unwrap();
        for i in 0..BITWIDTH {
            ct[i] += num & 0b1;
            num >>= 1;
        }
    }

    let mut gamma = 0;
    for i in 0..BITWIDTH {
        if ct[i] > line_ct / 2 {
            gamma |= 1 << i;
        }
    }
    let gamma = gamma;
    let epsilon = !gamma & BITMASK;
    println!("{}", gamma * epsilon);
    Ok(())
}
