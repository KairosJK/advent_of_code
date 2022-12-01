use std::{fs::read_to_string, collections::HashMap};

fn parse_line<'a>(input: &'a str, circuit: &mut HashMap<&'a str,u16>) -> Option<()> {

    fn derive_val(x: &str, circuit: &HashMap<&str, u16>) -> Option<u16> {
        if x.chars().all(|c| c.is_ascii_digit()) {
            return Some(x.parse::<u16>().unwrap());
        }
        circuit.get(x).copied()
    }

    let delim_line: Vec<&str> = input.split(' '). collect();
    match delim_line.len() {
        3 => { // assignment operation
            let op1 = derive_val(delim_line[0], &circuit)?;
            circuit.insert(delim_line[2], op1);
        },
        4 => { // not operation
            let op1 = derive_val(delim_line[1], &circuit)?;
            circuit.insert(delim_line[3], !op1);
        },
        5 => { // bitwise operation with assignment
            let x = delim_line[1];
            if x.eq("AND") {
                let op1 = derive_val(delim_line[0], &circuit)?;
                let op2 = derive_val(delim_line[2], &circuit)?;
                circuit.insert(delim_line[4],op1 & op2);
            } else if x.eq("OR") {
                let op1 = derive_val(delim_line[0], &circuit)?;
                let op2 = derive_val(delim_line[2], &circuit)?;
                circuit.insert(delim_line[4], op1 | op2);
            } else if x.eq("LSHIFT") {
                let op1 = derive_val(delim_line[0], &circuit)?;
                let op2 = derive_val(delim_line[2], &circuit)?;
                circuit.insert(delim_line[4], op1 << op2);
            } else if x.eq("RSHIFT") {
                let op1 = derive_val(delim_line[0], &circuit)?;
                let op2 = derive_val(delim_line[2], &circuit)?;
                circuit.insert(delim_line[4], op1 >> op2);
            } 
        },
        _ => (),
    }
Some(())
}

fn problem_one(input: &String) -> u16 {
    let mut circuit: HashMap<&str,u16>= HashMap::new();
    let mut line_iter: Vec<&str> = input.lines().collect();

    let mut current_pos = 0;
    while !line_iter.is_empty() {
        parse_line(line_iter[current_pos], &mut circuit);
        match parse_line(line_iter[current_pos], &mut circuit) {
            Some(_) => {
                line_iter.remove(current_pos);
            }
            None => {
                current_pos += 1;
            }
        }
        if current_pos >= line_iter.len() {
            current_pos = 0;
        }
    }
    *circuit.get("a").unwrap()
}

fn problem_two(input: &String) -> u16 {
    let mut circuit: HashMap<&str,u16>= HashMap::new();
    let mut line_iter: Vec<&str> = input.lines().collect();
    let wire_a_val = problem_one(input);

    let mut current_pos = 0;
    while !line_iter.is_empty() {
        circuit.insert("b", wire_a_val);
        parse_line(line_iter[current_pos], &mut circuit);
        match parse_line(line_iter[current_pos], &mut circuit) {
            Some(_) => {
                line_iter.remove(current_pos);
            }
            None => {
                current_pos += 1;
            }
        }
        if current_pos >= line_iter.len() {
            current_pos = 0;
        }
    }
    *circuit.get("a").unwrap()
}

fn main() {
    let input_data = read_to_string("input/day07.input").unwrap();
    println!("Problem 1: {}", problem_one(&input_data));
    println!("Problem 2: {}", problem_two(&input_data));
}
