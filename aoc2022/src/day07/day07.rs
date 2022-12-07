use std::{fs::read_to_string, collections::{HashMap, btree_map::Values}};

fn universal_solve(input: &String) -> HashMap<String, usize>{
    let history: Vec<_> = input
                        .lines()
                        .map(|x| x.split(" ").collect::<Vec<&str>>())
                        .collect();
    let mut filesystem: HashMap<String, usize> = HashMap::new();
    let mut filepath_stack: Vec<String> = Vec::new();
    let mut curr_filepath: String = String::new();
    for cmd in history {
        match cmd[0] {
            "$" => {
                match cmd.get(2) {
                    Some(&"/") => {
                        curr_filepath = "/root".to_string();
                        filesystem.entry(curr_filepath.clone()).or_insert(0);
                        filepath_stack.clear();
                        filepath_stack.push(curr_filepath.clone());
                    },
                    Some(&"..") => {
                        let mut new_filepath = curr_filepath.split("/")
                                                                       .filter(|str| !str.is_empty())
                                                                       .collect::<Vec<&str>>();
                        new_filepath.pop();
                        curr_filepath = new_filepath.iter().map(|str| format!("/{}", str)).collect::<String>();
                        filepath_stack.pop();
                    },
                    Some(_) => {
                        curr_filepath.push_str(format!("/{}", cmd[2]).as_str());
                        filepath_stack.push(curr_filepath.clone());
                        filesystem.insert(filepath_stack.last().unwrap().clone(), 0);
                    },
                    None => (),
                }
            },
            _ => {
                if cmd[0].chars().all(|c| c.is_ascii_digit()) {
                    for x in filepath_stack.clone() {
                        filesystem.insert(
                            x.clone(), 
                            filesystem.get(&x).expect(format!("{} {:?}", curr_filepath, filepath_stack).as_str()) + cmd[0].parse::<usize>().unwrap()
                        );
                    }
                }
            },
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
    let target = filesystem.get("/root").unwrap()-40_000_000;
    let mut closest = 0;
    for x in filesystem.values().collect::<Vec<&usize>>() {
        if target.abs_diff(*x) < target.abs_diff(closest) { closest = *x}
    }
    closest
}

fn main() {
    let input_data = read_to_string("input/day07.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
