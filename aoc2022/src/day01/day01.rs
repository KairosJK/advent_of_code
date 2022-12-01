use std::{fs::{read_to_string}};

fn problem_one(input: &String) -> usize {
    let mut higher_count: usize = 0;
    let mut current_count: usize= 0;
    for x in input.lines() {
        if x.is_empty() {
            if current_count > higher_count {
                higher_count = current_count;
            }
            current_count = 0;
        } else {
            current_count += x.parse::<usize>().unwrap();
        }
    }
    higher_count
}

fn problem_two(input: &String) -> usize {
    let mut vector: Vec<usize> = Vec::new();
    let mut current_count: usize = 0;
    for x in input.lines() {
        if x.is_empty() {
            vector.push(current_count);
            current_count = 0;
        } else {
            current_count += x.parse::<usize>().unwrap();
        }
    }
    vector.sort_by(|a, b| b.cmp(a));
    vector[0] + vector[1] + vector[2]
}

fn main() {
    let input_data = read_to_string("input/day01.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
