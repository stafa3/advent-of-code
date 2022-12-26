use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    if let Ok(lines) = read_lines("src/day_1/input.txt") {
        let mut heap = BinaryHeap::new();
        let mut total_elf_calories: i32 = 0;

        for line in lines {
            println!("total_elf_calories: {}", total_elf_calories);
            if let Ok(calorie) = line {
                if calorie.is_empty() {
                    heap.push(total_elf_calories);
                    total_elf_calories = 0;
                    continue;
                }

                total_elf_calories = total_elf_calories + calorie.parse::<i32>().unwrap();
            }
        }
        let top_1: i32 = heap.pop().unwrap();
        let top_2: i32 = heap.pop().unwrap();
        let top_3: i32 = heap.pop().unwrap();

        println!(
            "1: {} 2: {} 3: {} total: {}",
            top_1,
            top_2,
            top_3,
            top_1 + top_2 + top_3
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
