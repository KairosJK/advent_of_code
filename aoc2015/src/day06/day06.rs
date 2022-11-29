use std::fs::read_to_string;

fn problem_one(input: &String) -> usize {
    let mut light_field = vec![vec![false; 1000]; 1000];
    for current_line in input.lines() {
        mutate_lights(current_line, &mut light_field)
    }
    for i in light_field {
        for x in i {
            match x {
                true => print!("1 "),
                false => print!("0 "),
                _ => (),
            }
        }
        print!("\n");
    }
    return 0
    //light_field.iter().flatten().filter(|x| **x).count()
}

/* fn problem_two(input: &String) -> i32 {
    return 0;
} */

fn mutate_lights(line: &str, light_field: &mut Vec<Vec<bool>>) {
    let delim_str: Vec<&str> = line.split(|c| c == ' ' || c == ',').collect();
    let coord_vector: Vec<usize> = delim_str.iter()
                                .filter(|x| x.chars().all(|c| c.is_ascii_digit()))
                                .map(|digit| digit.parse::<usize>().unwrap())
                                .collect();   
     if line.contains("toggle") {
        light_negate(coord_vector, light_field);
    } else if line.contains("turn on") {
        light_on(coord_vector, light_field);
    } else if line.contains("turn off") {
        light_off(coord_vector, light_field);
    }
}

fn light_negate(coord_vector: Vec<usize>, light_field: &mut Vec<Vec<bool>>) {
    for x in coord_vector[0]..coord_vector[2] {
        for y in coord_vector[1]..coord_vector[3] {
            light_field[x][y] = !light_field[x][y];
        }
    }
}

fn light_on(coord_vector: Vec<usize>, light_field: &mut Vec<Vec<bool>>) {
    for x in coord_vector[0]..coord_vector[2] {
        for y in coord_vector[1]..coord_vector[3] {
            light_field[x][y] = true;
        }
    }
}

fn light_off(coord_vector: Vec<usize>, light_field: &mut Vec<Vec<bool>>) {
    for x in coord_vector[0]..coord_vector[2] {
        for y in coord_vector[1]..coord_vector[3] {
            light_field[x][y] = false;
        }
    }
}

fn main() {
    let input_data = read_to_string("input/day0601.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    /* println!("Problem 2: {}", problem_two(&input_data)); */
}
