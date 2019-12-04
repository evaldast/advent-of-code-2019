use std::fs::File;
use std::io::{BufReader, Lines};

use advent_of_code_2019::utils;

pub fn calculate(lines: Lines<BufReader<File>>) -> String {
    lines
        .filter_map(|line| utils::parse_uint(&line))
        .fold(0u32, |sum, val| sum + val / 3 - 2)
        .to_string()
}
