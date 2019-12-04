use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};

pub fn get_input(input_path: &str) -> Lines<BufReader<File>> {
    let file = File::open(input_path).expect("Failed opening file");

    BufReader::new(file).lines()
}

pub fn parse_uint(line: &Result<String, Error>) -> Option<u32> {
    match line {
        Ok(l) => Some(l.parse::<u32>().expect("Parse failed")),
        Err(_) => panic!("Input read failed"),
    }
}
