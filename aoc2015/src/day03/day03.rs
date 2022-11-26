use std::{fs::read_to_string, collections::HashMap};

fn add_tuples(tuple1: &mut(i32, i32), tuple2: (i32, i32)) {
    tuple1.0 += tuple2.0;
    tuple1.1 += tuple2.1;
}

fn problem_one(input: &String) -> i32 {
    let mut house_hash: HashMap<(i32, i32),i32>= HashMap::new();
    let mut current_location: (i32, i32) = (0, 0);
    for char in input.chars() {
        match char {
            '>' => add_tuples(&mut(current_location), (1, 0)),
            '<' => add_tuples(&mut(current_location), (-1, 0)),
            '^' => add_tuples(&mut(current_location), (0, 1)),
            'v' => add_tuples(&mut(current_location), (0, -1)),
            _   => (),
        }
        let house_count = house_hash.entry(current_location).or_insert(1);
        *house_count += 1;
    }
    return 0;
}

/* fn problem_two() {

} */

fn main() {
    let input_data = read_to_string("input/day03.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
   /* println!("Problem 2: {}", problem_two(input_data)); */
}
