use std::fs::File;
use std::io::{BufReader, Lines};

use crate::day_2;
use advent_of_code_2019::utils;

pub fn calculate(lines: Lines<BufReader<File>>) -> String {
    let input = utils::parse_lines_with_seperator::<u32>(lines, ',');

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input_clone = input.to_vec();

            input_clone[1] = noun;
            input_clone[2] = verb;

            if (day_2::task_1::process_input(&mut input_clone)) == 19690720u32 {
                return (100 * noun + verb).to_string();
            }
        }
    }

    "Fail".to_owned()
}
