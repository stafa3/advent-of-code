use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use log::debug;

struct Monkey<F>
where
    F: Fn(i32, String) -> i32,
{
    id: i32,
    items: Vec<i32>,
    true_throw_monkey_id: i32,
    false_throw_monkey_id: i32,
    stress_modifier: String,
    divisible_value: i32,
    perform_operation: F,
}

impl<F> Monkey<F>
where
    F: Fn(i32, String) -> i32,
{
    fn new(
        id: i32,
        items: Vec<i32>,
        true_throw_monkey_id: i32,
        false_throw_monkey_id: i32,
        stress_modifier: String,
        divisible_value: i32,
        perform_operation: F,
    ) -> Monkey<F> {
        Monkey {
            id,
            items,
            true_throw_monkey_id,
            false_throw_monkey_id,
            stress_modifier,
            divisible_value,
            perform_operation,
        }
    }
}

pub fn main() {
    if let Ok(lines_object) = read_lines("src/day_11/input.txt") {
        let mut monkeys: Vec<Monkey<_>> = Vec::new();
        //iterate through all Monkeys and initialize them
        let lines: Vec<String> = lines_object.collect::<Result<_, _>>().unwrap();
        populate_monkeys(lines, &mut monkeys);

        let mut num_times_monkey_inspected = vec![0; monkeys.len()];
        for _ in 0..20 {
            for monkey_index in 0..monkeys.len() {
                let mut item_len = monkeys[monkey_index].items.len();
                while item_len > 0 {
                    let mut monkey_id_to_throw = 0;
                    let mut worry_level = 0;
                    {
                        let monkey = &monkeys[monkey_index];
                        let item = monkey.items.first().unwrap();
                        num_times_monkey_inspected[monkey_index] += 1;
                        worry_level =
                            (monkey.perform_operation)(*item, monkey.stress_modifier.clone());
                        worry_level = worry_level / 3;
                        if worry_level % monkey.divisible_value == 0 {
                            monkey_id_to_throw = monkey.true_throw_monkey_id;
                        } else {
                            monkey_id_to_throw = monkey.false_throw_monkey_id;
                        }
                    }
                    {
                        let monkey_to_throw = &mut monkeys[monkey_id_to_throw as usize];
                        monkey_to_throw.items.push(worry_level);
                    }
                    {
                        let monkey = &mut monkeys[monkey_index];
                        monkey.items.remove(0);
                    }
                    item_len -= 1;
                }
            }
            for monkey in &monkeys {
                debug!("Monkey {}: {:?}", monkey.id, monkey.items);
            }
            debug!("\n");
            debug!("{:?}", num_times_monkey_inspected);
        }
        num_times_monkey_inspected.sort();
        debug!(
            "{}",
            num_times_monkey_inspected[monkeys.len() - 2]
                * num_times_monkey_inspected[monkeys.len() - 1]
        );
    }
}

fn populate_monkeys(lines: Vec<String>, monkeys: &mut Vec<Monkey<fn(i32, String) -> i32>>) {
    let mut line_index = 0;
    while line_index < lines.len() {
        let monkey_line = &lines[line_index];
        let starting_items_line = &lines[line_index + 1];
        let operation_line = &lines[line_index + 2];
        let test_line = &lines[line_index + 3];
        let true_condition_line = &lines[line_index + 4];
        let false_condition_line = &lines[line_index + 5];

        //populate monkey id
        let monkey_id: i32 = monkey_line.split_whitespace().collect::<Vec<&str>>()[1]
            .chars()
            .collect::<Vec<char>>()[0]
            .to_string()
            .parse()
            .unwrap();
        //populate starting items
        let mut starting_items: Vec<i32> = Vec::new();
        {
            let mut starting_items_split = starting_items_line
                .split_whitespace()
                .collect::<Vec<&str>>();
            let starting_items_strings = starting_items_split.split_off(2);
            let modified_strings: Vec<String> = starting_items_strings
                .into_iter()
                .map(|s| s.replace(",", ""))
                .collect();
            for s in modified_strings {
                starting_items.push(s.parse().unwrap());
            }
        }
        //populate perform_operation
        let operation_split = operation_line.split_whitespace().collect::<Vec<&str>>();
        let operator_last_word = operation_split.last().unwrap();
        let operator = operation_split[operation_split.len() - 2];
        let perform_operation = match_operator_to_operation(operator);

        //get numbers
        let divisible_value = get_last_char(test_line);
        let true_throwing_monkey: i32 = get_last_char(true_condition_line);
        let false_throwing_monkey: i32 = get_last_char(false_condition_line);

        let monkey = Monkey::new(
            monkey_id,
            starting_items,
            true_throwing_monkey,
            false_throwing_monkey,
            operator_last_word.to_string(),
            divisible_value,
            perform_operation,
        );

        monkeys.push(monkey);

        line_index += 7
    }
}

fn match_operator_to_operation(operator: &str) -> fn(i32, String) -> i32 {
    let perform_operation: fn(i32, String) -> i32 = match operator {
        "+" => {
            fn temp(old: i32, operation_last_word: String) -> i32 {
                if operation_last_word == "old" {
                    old + old
                } else {
                    let val: i32 = operation_last_word.parse().unwrap();
                    old + val
                }
            }
            temp
        }
        "-" => {
            fn temp(old: i32, operation_last_word: String) -> i32 {
                if operation_last_word == "old" {
                    old - old
                } else {
                    let val: i32 = operation_last_word.parse().unwrap();
                    old - val
                }
            }
            temp
        }
        "*" => {
            fn temp(old: i32, operation_last_word: String) -> i32 {
                if operation_last_word == "old" {
                    old * old
                } else {
                    let val: i32 = operation_last_word.parse().unwrap();
                    old * val
                }
            }
            temp
        }
        "/" => {
            fn temp(old: i32, operation_last_word: String) -> i32 {
                if operation_last_word == "old" {
                    old / old
                } else {
                    let val: i32 = operation_last_word.parse().unwrap();
                    old / val
                }
            }
            temp
        }
        _ => {
            debug!("holy shit idk {}", operator);
            fn temp(_: i32, _: String) -> i32 {
                0
            }
            temp
        }
    };
    perform_operation
}

fn get_last_char(test_line: &String) -> i32 {
    let divisible_value: i32 = test_line
        .split_whitespace()
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    divisible_value
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
