use std::fs;
use std::vec::Vec;

pub fn run(input: String) {
    let input_split = input.lines();
    let mut current = 0;
    let mut max = 0;
    let mut numbers = Vec::new();

    for line in input_split {
        if line == "" {
            if (current > max) {
                max = current;
            }
            numbers.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    numbers.sort();
    let mut pt2_sum = 0;
    let max_3 = &numbers[numbers.len()-3..numbers.len()];
    for max_val in max_3 {
        pt2_sum += max_val;
    }

    println!("Pt1: {}", max);
    println!("Pt2: {}", pt2_sum);
}