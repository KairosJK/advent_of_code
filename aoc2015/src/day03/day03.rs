use std::{fs::read_to_string, collections::HashMap};

fn add_tuples(tuple1: &mut(i32, i32), tuple2: (i32, i32)) {
    tuple1.0 += tuple2.0;
    tuple1.1 += tuple2.1;
}

fn problem_one(input: &String) -> usize {
    let mut house_hash: HashMap<(i32, i32),i32>= HashMap::new();
    house_hash.insert((0, 0), 1);
    let mut current_location: (i32, i32) = (0, 0);
    for char in input.chars() {
        match char {
            '>' => add_tuples(&mut(current_location), (1, 0)),
            '<' => add_tuples(&mut(current_location), (-1, 0)),
            '^' => add_tuples(&mut(current_location), (0, 1)),
            'v' => add_tuples(&mut(current_location), (0, -1)),
            _   => (),
        }
        *house_hash.entry(current_location).or_insert(0) += 1;
    }
    return house_hash.values().filter(|&&x| x >= 1).count();
}

fn problem_two(input: &String) -> usize {
    let mut house_hash: HashMap<(i32, i32),i32>= HashMap::new();
    house_hash.insert((0, 0), 2);
    let mut santa_location: (i32, i32) = (0, 0);
    let mut robo_santa_location: (i32, i32) = (0, 0);
    for (i, char) in input.chars().enumerate() {
        let current_location: &mut(i32, i32);
        if i  % 2 == 0 {
            current_location = &mut(santa_location);
        } else {
            current_location = &mut(robo_santa_location);
        }
        match char {
            '>' => add_tuples(current_location, (1, 0)),
            '<' => add_tuples(current_location, (-1, 0)),
            '^' => add_tuples(current_location, (0, 1)),
            'v' => add_tuples(current_location, (0, -1)),
            _   => (),
        }
        *house_hash.entry(*current_location).or_insert(0) += 1;
    }
    return house_hash.values().filter(|&&x| x >= 1).count();
}

fn main() {
    let input_data = read_to_string("input/day03.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
