use std::fs::File;
use std::io::{BufReader, Lines};

use advent_of_code_2019::utils;

pub fn calculate(lines: Lines<BufReader<File>>) -> String {
    let input: Vec<u32> = lines
        .collect::<Result<String, _>>()
        .unwrap()
        .split(',')
        .map(|item| item.parse::<u32>().expect("Failed parsing int value"))
        .collect();

    let opcode_chunks: Vec<&[u32]> = input.chunks(4).collect();

    println!("{:?}", opcode_chunks);

    "unfinished".to_owned()
}
