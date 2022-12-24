use std::vec::Vec;

pub fn run(input: String) {
    let mut lines: Vec<&str> = Vec::new();
    for line in input.lines() {
        if line != "" {
            lines.push(line);
        }
    }
    let mut pt1 = 0;
    let mut pt1_line_idx = 1;
    for line_idx in (0..lines.len()).step_by(2) {
        println!("LINE INDEX {}", pt1_line_idx);
        let line_a = String::from(lines[line_idx]);
        let line_b = String::from(lines[line_idx + 1]);
        let result = compare_values(&line_a, &line_b);
        match result {
            Some(false) => (),
            Some(true) => {
                pt1 += pt1_line_idx;
                println!("OK: {}", pt1_line_idx);
            }
            None => panic!(),
        };
        pt1_line_idx += 1;
        println!("==============================================");
        //println!("Result: {} for {} - {}", result, line_a, line_b);
    }
    println!("Pt1: {}", pt1);
}

fn compare_values(str_a: &String, str_b: &String) -> Option<bool> {
    let list_a = split_top_level(&str_a);
    let list_b = split_top_level(&str_b);
    println!("Compare {:?} === {:?}", list_a, list_b);

    for (idx, item_a) in list_a.iter().enumerate() {
        let item_b = match list_b.get(idx) {
            Some(value) => value,
            None => {
                println!("len_b < len_a: {} < {}", list_b.len(), list_a.len());
                return Some(false);
            } // b ran out of items before a
        };
        let item_a_is_int = item_a.chars().nth(0).unwrap() != '[';
        let item_b_is_int = item_b.chars().nth(0).unwrap() != '[';
        if item_a_is_int && item_b_is_int {
            // both ints
            let parsed_a = item_a.parse::<i32>().unwrap();
            let parsed_b = item_b.parse::<i32>().unwrap();
            println!("ints: {}, {}", parsed_a, parsed_b);
            if parsed_a == parsed_b {
                continue;
            } else {
                return Some(parsed_a < parsed_b);
            }
        } else if item_a_is_int {
            // only a is int
            let new_a = format!("[{}]", item_a);
            match compare_values(&new_a, item_b) {
                Some(value) => return Some(value),
                None => continue,
            };
        } else if item_b_is_int {
            // only b is int
            let new_b = format!("[{}]", item_b);
            match compare_values(item_a, &new_b) {
                Some(value) => return Some(value),
                None => continue,
            };
        } else {
            // both lists
            match compare_values(item_a, item_b) {
                Some(value) => return Some(value),
                None => continue,
            };
        }
    }
    
    if list_a.len() == list_b.len() {
        return None; // inconclusive
    }
    return Some(list_a.len() < list_b.len());
}

fn split_top_level(input: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut next = String::new();
    let mut depth = 0;
    let split_level = get_split_level(input);
    print!("Split: {} -> ", input);
    for chara in input.chars() {
        if depth == split_level && (chara == ',' || chara == ']') {
            if next.len() == 0 {
                // empty list []
                continue;
            }
            print!("{} ", next);
            result.push(String::from(next));
            next = String::new();
            continue;
        }
        if depth >= split_level {
            next.push(chara);
        }
        if chara == '[' {
            depth += 1;
        } else if chara == ']' {
            depth -= 1;
        }
    }

    if next.len() > 0 {
        // input was an int
        print!("{} ", next);
        result.push(String::from(next));
    }
    println!("Len: {}", result.len());
    return result;
}

fn get_split_level(input: &String) -> i32 {
    let mut depth = 0;
    for (idx, chara) in input.chars().enumerate() {
        if chara == ',' && depth == 0 {
            return 0;
        }
        if chara == '[' {
            depth += 1;
        } else if chara == ']' {
            depth -= 1;
        }
        if idx < input.len() - 1 && depth == 0 {
            return 0;
        }
    }
    return 1;
}