use std::fs::read_to_string;

struct Gift {
    l: u32,
    w: u32,
    h: u32,
}

fn problem_one(input: &Vec<Gift>) -> u32 {
    let mut total_paper: u32 = 0;
    for x in input {
        let mut equation_arr = [(x.l * x.w), (x.w * x.h), (x.h * x.l)];     // construct array of operations
        equation_arr.sort();                                                      // sort l,w,h
        total_paper += 2 * equation_arr.iter().sum::<u32>() + equation_arr[0];    // 2*l*w + 2*w*h + 2*h*l + min_area
    }
    return total_paper;
}

fn problem_two(input: &Vec<Gift>) -> u32 {
    let mut total_ribons: u32 = 0;
    for x in input {
        let mut sorted_sides = [x.l, x.w, x.h];
        sorted_sides.sort();
        total_ribons += (sorted_sides.iter().product::<u32>()) + (2 * sorted_sides[0] + 2 * sorted_sides[1]);
    }
    return total_ribons;
}

fn parse_string(input: &String) -> Vec<Gift> {
    let mut gift_vector: Vec<Gift> = Vec::new();
    for line in input.lines() {
        let mut val_iter = line.trim().split('x').map(|val| val.parse::<u32>().unwrap());
        let new_gift = Gift {
            l: val_iter.next().unwrap(),
            w: val_iter.next().unwrap(),
            h: val_iter.next().unwrap(),
        };
        gift_vector.push(new_gift);
    }
    return gift_vector;
}

fn main() {
    let input_data = parse_string(&read_to_string("input/day02.input").unwrap());
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
