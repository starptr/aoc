use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const BITWIDTH: usize = 12;

fn filter_nums(nums: HashSet<u16>, digit: usize, keep_one: bool) -> HashSet<u16> {
    nums.into_iter()
        .filter(|num| (num >> digit) & 0b1 == (if keep_one { 0b1 } else { 0b0 }))
        .collect()
}

fn apply(mut nums: HashSet<u16>, is_oxygen: bool) -> u16 {
    for i in (0..BITWIDTH).rev() {
        let mut ct = 0;
        for num in &nums {
            let digit = (num >> i) & 0b1;
            ct += digit;
        }

        let len = nums.len() as u16;
        let half = (nums.len() / 2) as u16;
        let zeroes = nums.len() as u16 - ct;
        nums = filter_nums(
            nums,
            i,
            if is_oxygen {
                if ct >= half {
                    true
                } else {
                    false
                }
            } else {
                if zeroes == 0 || (zeroes > half && zeroes != len) {
                    true
                } else {
                    false
                }
            },
        );

        if nums.len() == 1 {
            break;
        }
    }
    nums.into_iter().next().unwrap()
}

fn main() -> io::Result<()> {
    let fin = File::open("./inputs/03.in")?;
    let reader = BufReader::new(fin);
    let nums: HashSet<_> = reader
        .lines()
        .map(|line| u16::from_str_radix(&line.unwrap(), 2).unwrap())
        .collect();

    let oxy = apply(nums.clone(), true) as i32;
    let co2 = apply(nums, false) as i32;

    println!("{}", oxy * co2);

    Ok(())
}
