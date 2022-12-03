use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut total_value = 0;
        for line in lines {
            if let Ok(rucksack) = line {
                let half_size = rucksack.len() / 2;
                let (first_half, second_half) = rucksack.split_at(half_size);

                let first_half_vec: Vec<char> = first_half.chars().collect();
                let second_half_vec: Vec<char> = second_half.chars().collect();
                for c in first_half_vec {
                    if second_half_vec.contains(&c) {
                        let dupe_value = c as u32;

                        if dupe_value > 64 && dupe_value < 91 {
                            total_value = total_value + dupe_value - 38;
                            println!("{}: {}, converted: {}", c, dupe_value, dupe_value - 38);
                        } else {
                            total_value = total_value + dupe_value - 96;
                            println!("{}: {}, converted: {}", c, dupe_value, dupe_value - 96);
                        }
                        break;
                    };
                }
            }
        }
        println!("total_value: {}", total_value);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
