use std::collections::HashMap;
use std::vec::Vec;

use itertools::Itertools;

pub fn run(input: String) {
    let input_split = input.split("\r\n\r\n");
    let mut crates: HashMap<usize, Vec<char>> = HashMap::new();
    let mut crates_pt2: HashMap<usize, Vec<char>> = HashMap::new();
    let mut indices: HashMap<usize, usize> = HashMap::new();
    for (idx, split_string) in input_split.enumerate() {
        let lines = split_string.lines();
        if idx == 0 {
            // crates
            for (i, line) in lines.rev().enumerate() {
                for (j, character) in line.chars().enumerate() {
                    if [']', '[', ' '].contains(&character) {
                        continue;
                    }
                    if i == 0 {
                        crates.insert(j, Vec::new());
                        indices.insert(crates.len(), j);
                    } else {
                        let v = crates.get_mut(&j).unwrap();
                        v.push(character);
                    }
                }
            }
            crates_pt2 = crates.clone();
        } else {
            // moves
            for line in lines {
                let mut m = 0;
                let mut f = 0;
                let mut t = 0;

                for (i, substring) in line.split(' ').enumerate() {
                    if i == 1 {
                        m = substring.parse::<i32>().unwrap() as usize;
                    }
                    if i == 3 {
                        f = substring.parse::<i32>().unwrap() as usize;
                    }
                    if i == 5 {
                        t = substring.parse::<i32>().unwrap() as usize;
                    }
                }
                let mut v_pt2 = Vec::new();
                for _ in 0..m {
                    let character;
                    {
                        let v_f = crates.get_mut(indices.get(&f).unwrap()).unwrap();
                        character = v_f.pop().unwrap();

                        let v_f_pt2 = crates_pt2.get_mut(indices.get(&f).unwrap()).unwrap();
                        v_pt2.push(v_f_pt2.pop().unwrap());
                    }
                    let v_t = crates.get_mut(indices.get(&t).unwrap()).unwrap();
                    v_t.push(character);
                }

                while (v_pt2.len() > 0) {
                    let v_t_pt2 = crates_pt2.get_mut(indices.get(&t).unwrap()).unwrap();
                    v_t_pt2.push(v_pt2.pop().unwrap());
                }
            }
        }
    }
    println!("{:?}", crates_pt2);

    print!("Pt1: ");
    print_crates(&crates);
    print!("Pt2: ");
    print_crates(&crates_pt2);
}

fn print_crates(crates: &HashMap<usize, Vec<char>>) {
    for (_, j) in crates.iter().sorted() {
        print!("{}", j.last().unwrap());
    }
    println!();
}
