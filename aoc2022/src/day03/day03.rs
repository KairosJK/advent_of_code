use std::fs::read_to_string;

fn problem_one(input: &String) -> usize {
    let mut count: usize = 0;
    for line in input.lines() {
        let vec: Vec<u8> = line.chars().map(|c| {
            if c.is_ascii_lowercase() {
                c as u8 - 96
            } else {
                c as u8 - 38
            }
        }).collect();
        let half_of_vector: usize = vec.len()/2;
        for i in 0..half_of_vector {
            if vec[(half_of_vector)..vec.len()].contains(&vec[i]) {
                count += vec[i] as usize;
                break;
            }
        }
    }
    return count;
}

fn problem_two(input: &String) -> usize {
    let mut count: usize = 0;
    let lines: Vec<&str> = input.split("\n").collect();
    let mut x = 0; 
    while x < lines.len()-2 {
        for c in lines[x].chars() {
            if lines[x+1].contains(c) && lines[x+2].contains(c) {
                if c.is_ascii_lowercase() {
                    count +=  c as usize - 96;
                    break;
                } else {
                    count +=  c as usize - 38;
                    break;
                }
            }
        }
        x = x + 3;
    }
    return count;
}

fn main() {
    let input_data = read_to_string("input/day03.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
