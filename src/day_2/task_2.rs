
use std::fs::File;
use std::io::{BufReader, Lines};

use crate::day_2;

pub fn calculate(lines: Lines<BufReader<File>>) -> String {
    let input: Vec<u32> = lines
        .collect::<Result<String, _>>()
        .unwrap()
        .split(',')
        .map(|item| item.parse::<u32>().expect("Failed parsing int value"))
        .collect();

    for verb in 0..=99 {
        for noun in 0..=99 {
            let mut input_clone = input.to_vec();
            
            input_clone[1] = verb;
            input_clone[2] = noun;

            if (day_2::task_1::process_input(&mut input_clone)) == 19690720u32 {
                return (100 * (noun + verb)).to_string()
            }
        }
    }
    
    "Fail".to_owned()
}
