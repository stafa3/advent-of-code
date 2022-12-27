use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use log::debug;

pub fn main() {
    let mut head_x: i32 = 0;
    let mut head_y: i32 = 0;

    let mut tail_x: i32 = 0;
    let mut tail_y: i32 = 0;

    let mut tail_visited_coordinates = HashSet::<String>::new();

    //start at 0,0 for both head and tail
    //for each line
    //go the direction, move head and adjust tail
    //add the resulting tail coordinates to a visited set
    //return length of the visited set
    if let Ok(lines) = read_lines("src/day_9/input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let direction = line.split_whitespace().collect::<Vec<&str>>()[0];
                let amount: i32 = line.split_whitespace().collect::<Vec<&str>>()[1]
                    .parse()
                    .unwrap();
                if direction == "L" {
                    for _ in 0..amount {
                        head_x -= 1;
                        if ((head_x - tail_x).abs() == 2 && head_y == tail_y)
                            || ((head_y - tail_y).abs() == 2 && head_x == tail_x)
                        {
                            tail_x -= 1;
                        } else if !((head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1) {
                            tail_x -= 1;
                            tail_y = head_y;
                        }
                        tail_visited_coordinates.insert(format!("{},{}", tail_x, tail_y));
                    }
                } else if direction == "R" {
                    for _ in 0..amount {
                        head_x += 1;
                        if ((head_x - tail_x).abs() == 2 && head_y == tail_y)
                            || ((head_y - tail_y).abs() == 2 && head_x == tail_x)
                        {
                            tail_x += 1;
                        } else if !((head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1) {
                            tail_x += 1;
                            tail_y = head_y;
                        }
                        tail_visited_coordinates.insert(format!("{},{}", tail_x, tail_y));
                    }
                } else if direction == "U" {
                    for _ in 0..amount {
                        head_y += 1;
                        if ((head_x - tail_x).abs() == 2 && head_y == tail_y)
                            || ((head_y - tail_y).abs() == 2 && head_x == tail_x)
                        {
                            tail_y += 1;
                        } else if !((head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1) {
                            tail_y += 1;
                            tail_x = head_x;
                        }
                        tail_visited_coordinates.insert(format!("{},{}", tail_x, tail_y));
                    }
                } else if direction == "D" {
                    for _ in 0..amount {
                        head_y -= 1;
                        if ((head_x - tail_x).abs() == 2 && head_y == tail_y)
                            || ((head_y - tail_y).abs() == 2 && head_x == tail_x)
                        {
                            tail_y -= 1;
                        } else if !((head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1) {
                            tail_y -= 1;
                            tail_x = head_x;
                        }
                        tail_visited_coordinates.insert(format!("{},{}", tail_x, tail_y));
                    }
                } else {
                    debug!("uh oh spaghettios");
                }
            }
        }

        debug!("result: {}, {}", head_x, head_y);
    }

    debug!("{}", tail_visited_coordinates.len());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
