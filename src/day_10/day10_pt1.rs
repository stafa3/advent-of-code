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
        let mut signal_strength_sum = 0;
        //cycle loop
        loop {
            if line_counter >= lines.len() {
                break;
            }
            line = lines[line_counter].clone();
            if ((cycle - 20) % 40) == 0 {
                debug!("[cycle: {}][x: {}][{}] x: {}", cycle, x, line, x);
                signal_strength_sum += cycle * x;
            }
            if line.starts_with("addx") {
                if should_do_operation {
                    let val: i32 = line.split_whitespace().collect::<Vec<&str>>()[1]
                        .parse()
                        .unwrap();
                    // debug!(
                    //     "[cycle: {}][x: {}][{}] Doing operation, adding {} to x",
                    //     cycle, x, line, val
                    // );
                    x += val;
                    should_do_operation = !should_do_operation;
                    line_counter += 1;
                } else {
                    // debug!("[cycle: {}][x: {}][{}] Not doing operation", cycle, x, line);
                    should_do_operation = !should_do_operation;
                }
            } else {
                // debug!(
                //     "[cycle: {}][x: {}][{}] Moving to next cycle and line",
                //     cycle, x, line
                // );
                line_counter += 1;
            }
            cycle += 1;
        }
        debug!("{}", signal_strength_sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
