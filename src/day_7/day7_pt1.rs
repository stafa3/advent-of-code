use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

use log::debug;

struct FileNode {
    name: String,
    children: Vec<FileNode>,
    size: i32,
    is_directory: bool,
}

use std::fmt;

impl fmt::Display for FileNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        self.fmt_helper(&mut output, 0);
        write!(f, "{}", output)
    }
}

impl FileNode {
    fn fmt_helper(&self, output: &mut String, indent_level: usize) {
        output.push_str(&" ".repeat(indent_level * 4));
        output.push_str(&format!("{} {}\n", self.size, self.name));

        for child in &self.children {
            child.fmt_helper(output, indent_level + 1);
        }
    }
}

pub fn main() {
    if let Ok(lines) = read_lines("src/day_7/input.txt") {
        let mut dir_tree: FileNode = populate_dir_tree(lines);

        update_directory_sizes(&mut dir_tree);

        let sum_of_directories_under_10000 = get_sum_of_directories_under_10000(&mut dir_tree);

        debug!("{}", sum_of_directories_under_10000);
    }
}

fn get_sum_of_directories_under_10000(node: &mut FileNode) -> i32 {
    let mut total_size = 0;
    if node.is_directory && node.size < 100000 {
        total_size += node.size;
    }
    for child in node.children.iter_mut() {
        total_size += get_sum_of_directories_under_10000(child);
    }
    total_size
}

fn update_directory_sizes(node: &mut FileNode) {
    let mut size = node.size;
    for child in node.children.iter_mut() {
        update_directory_sizes(child);
        size += child.size;
    }
    node.size = size;
}

fn populate_dir_tree(mut lines: Lines<BufReader<File>>) -> FileNode {
    let mut curr_dir: Vec<String> = vec![];
    let mut dir_tree = FileNode {
        name: String::from("/"),
        children: Vec::new(),
        size: 0,
        is_directory: true,
    };
    while let Some(line) = lines.next() {
        let line = line.expect("Failed to read line");

        if line.starts_with("$") {
            let words: Vec<&str> = line.split_whitespace().collect();
            match words[1] {
                "cd" => match words[2] {
                    ".." => {
                        curr_dir.pop();
                    }
                    _ => {
                        curr_dir.push(words[2].to_owned());
                    }
                },
                _ => {}
            }
        } else if line.starts_with("dir") {
            let new_dir = line.split_whitespace().collect::<Vec<&str>>()[1];
            let mut curr_tree = &mut dir_tree;

            for dir in &curr_dir {
                let dir_node_index = curr_tree
                    .children
                    .iter()
                    .enumerate()
                    .find(|(_, node)| node.name.as_str() == dir)
                    .map(|(i, _)| i);
                match dir_node_index {
                    Some(index) => curr_tree = &mut curr_tree.children[index],
                    None => {}
                }
            }

            curr_tree.children.push(FileNode {
                name: new_dir.to_owned(),
                children: Vec::new(),
                size: 0,
                is_directory: true,
            });
        } else if line.split_whitespace().collect::<Vec<&str>>()[0]
            .chars()
            .next()
            .unwrap()
            .is_numeric()
        {
            let file_size = line.split_whitespace().collect::<Vec<&str>>()[0];
            let file_name = line.split_whitespace().collect::<Vec<&str>>()[1];
            let mut curr_tree = &mut dir_tree;

            for dir in &curr_dir {
                let dir_node_index = curr_tree
                    .children
                    .iter()
                    .enumerate()
                    .find(|(_, node)| node.name.as_str() == dir)
                    .map(|(i, _)| i);
                match dir_node_index {
                    Some(index) => curr_tree = &mut curr_tree.children[index],
                    None => {}
                }
            }

            curr_tree.children.push(FileNode {
                name: file_name.to_owned(),
                children: Vec::new(),
                size: file_size.parse().unwrap(),
                is_directory: false,
            });
        } else {
        }
    }
    dir_tree
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
