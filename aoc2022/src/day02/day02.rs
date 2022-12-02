use std::{fs::read_to_string};

fn problem_one(input: &String) -> usize {
    let mut count = 0;
    for l in input.lines() {
        let c: Vec<&str> = l.split(" ").collect();
        match c[0] {
            "A" => {
                match c[1] {
                    "X" => count += 1 + 3,
                    "Y" => count += 2 + 6,
                    "Z" => count += 3,
                    _ => (),
                }
            },
            "B" => {
                match c[1] {
                    "X" => count += 1,
                    "Y" => count += 2 + 3,
                    "Z" => count += 3 + 6,
                    _ => (),
                }
            },
            "C" => {
                match c[1] {
                    "X" => count += 1 + 6,
                    "Y" => count += 2,
                    "Z" => count += 3 + 3,
                    _ => (),
                }
            }
            _ => (),
        }
    }
    return count;
}

fn problem_two(input: &String) -> usize {
    let mut count = 0;
    for l in input.lines() {
        let c: Vec<&str> = l.split(" ").collect();
        match c[0] {
            "A" => {
                match c[1] {
                    "X" => count += 3 + 0,
                    "Y" => count += 1 + 3,
                    "Z" => count += 2 + 6,
                    _ => (),
                }
            },
            "B" => {
                match c[1] {
                    "X" => count += 1,
                    "Y" => count += 2 + 3,
                    "Z" => count += 3 + 6,
                    _ => (),
                }
            },
            "C" => {
                match c[1] {
                    "X" => count += 2,
                    "Y" => count += 3 + 3,
                    "Z" => count += 1 + 6,
                    _ => (),
                }
            }
            _ => (),
        }
    }
    return count;
}

fn main() {
    let input_data = read_to_string("input/day02.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
