use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};
use std::str;

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

pub fn parse_lines_with_seperator<T>(lines: Lines<BufReader<File>>, seperator: char) -> Vec<T>
where
    T: str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    lines
        .collect::<Result<String, _>>()
        .unwrap()
        .split(seperator)
        .map(|item| item.parse::<T>().expect("Failed parsing int value"))
        .collect()
}
