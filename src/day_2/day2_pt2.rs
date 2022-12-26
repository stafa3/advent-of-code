use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    if let Ok(lines) = read_lines("src/day_2/input.txt") {
        let mut total_score = 0;
        for line in lines {
            if let Ok(round) = line {
                let round_vec: Vec<char> = round.chars().collect();
                let opponent = round_vec[0];
                let strategy = round_vec[2];

                if strategy == 'X' {
                    if opponent == 'A' {
                        total_score = total_score + 0 + 3
                    } else if opponent == 'B' {
                        total_score = total_score + 0 + 1
                    } else {
                        total_score = total_score + 0 + 2
                    }
                    // println!("round: {}, {} result_score: {} bonus: {} total_score: {}", opponent, mine, result_score, 1, total_score)
                } else if strategy == 'Y' {
                    if opponent == 'A' {
                        total_score = total_score + 3 + 1
                    } else if opponent == 'B' {
                        total_score = total_score + 3 + 2
                    } else {
                        total_score = total_score + 3 + 3
                    }
                    // println!("round: {}, {} result_score: {} bonus: {} total_score: {}", opponent, mine, result_score, 2, total_score)
                } else {
                    if opponent == 'A' {
                        total_score = total_score + 6 + 2
                    } else if opponent == 'B' {
                        total_score = total_score + 6 + 3
                    } else {
                        total_score = total_score + 6 + 1
                    }
                    // println!("round: {}, {} result_score: {} bonus: {} total_score: {}", opponent, mine, result_score, 3, total_score)
                }
            }
        }
        println!("{}", total_score);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
