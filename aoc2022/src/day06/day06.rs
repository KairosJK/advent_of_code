use std::fs::read_to_string;
use itertools::Itertools;

fn universal_solve(input: &String, delim: usize) -> usize {
    let string_bytes: Vec<u8> = input.chars().map(|c| c as u8).collect();
    for i in 0..string_bytes.len()-delim {
        let substring = string_bytes[i..i+delim].to_vec();
        if  substring.iter().unique().collect::<Vec<&u8>>().len() == substring.len() {
            return i+delim;
        }
    }
    input.len()
}

fn problem_one(input: &String) -> usize {
    universal_solve(input, 4)
}

fn problem_two(input: &String) -> usize {
    universal_solve(input, 14)
}

fn main() {
    let input_data = read_to_string("input/day06.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data)); // 1912
    println!("Problem 2: {}", problem_two(&input_data)); // 2122
}
