use std::fs::read_to_string;

fn problem_one(input: &String) -> usize {
    let vec: Vec<u8> = input.chars().map(|c| c as u8).collect();
    for i in 0..vec.len()-4 {
        let comb_vec = vec![vec[i], vec[i+1], vec[i+2], vec[i+3]];
        if get_c_a(comb_vec) {
            return i+4;
        }
    }
    return input.len();
}

fn get_c_a(comb_vec: Vec<u8>) -> bool {
    let length = comb_vec.len();
    for x in 0..length {
        for y in 0..length {
            if comb_vec[x] == comb_vec[y] && x != y {
                return false;
            }
        }
    }
    true
}

fn problem_two(input: &String) -> usize {
    let vec: Vec<u8> = input.chars().map(|c| c as u8).collect();
    for i in 0..vec.len()-13 {
        let comb_vec = vec[i..i+14].to_vec();
/*         for x in comb_vec.clone() {
            print!("{} ", x as char);
        }
        println!("\n\n"); */
        if get_c_a(comb_vec) {
            return i+14;
        }
    }
    return input.len();
}

fn main() {
    let input_data = read_to_string("input/day06.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
