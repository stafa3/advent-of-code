use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut total_value = 0;
        for line in lines {
            if let Ok(pairs_str) = line {
                let pairs: Vec<&str> = pairs_str.split(',').collect();

                let first: Vec<&str> = pairs[0].split("-").collect();
                let second: Vec<&str> = pairs[1].split("-").collect();

                let first_left: u32 = first[0].parse().unwrap();
                let first_right: u32 = first[1].parse().unwrap();

                let second_left: u32 = second[0].parse().unwrap();
                let second_right: u32 = second[1].parse().unwrap();

                if first_left <= second_left && first_right >= second_right {
                    total_value = total_value + 1;
                } else if second_left <= first_left && second_right >= first_right {
                    total_value = total_value + 1;
                }
            }
        }
        println!("{}", total_value);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
