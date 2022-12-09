use std::collections::HashSet;

#[derive(Clone)]
struct Knot {
    x: i32,
    y: i32,
}

pub fn run(input: String) {
    let input_copy = String::from(input);
    println!("Pt1 {}", run_for_len(&input_copy, 2));
    println!("Pt2 {}", run_for_len(&input_copy, 10));
}

fn run_for_len(input: &String, number_of_knots: usize) -> usize {
    let mut knots: Vec<Knot> = vec![Knot { x: 0, y: 0 }; number_of_knots];
    let mut tail_set: HashSet<String> = HashSet::new();

    for line in input.lines() {
        println!("{}", line);
        for idx in 0..knots.len() - 1 {
            let mut params = line.split(" ");
            let cmd = params.next().unwrap();
            let step_size = params.next().unwrap().parse::<i32>().unwrap();

            for _ in 0..step_size {
                if idx == 0 {
                    if cmd == "L" {
                        knots[idx].x -= 1;
                    } else if cmd == "R" {
                        knots[idx].x += 1;
                    } else if cmd == "D" {
                        knots[idx].y -= 1;
                    } else if cmd == "U" {
                        knots[idx].y += 1;
                    } else {
                        panic!("cmd");
                    }
                }

                if (knots[idx].x - knots[idx + 1].x).abs() > 1 {
                    if knots[idx].x > knots[idx + 1].x {
                        knots[idx + 1].x += 1;
                    } else {
                        knots[idx + 1].x -= 1;
                    }

                    if knots[idx].y > knots[idx + 1].y {
                        knots[idx + 1].y += 1;
                    } else if knots[idx].y < knots[idx + 1].y {
                        knots[idx + 1].y -= 1;
                    }
                } else if (knots[idx].y - knots[idx + 1].y).abs() > 1 {
                    if knots[idx].y > knots[idx + 1].y {
                        knots[idx + 1].y += 1;
                    } else {
                        knots[idx + 1].y -= 1;
                    }

                    if knots[idx].x > knots[idx + 1].x {
                        knots[idx + 1].x += 1;
                    } else if knots[idx].x < knots[idx + 1].x {
                        knots[idx + 1].x -= 1;
                    }
                }
                
                if idx == knots.len() - 2 {
                    let key = format!("{},{}", knots[idx + 1].x, knots[idx + 1].y);
                    if !tail_set.contains(&key) {
                        tail_set.insert(key);
                    }
                }
            }            
        }
        /*for row in (0..10).rev() {
            for col in 0..10 {
                let mut character:String = String::from("#");
                for (k_idx, k) in knots.iter().enumerate() {
                    if k.x == col && k.y == row {
                        character = String::from(format!("{}", k_idx));
                        break;
                    }
                }
                print!("{}", character);
            }
            println!();
        }
        println!();*/
    }
    return tail_set.len();
}
