use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

struct Board {
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
        let coord = self.num_coords.get(&num);
        if let Some(coord) = coord {
            self.matrix[coord.0][coord.1] = true;

            // check row-bingo
            if self.matrix[coord.0].into_iter().all(|b| b) {
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
            matrix: [[false; 5]; 5],
            num_coords,
        }
    }
}

fn find_score_of_best(seq: &mut dyn Iterator<Item = i32>, mut boards: Vec<Board>) -> i32 {
    while let Some(num) = seq.next() {
        for board in &mut boards {
            if let Some(score) = board.mark_and_score(num) {
                return score;
            }
        }
    }
    return 0;
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
            // let nums = line.split_whitespace().map(str_to_i32);
            let mut nums = line
                .split_whitespace()
                .map(|str| i32::from_str_radix(str, 10).unwrap());

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
    println!("{}", find_score_of_best(&mut seq, boards));
    Ok(())
}
