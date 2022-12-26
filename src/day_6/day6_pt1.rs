use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    log::debug!("in main here");
    if let Ok(lines) = read_lines("src/day_6/input.txt") {
        for line in lines {
            let mut q: VecDeque<char> = VecDeque::new();
            if let Ok(line) = line {
                for (i, c) in line.chars().enumerate() {
                    if q.len() > 4 {
                        q.pop_front();
                    }
                    if q.len() == 4 {
                        //evaluate if the queue is a proper beginning sequence
                        let uniques = HashSet::<char>::from_iter(q.clone().into_iter());
                        if uniques.len() == 4 {
                            log::debug!("index: {}", i);
                            break;
                        }
                        //return index if yes
                    }
                    q.push_back(c);
                }
            }
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
