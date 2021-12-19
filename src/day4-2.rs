use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

struct Board {
    is_done: bool,
    matrix: [[bool; 5]; 5],
    num_coords: HashMap<i32, (usize, usize)>,
}

impl Board {
    fn unmarked_sum(&self) -> i32 {
        let mut sum = 0;
        for (num, (i, j)) in &self.num_coords {
            if !self.matrix[*i][*j] {
                sum += num;
            }
        }
        sum
    }
    fn mark_and_score(&mut self, num: i32) -> Option<i32> {
        if self.is_done {
            return None;
        }

        let coord = self.num_coords.get(&num);
        if let Some(coord) = coord {
            self.matrix[coord.0][coord.1] = true;

            // check row-bingo
            if self.matrix[coord.0].into_iter().all(|b| b) {
                self.is_done = true;
                return Some(self.unmarked_sum() * num);
            }

            // check col-bingo
            let mut is_bingo = true;
            for i in 0..5 {
                if !self.matrix[i][coord.1] {
                    is_bingo = false;
                    break;
                }
            }
            if is_bingo {
                self.is_done = true;
                return Some(self.unmarked_sum() * num);
            }
        }
        None
    }
    fn new(matrix: [[i32; 5]; 5]) -> Board {
        let mut num_coords = HashMap::new();
        for i in 0..5 {
            for j in 0..5 {
                num_coords.insert(matrix[i][j], (i, j));
            }
        }
        Board {
            is_done: false,
            matrix: [[false; 5]; 5],
            num_coords,
        }
    }
}

fn find_score_of_worst(seq: &mut dyn Iterator<Item = i32>, mut boards: Vec<Board>) -> i32 {
    let mut last_score = 0;
    while let Some(num) = seq.next() {
        for board in &mut boards {
            if let Some(score) = board.mark_and_score(num) {
                println!("{}", last_score);
                last_score = score;
            }
        }
    }
    return last_score;
}

fn main() -> Result<()> {
    let fin = File::open("./inputs/04.in")?;
    let reader = BufReader::new(fin);

    let str_to_i32 = |str| i32::from_str_radix(str, 10).unwrap();

    let mut lines = reader.lines().map(|wrapped| wrapped.unwrap());
    let seq = lines.next().unwrap();
    let mut seq = seq.split(',').map(str_to_i32);

    let mut boards: Vec<Board> = Vec::new();
    while let Some(_blank_line) = lines.next() {
        let mut matrix = [[0; 5]; 5];
        for i in 0..5 {
            let line = lines.next().unwrap();
            // TODO: Why doesn't this work?
            let mut nums = line.split_whitespace().map(str_to_i32);
            //let mut nums = line
            //    .split_whitespace()
            //    .map(|str| i32::from_str_radix(str, 10).unwrap());

            matrix[i] = [
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            ];
        }
        boards.push(Board::new(matrix));
    }
    println!("{}", find_score_of_worst(&mut seq, boards));
    Ok(())
}
