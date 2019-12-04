use std::fs::File;
use std::io::{BufReader, Lines};

use advent_of_code_2019::utils;

pub fn calculate(lines: Lines<BufReader<File>>) -> String {
    lines
        .filter_map(|line| utils::parse_uint(&line))
        .fold(0u32, |sum, val| sum + calculate_fuel_recursively(calculate_fuel(val)))
        .to_string()
}

fn calculate_fuel(module_mass: u32) -> u32 {
    match (module_mass / 3).checked_sub(2) {
        Some(x) => x,
        None => 0
    }
}

fn calculate_fuel_recursively(module_mass: u32) -> u32 {
    if module_mass == 0 {
        return 0
    }

    module_mass + calculate_fuel_recursively(calculate_fuel(module_mass))
}
