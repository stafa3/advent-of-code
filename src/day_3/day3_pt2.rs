use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    if let Ok(lines) = read_lines("src/day_3/input.txt") {
        let mut total_value = 0;
        let rucksacks: Vec<String> = lines.map(|l| l.expect("Could not parse line")).collect();

        for i in (0..rucksacks.len()).step_by(3) {
            let rucksack_1 = &rucksacks[i];
            let rucksack_2 = &rucksacks[i + 1];
            let rucksack_3 = &rucksacks[i + 2];

            let rucksack_1_chars: Vec<char> = rucksack_1.chars().collect();
            let rucksack_2_chars: Vec<char> = rucksack_2.chars().collect();
            let rucksack_3_chars: Vec<char> = rucksack_3.chars().collect();

            for c in rucksack_1_chars {
                if rucksack_2_chars.contains(&c) && rucksack_3_chars.contains(&c) {
                    let dupe_value = c as u32;
                    if dupe_value > 64 && dupe_value < 91 {
                        total_value = total_value + dupe_value - 38;
                    } else {
                        total_value = total_value + dupe_value - 96;
                    }
                    break;
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
