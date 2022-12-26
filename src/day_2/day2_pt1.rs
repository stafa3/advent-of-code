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
                let mine = round_vec[2];

                let result_score = get_result_score(opponent, mine);
                if mine == 'X' {
                    total_score = total_score + result_score + 1
                    // println!("round: {}, {} result_score: {} bonus: {} total_score: {}", opponent, mine, result_score, 1, total_score)
                } else if mine == 'Y' {
                    total_score = total_score + result_score + 2
                    // println!("round: {}, {} result_score: {} bonus: {} total_score: {}", opponent, mine, result_score, 2, total_score)
                } else {
                    total_score = total_score + result_score + 3
                    // println!("round: {}, {} result_score: {} bonus: {} total_score: {}", opponent, mine, result_score, 3, total_score)
                }
            }
        }
        println!("{}", total_score);
    }
}

fn get_result_score(opponent: char, mine: char) -> i32 {
    if opponent == 'A' {
        if mine == 'X' {
            3
        } else if mine == 'Y' {
            6
        } else {
            0
        }
    } else if opponent == 'B' {
        if mine == 'X' {
            0
        } else if mine == 'Y' {
            3
        } else {
            6
        }
    } else {
        if mine == 'X' {
            6
        } else if mine == 'Y' {
            0
        } else {
            3
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
