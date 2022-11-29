use std::fs::read_to_string;

fn problem_one(input: &String) -> usize {
    let mut light_field = vec![vec![false; 1000]; 1000];
    for current_line in input.lines() {
        q1_mutate_lights(current_line, &mut light_field)
    }
    light_field.iter().flatten().filter(|x| **x).count()
}

fn problem_two(input: &String) -> usize {
    let mut light_field = vec![vec![0; 1000]; 1000];
    for current_line in input.lines() {
        q2_mutate_lights(current_line, &mut light_field)
    }
    light_field.iter().flatten().sum()
}

fn q1_mutate_lights(line: &str, light_field: &mut Vec<Vec<bool>>) {
    let delim_str: Vec<&str> = line.split(|c| c == ' ' || c == ',').collect();
    let coord_vector: Vec<usize> = delim_str.iter()
                                .filter(|x| x.chars().all(|c| c.is_ascii_digit()))
                                .map(|digit| digit.parse::<usize>().unwrap())
                                .collect();   
    if line.contains("toggle") {
        for x in coord_vector[0]..=coord_vector[2] {
            for y in coord_vector[1]..=coord_vector[3] {
                light_field[x][y] = !light_field[x][y];
            }
        }
    } else if line.contains("turn on") {
        for x in coord_vector[0]..=coord_vector[2] {
            for y in coord_vector[1]..=coord_vector[3] {
                light_field[x][y] = true;
            }
        }
    } else if line.contains("turn off") {
        for x in coord_vector[0]..=coord_vector[2] {
            for y in coord_vector[1]..=coord_vector[3] {
                light_field[x][y] = false;
            }
        }
    }
}

fn q2_mutate_lights(line: &str, light_field: &mut Vec<Vec<usize>>) {
    let delim_str: Vec<&str> = line.split(|c| c == ' ' || c == ',').collect();
    let coord_vector: Vec<usize> = delim_str.iter()
                                .filter(|x| x.chars().all(|c| c.is_ascii_digit()))
                                .map(|digit| digit.parse::<usize>().unwrap())
                                .collect();   
    if line.contains("toggle") {
        for x in coord_vector[0]..=coord_vector[2] {
            for y in coord_vector[1]..=coord_vector[3] {
                light_field[x][y] += 2;
            }
        }
    } else if line.contains("turn on") {
        for x in coord_vector[0]..=coord_vector[2] {
            for y in coord_vector[1]..=coord_vector[3] {
                light_field[x][y] += 1;
            }
        }
    } else if line.contains("turn off") {
        for x in coord_vector[0]..=coord_vector[2] {
            for y in coord_vector[1]..=coord_vector[3] {
                if light_field[x][y] != 0 {
                    light_field[x][y] -= 1;
                }
            }
        }
    }
}

fn main() {
    let input_data = read_to_string("input/day06.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
