// use std::io::{BufRead, BufReader, Result};
//
// fn main() -> Result<()> {
//     let input = "\n28, 36, 1, 3, 9 \n 69, 420, 21, 22, 23".as_bytes();
//     let reader = BufReader::new(input);
//
//     let str_to_i32 = |str| i32::from_str_radix(str, 10).unwrap();
//
//     let mut lines = reader.lines().map(|wrapped| wrapped.unwrap());
//
//     for _i in 0..1 {
//         let line = lines.next().unwrap();
//         // TODO: Why doesn't this work?
//         let _nums = line.split_whitespace().map(str_to_i32);
//     }
//     Ok(())
// }
use std::io::Result;
fn main() -> Result<()> {
    Ok(())
}
