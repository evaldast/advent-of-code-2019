use std::fs::File;
use std::io::{BufReader, Error, Lines, BufRead};

pub fn get_input(input_path: &str) -> Lines<BufReader<File>> {
    let file = File::open(input_path).expect("Failed opening file");

    BufReader::new(file).lines()
}

pub fn parse_int(line: &Result<String, Error>) -> Option<u32> {
    match line {
        Ok(l) => return Some(l.parse::<u32>().expect("Parse failed")),
        Err(_) => panic!("Input read failed"),
    };
}