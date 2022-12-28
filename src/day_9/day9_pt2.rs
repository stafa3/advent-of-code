use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Div;
use std::path::Path;

use log::debug;

pub fn main() {
    let mut knot_coordinates: Vec<(i32, i32)> = vec![(0, 0); 10];

    let mut tail_visited_coordinates = HashSet::<String>::new();
    if let Ok(lines) = read_lines("src/day_9/input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let direction = line.split_whitespace().collect::<Vec<&str>>()[0];
                let amount: i32 = line.split_whitespace().collect::<Vec<&str>>()[1]
                    .parse()
                    .unwrap();
                if direction == "L" {
                    for _ in 0..amount {
                        knot_coordinates[0].0 -= 1;
                        for knot in 1..10 {
                            let x_delta = knot_coordinates[((knot - 1) as usize)].0
                                - knot_coordinates[(knot as usize)].0;
                            let y_delta = knot_coordinates[((knot - 1) as usize)].1
                                - knot_coordinates[(knot as usize)].1;
                            if knot_coordinates[((knot - 1) as usize)].0
                                == knot_coordinates[(knot as usize)].0
                            {
                                if y_delta == 2 {
                                    knot_coordinates[knot as usize].1 += 1;
                                } else if y_delta == -2 {
                                    knot_coordinates[knot as usize].1 -= 1;
                                }
                            } else if knot_coordinates[((knot - 1) as usize)].1
                                == knot_coordinates[knot as usize].1
                            {
                                if x_delta == 2 {
                                    knot_coordinates[knot as usize].0 += 1;
                                } else if x_delta == -2 {
                                    knot_coordinates[knot as usize].0 -= 1;
                                }
                            } else if !touching(
                                knot_coordinates[((knot - 1) as usize)],
                                knot_coordinates[(knot as usize)],
                            ) {
                                let x_delta = knot_coordinates[((knot - 1) as usize)].0
                                    - knot_coordinates[(knot as usize)].0;
                                let y_delta = knot_coordinates[((knot - 1) as usize)].1
                                    - knot_coordinates[(knot as usize)].1;
                                knot_coordinates[(knot as usize)].0 += x_delta.div(x_delta.abs());
                                knot_coordinates[(knot as usize)].1 += y_delta.div(y_delta.abs());
                            }
                        }
                        tail_visited_coordinates.insert(format!(
                            "{},{}",
                            knot_coordinates[9].0, knot_coordinates[9].1
                        ));
                    }
                } else if direction == "R" {
                    for _ in 0..amount {
                        knot_coordinates[0].0 += 1;
                        for knot in 1..10 {
                            let x_delta = knot_coordinates[((knot - 1) as usize)].0
                                - knot_coordinates[(knot as usize)].0;
                            let y_delta = knot_coordinates[((knot - 1) as usize)].1
                                - knot_coordinates[(knot as usize)].1;
                            if knot_coordinates[((knot - 1) as usize)].0
                                == knot_coordinates[(knot as usize)].0
                            {
                                if y_delta == 2 {
                                    knot_coordinates[knot as usize].1 += 1;
                                } else if y_delta == -2 {
                                    knot_coordinates[knot as usize].1 -= 1;
                                }
                            } else if knot_coordinates[((knot - 1) as usize)].1
                                == knot_coordinates[knot as usize].1
                            {
                                if x_delta == 2 {
                                    knot_coordinates[knot as usize].0 += 1;
                                } else if x_delta == -2 {
                                    knot_coordinates[knot as usize].0 -= 1;
                                }
                            } else if !touching(
                                knot_coordinates[((knot - 1) as usize)],
                                knot_coordinates[(knot as usize)],
                            ) {
                                let x_delta = knot_coordinates[((knot - 1) as usize)].0
                                    - knot_coordinates[(knot as usize)].0;
                                let y_delta = knot_coordinates[((knot - 1) as usize)].1
                                    - knot_coordinates[(knot as usize)].1;
                                knot_coordinates[(knot as usize)].0 += x_delta.div(x_delta.abs());
                                knot_coordinates[(knot as usize)].1 += y_delta.div(y_delta.abs());
                            }
                        }

                        tail_visited_coordinates.insert(format!(
                            "{},{}",
                            knot_coordinates[9].0, knot_coordinates[9].1
                        ));
                    }
                } else if direction == "U" {
                    for _ in 0..amount {
                        knot_coordinates[0].1 += 1;
                        for knot in 1..10 {
                            let x_delta = knot_coordinates[((knot - 1) as usize)].0
                                - knot_coordinates[(knot as usize)].0;
                            let y_delta = knot_coordinates[((knot - 1) as usize)].1
                                - knot_coordinates[(knot as usize)].1;
                            if knot_coordinates[((knot - 1) as usize)].0
                                == knot_coordinates[(knot as usize)].0
                            {
                                if y_delta == 2 {
                                    knot_coordinates[knot as usize].1 += 1;
                                } else if y_delta == -2 {
                                    knot_coordinates[knot as usize].1 -= 1;
                                }
                            } else if knot_coordinates[((knot - 1) as usize)].1
                                == knot_coordinates[knot as usize].1
                            {
                                if x_delta == 2 {
                                    knot_coordinates[knot as usize].0 += 1;
                                } else if x_delta == -2 {
                                    knot_coordinates[knot as usize].0 -= 1;
                                }
                            } else if !touching(
                                knot_coordinates[((knot - 1) as usize)],
                                knot_coordinates[(knot as usize)],
                            ) {
                                let x_delta = knot_coordinates[((knot - 1) as usize)].0
                                    - knot_coordinates[(knot as usize)].0;
                                let y_delta = knot_coordinates[((knot - 1) as usize)].1
                                    - knot_coordinates[(knot as usize)].1;
                                knot_coordinates[(knot as usize)].0 += x_delta.div(x_delta.abs());
                                knot_coordinates[(knot as usize)].1 += y_delta.div(y_delta.abs());
                            }
                        }

                        tail_visited_coordinates.insert(format!(
                            "{},{}",
                            knot_coordinates[9].0, knot_coordinates[9].1
                        ));
                    }
                } else if direction == "D" {
                    for _ in 0..amount {
                        knot_coordinates[0].1 -= 1;
                        for knot in 1..10 {
                            let x_delta = knot_coordinates[((knot - 1) as usize)].0
                                - knot_coordinates[(knot as usize)].0;
                            let y_delta = knot_coordinates[((knot - 1) as usize)].1
                                - knot_coordinates[(knot as usize)].1;
                            if knot_coordinates[((knot - 1) as usize)].0
                                == knot_coordinates[(knot as usize)].0
                            {
                                if y_delta == 2 {
                                    knot_coordinates[knot as usize].1 += 1;
                                } else if y_delta == -2 {
                                    knot_coordinates[knot as usize].1 -= 1;
                                }
                            } else if knot_coordinates[((knot - 1) as usize)].1
                                == knot_coordinates[knot as usize].1
                            {
                                if x_delta == 2 {
                                    knot_coordinates[knot as usize].0 += 1;
                                } else if x_delta == -2 {
                                    knot_coordinates[knot as usize].0 -= 1;
                                }
                            } else if !touching(
                                knot_coordinates[((knot - 1) as usize)],
                                knot_coordinates[(knot as usize)],
                            ) {
                                let x_delta = knot_coordinates[((knot - 1) as usize)].0
                                    - knot_coordinates[(knot as usize)].0;
                                let y_delta = knot_coordinates[((knot - 1) as usize)].1
                                    - knot_coordinates[(knot as usize)].1;
                                knot_coordinates[(knot as usize)].0 += x_delta.div(x_delta.abs());
                                knot_coordinates[(knot as usize)].1 += y_delta.div(y_delta.abs());
                            }
                        }

                        tail_visited_coordinates.insert(format!(
                            "{},{}",
                            knot_coordinates[9].0, knot_coordinates[9].1
                        ));
                    }
                } else {
                    debug!("uh oh spaghettios");
                }
            }
        }
    }

    debug!("{}", tail_visited_coordinates.len());
}

fn touching(curr_knot_coordinate: (i32, i32), next_knot_coordinate: (i32, i32)) -> bool {
    (curr_knot_coordinate.0 - next_knot_coordinate.0).abs() <= 1
        && (curr_knot_coordinate.1 - next_knot_coordinate.1).abs() <= 1
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
