use std::fs::read_to_string;

fn problem_one(input: &String) -> String {
    let mut v = get_crate_stacks(input);
    for line in input.lines() {
        let delim_str: Vec<&str> = line.split(" ").collect();
        if delim_str[0] == "move" {
            let mv = get_operations(delim_str);
            for _x in 0..mv[0] {
                let j = v[mv[1]-1].pop().unwrap();
                v[mv[2]-1].push(j);
            }
        }
    }
    let mut end_str = String::new();
    for x in 0..v.len() {
        if !v[x].is_empty() {
            end_str.push(v[x].pop().unwrap());
        }
    }
    return end_str;
}

fn problem_two(input: &String) -> String {
    let mut v = get_crate_stacks(input);
    for line in input.lines() {
        let delim_str: Vec<&str> = line.split(" ").collect();
        if delim_str[0] == "move" {
            let mv = get_operations(delim_str);
            let mut buf_vec: Vec<char> = Vec::new();
            for _x in 0..mv[0] { buf_vec.push(v[mv[1]-1].pop().unwrap()); }
            buf_vec.reverse();
            v[mv[2]-1].append(&mut buf_vec);
        }
    }
    let mut end_str = String::new();
    for x in 0..v.len() {
        if !v[x].is_empty() {
            end_str.push(v[x].pop().unwrap());
        }
    }
    return end_str;
}

fn get_operations(delim_str: Vec<&str>) -> Vec<usize>{
    delim_str
        .iter()
        .filter(|str| str.chars().all(|c| c.is_ascii_digit()))
        .map(|digit| digit.parse::<usize>().unwrap())
        .collect()
}

fn get_crate_stacks(input: &String) -> Vec<Vec<char>> {
    let mut v = vec![Vec::new(); 15];
    for line in input.clone().lines() {
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_uppercase() {
                v[i/4].push(c);
            }
        }
    }
    for x in &mut v {
        x.reverse();
    }
    return v;
}

fn main() {
    let input_data = read_to_string("input/day05.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
