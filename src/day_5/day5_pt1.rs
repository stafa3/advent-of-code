use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    if let Ok(lines) = read_lines("src/day_5/input.txt") {
        // Parse just the crates first, put them in a data structure
        let mut vec_of_crate_stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
        for line in lines {
            if let Ok(line) = line {
                if line.contains("[") {
                    let mut crate_stack_index_in_vec: usize = 0;
                    for crate_index_in_line in (1..(line.len() as u32)).step_by(4) {
                        let crate_to_insert = line
                            .to_string()
                            .chars()
                            .nth(crate_index_in_line as usize)
                            .unwrap();

                        if crate_to_insert == ' ' {
                            crate_stack_index_in_vec = crate_stack_index_in_vec + 1;
                            continue;
                        }
                        if vec_of_crate_stacks.get(crate_stack_index_in_vec).is_none() {
                            let mut crate_stack: Vec<char> = Vec::new();
                            crate_stack.insert(0, crate_to_insert);
                            vec_of_crate_stacks.insert(crate_index_in_line as usize, crate_stack);
                            crate_stack_index_in_vec = crate_stack_index_in_vec + 1;
                        } else {
                            let crate_stack = &mut vec_of_crate_stacks[crate_stack_index_in_vec];
                            crate_stack.insert(0, crate_to_insert);
                            crate_stack_index_in_vec = crate_stack_index_in_vec + 1;
                        }
                    }
                } else if line.contains("m") {
                    let split_line: Vec<&str> = line.split_whitespace().collect();
                    let num_to_move: u32 = split_line[1].parse().unwrap();
                    let from_crate: usize = split_line[3].parse().unwrap();
                    let to_crate: usize = split_line[5].parse().unwrap();

                    let mutable_vec_of_crate_stacks = &mut vec_of_crate_stacks;
                    let mut to_push = '0';
                    for _i in 0..num_to_move {
                        if let Some(from_crate_stack) =
                            mutable_vec_of_crate_stacks.get_mut(from_crate - 1)
                        {
                            to_push = from_crate_stack.pop().unwrap()
                        }
                        if let Some(to_crate_stack) =
                            mutable_vec_of_crate_stacks.get_mut(to_crate - 1)
                        {
                            to_crate_stack.push(to_push);
                        }
                    }
                }
            }
        }

        let mut final_value = String::new();
        for crate_vec in vec_of_crate_stacks.iter() {
            final_value.push(*crate_vec.last().unwrap());
        }
        log::debug!("final_value: {}", final_value);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
