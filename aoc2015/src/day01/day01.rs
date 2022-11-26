use std::{fs::{read_to_string}};

fn problem_one(input: &String) -> i32 {
    let mut count: i32 = 0;
    for char in input.chars() {
        match char {
            '(' => count = count + 1,   // go up a floor
            ')' => count = count - 1,   // go down a floor
            _ => {},                    // do nothing
        }
        
    }
    return count;
}

fn problem_two(input: &String) -> i32 {
    let mut count: i32 = 0;
    for (index, char) in input.chars().enumerate() {
        match char {
            '(' => count = count + 1,   // go up a floor
            ')' => count = count - 1,   // go down a floor
            _ => {},                    // do nothing
        }
        if count == -1 {
            return (index as i32) + 1;
        }
    }
    return 0;
}

fn main() {
    let input_data = read_to_string("input/day01.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
