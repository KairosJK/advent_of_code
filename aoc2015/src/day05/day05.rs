use std::{fs::read_to_string};

fn problem_one(input: &String) -> usize {
    input.lines().filter(|x| has_vowel(x) && has_double(x) && !has_denied_patterns(x)).count()
}

fn has_vowel(input: &str) -> bool {
    let vowels = ['a','e','i','o','u'];
    input.chars().filter(|x| vowels.contains(x)).count() >= 3
}

fn has_denied_patterns(input: &str) -> bool {
    let denied_patterns = ["ab", "cd", "pq", "xy"];
    for x in denied_patterns {
        if input.contains(x) {
            return true;
        }
    }
    false
}

fn has_double(input: &str) -> bool {
    for i in 1..input.len() {
        if input.chars().nth(i).eq(&input.chars().nth(i-1)) {
            return true;
        }
    }
    false
}

fn problem_two(input: &String) -> usize {
    input.lines().filter(|x| has_skipped_repeat(x) && has_pair_no_overlap(x)).count()
}

fn has_skipped_repeat(input: &str) -> bool {
    for i in 2..input.len() {
        if input.chars().nth(i).eq(&input.chars().nth(i-2)) {
            return true;
        }
    }
    false
}

fn has_pair_no_overlap(input: &str) -> bool {
    false
}


fn main() {
    let input_data = read_to_string("input/day05.input").unwrap();
    println!("\nProblem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
