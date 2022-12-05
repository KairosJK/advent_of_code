use std::{fs::read_to_string};

fn problem_one(input: &String) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let delim_str: Vec<&str> = line.split(|c| c == '-' || c == ',').collect();
        let v: Vec<usize> = delim_str.iter()
                                    .map(|digit| digit.parse::<usize>().unwrap())
                                    .collect();  
        if v[0] <= v[2] && v[1] >= v[3] {
            count += 1;
        } else if v[2] <= v[0] && v[3] >= v[1] {
            count += 1;
        }
    }
    return count;
}

fn problem_two(input: &String) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let delim_str: Vec<&str> = line
                                    .split(|c| c == '-' || c == ',')
                                    .collect();
        let v: Vec<usize> = delim_str
                                .iter()
                                .map(|digit| digit.parse::<usize>().unwrap())
                                .collect();
        if (v[0] >= v[2] && v[0] <= v[3]) || (v[1] >= v[2] && v[1] <= v[3]) {
            count += 1;
        } else if (v[2] >= v[0] && v[2] <= v[1]) || (v[3] >= v[0] && v[3] <= v[1]) {
            count += 1;
        }
    }
    return count;
}

fn main() {
    let input_data = read_to_string("input/day04.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
