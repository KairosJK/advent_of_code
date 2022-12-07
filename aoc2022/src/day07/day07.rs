use std::{fs::read_to_string, collections::HashMap};

fn universal_solve(input: &String) -> HashMap<String, usize>{
    let history: Vec<_> = input
                        .lines()
                        .map(|x| x.split(" ").collect::<Vec<&str>>())
                        .collect();
    let mut filesystem: HashMap<String, usize> = HashMap::new();
    let mut filepath_stack: Vec<String> = Vec::new();
    for cmd in history {
        if cmd[0] == "$" {
            match cmd.get(2) {
                Some(&"..") => { filepath_stack.pop(); },
                Some(&"/") => {
                    filepath_stack.clear();
                    filepath_stack.push("/root".to_string());
                    filesystem.entry(filepath_stack.last().unwrap().clone()).or_insert(0);
                },
                Some(_) => {
                    filepath_stack.push(format!("{}/{}", filepath_stack.last().unwrap().clone(), cmd[2]));
                    filesystem.insert(filepath_stack.last().unwrap().clone(), 0);
                },
                None => (),
            }
        } else {
            if cmd[0].chars().all(|c| c.is_ascii_digit()) {
                for x in filepath_stack.clone() {
                    filesystem.insert(x.clone(), filesystem.get(&x).unwrap() + cmd[0].parse::<usize>().unwrap());
                }
            }
        }
    }
    filesystem
}
    

fn problem_one(input: &String) -> usize {
    let filesystem = universal_solve(input); 
    filesystem.iter()
              .filter(|(x, y)| (**x) != "/root".to_string() && **y <= 100000)
              .map(|(_, y)| y)
              .sum()
}

fn problem_two(input: &String) -> usize {
    let filesystem = universal_solve(input);
    let target = filesystem.get("/root").unwrap() - 40_000_000;
    let mut closest = 0;
    for x in filesystem.values().collect::<Vec<&usize>>() {
        if target.abs_diff(*x) < target.abs_diff(closest) { closest = *x }
    }
    closest
}

fn main() {
    /*
    Problem 1: 1501149
    Problem 2: 10096985
    */
    let input_data = read_to_string("input/day07.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
