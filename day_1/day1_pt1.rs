use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut highest_elf_num: i32 = 1;
        let mut highest_elf_calories: i32 = 0;
        let mut curr_elf_num: i32 = 1;
        let mut total_elf_calories: i32 = 0;
        for line in lines {
            println!("total_elf_calories: {}", total_elf_calories);
            if let Ok(calorie) = line {
                if calorie.is_empty() {
                    if total_elf_calories > highest_elf_calories {
                        highest_elf_calories = total_elf_calories;
                        highest_elf_num = curr_elf_num;
                    }
                    curr_elf_num = curr_elf_num + 1;
                    total_elf_calories = 0;
                    continue;
                }

                total_elf_calories = total_elf_calories + calorie.parse::<i32>().unwrap();
            }
        }

        println!(
            "calories: {} elf num: {}",
            highest_elf_calories, highest_elf_num
        );
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
