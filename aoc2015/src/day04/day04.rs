use std::{fs::read_to_string};
use md5::{self, Md5, Digest};

fn problem_one(input: &String) -> usize {
    let mut hasher = Md5::new();
    let mut append_num: usize = 0;
    loop {
        hasher.update(format!("{}{}", input, append_num.to_string()));
        let hash: String = format!("{:02X}", hasher.finalize_reset());
        let hash_five_most_sig_bits = &hash[..5];
        if hash_five_most_sig_bits.eq("00000") {
            break;
        }
        append_num += 1;
    }
    return append_num;
}

fn problem_two(input: &String) -> usize {
    let mut hasher = Md5::new();
    let mut append_num: usize = 0;
    loop {
        hasher.update(format!("{}{}", input, append_num.to_string()));
        let hash: String = format!("{:02X}", hasher.finalize_reset());
        let hash_five_most_sig_bits = &hash[..6];
        if hash_five_most_sig_bits.eq("000000") {
            break;
        }
        append_num += 1;
    }
    return append_num;
}

fn main() {
    let input_data = read_to_string("input/day04.input").unwrap();
    println!("\nProblem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
