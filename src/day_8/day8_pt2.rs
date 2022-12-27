use std::cmp;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

use log::debug;

pub fn main() {
    if let Ok(lines) = read_lines("src/day_8/input.txt") {
        let trees: Vec<Vec<i32>> = populate_trees(lines);
        let mut max_scenic_score = 0;
        for (row_index, tree_row) in trees.iter().enumerate() {
            for (column_index, tree_height) in tree_row.iter().enumerate() {
                if row_index == 0
                    || column_index == 0
                    || row_index == trees.len() - 1
                    || column_index == tree_row.len() - 1
                {
                    continue;
                } else {
                    let mut column: Vec<i32> = trees.iter().map(|row| row[column_index]).collect();
                    let column_slices = column.split_at_mut(row_index);
                    let mut top_slice = column_slices.0.to_vec();
                    top_slice.reverse();
                    let mut bottom_slice = column_slices.1.to_vec();
                    let _ = bottom_slice.remove(0);
                    let row_slices = tree_row.split_at(column_index);
                    let mut left_slice = row_slices.0.to_vec();
                    left_slice.reverse();
                    let mut right_slice = row_slices.1.to_vec();
                    let _ = right_slice.remove(0);

                    let mut scenic_score = 1;

                    for view in [top_slice, bottom_slice, left_slice, right_slice] {
                        let mut num_viewable_trees = 0;
                        for tree_in_view_height in view {
                            if tree_height <= &tree_in_view_height {
                                num_viewable_trees += 1;
                                break;
                            } else if tree_height > &tree_in_view_height {
                                num_viewable_trees += 1;
                            }
                        }
                        scenic_score *= num_viewable_trees;
                    }

                    max_scenic_score = cmp::max(max_scenic_score, scenic_score);
                }
            }
        }
        debug!("max score: {}", max_scenic_score);
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
