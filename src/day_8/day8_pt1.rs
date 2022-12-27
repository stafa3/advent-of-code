use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

use log::debug;

pub fn main() {
    if let Ok(lines) = read_lines("src/day_8/input.txt") {
        let trees: Vec<Vec<i32>> = populate_trees(lines);
        let mut visible_tree_count = 0;
        for (row_index, tree_row) in trees.iter().enumerate() {
            for (column_index, tree_height) in tree_row.iter().enumerate() {
                if row_index == 0
                    || column_index == 0
                    || row_index == trees.len() - 1
                    || column_index == tree_row.len() - 1
                {
                    visible_tree_count += 1;
                } else {
                    let mut column: Vec<i32> = trees.iter().map(|row| row[column_index]).collect();
                    let column_slices = column.split_at_mut(row_index);
                    let mut top_slice = column_slices.0.to_vec();
                    let mut bottom_slice = column_slices.1.to_vec();
                    let _ = bottom_slice.remove(0);

                    top_slice.sort();
                    bottom_slice.sort();

                    let row_slices = tree_row.split_at(column_index);
                    let mut left_slice = row_slices.0.to_vec();
                    let mut right_slice = row_slices.1.to_vec();
                    let _ = right_slice.remove(0);

                    left_slice.sort();
                    right_slice.sort();

                    if tree_height > &top_slice.last().unwrap()
                        || tree_height > &bottom_slice.last().unwrap()
                        || tree_height > &left_slice.last().unwrap()
                        || tree_height > &right_slice.last().unwrap()
                    {
                        visible_tree_count += 1;
                    }
                }
            }
        }
        debug!("visible_trees: {}", visible_tree_count);
    }
}

fn populate_trees(lines: Lines<BufReader<File>>) -> Vec<Vec<i32>> {
    let mut trees: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            let row: Vec<i32> = line
                .split("")
                .filter(|&x| !x.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            trees.push(row);
        }
    }

    trees
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
