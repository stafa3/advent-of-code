use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use log::debug;

pub fn main() {
    if let Ok(lines_object) = read_lines("src/day_10/input.txt") {
        let lines: Vec<String> = lines_object.collect::<Result<_, _>>().unwrap();
        let mut cycle = 1;
        let mut should_do_operation = false;
        let mut x = 1;
        let mut line_counter = 0;
        let mut line = lines[line_counter].clone();
        let mut crt_line: Vec<&str> = Vec::new();
        //cycle loop
        loop {
            if line_counter >= lines.len() {
                break;
            }
            if (x - 1..x + 2).contains(&((cycle - 1) % 40)) {
                crt_line.push("#");
            } else {
                crt_line.push(".");
            }
            line = lines[line_counter].clone();
            if (cycle % 40) == 0 {
                debug!("{}", crt_line.join(""));
                crt_line.clear();
            }
            if line.starts_with("addx") {
                if should_do_operation {
                    let val: i32 = line.split_whitespace().collect::<Vec<&str>>()[1]
                        .parse()
                        .unwrap();
                    x += val;
                    should_do_operation = !should_do_operation;
                    line_counter += 1;
                } else {
                    should_do_operation = !should_do_operation;
                }
            } else {
                line_counter += 1;
            }
            cycle += 1;
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
