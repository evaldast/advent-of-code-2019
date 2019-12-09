use std::fs::File;
use std::io::{BufReader, Lines};

pub fn calculate(lines: Lines<BufReader<File>>) -> String {
    let mut input: Vec<u32> = lines
        .collect::<Result<String, _>>()
        .unwrap()
        .split(',')
        .map(|item| item.parse::<u32>().expect("Failed parsing int value"))
        .collect();

    initiate_gravity_restore_assist(&mut input);    
    let result = process_input(&mut input);    

    result.to_string()
}

fn process_input(input: &mut [u32]) -> u32 {    
    for index in (0..input.len()).step_by(4) {        
        let operation: u32 = input[index];
        let op_indices = get_opcode_indices(index, input);

        match operation {
            1 => add(op_indices, input),
            2 => multiply(op_indices, input),
            99 => return input[0],                
            _ => panic!("Undefined opcode")
        }
    }

    0
}

fn add(indices: (usize, usize, usize), input: &mut [u32]) {
    input[indices.0] = input[indices.1] + input[indices.2]
}

fn multiply(indices: (usize, usize, usize), input: &mut [u32]) {    
    input[indices.0] = input[indices.1] * input[indices.2]
}

fn initiate_gravity_restore_assist(input: &mut [u32]) {
    input[1] = 12;
    input[2] = 2;
}

fn get_opcode_indices(index: usize, input: &[u32]) -> (usize, usize, usize) {
    let result_index = input[index + 3] as usize;
    let number_one_index = input[index + 1] as usize;
    let number_two_index = input[index + 2] as usize;

    (result_index, number_one_index, number_two_index)
}
