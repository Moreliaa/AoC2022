use std::vec::Vec;

pub fn run(input: String) {
    let mut x = 1;
    let mut cycle = 0;
    let mut last_x;
    let mut last_cycle;
    let mut pt1 = 0;
    let mut pt2:Vec<char> = Vec::new();

    let target_cycles = [20, 60, 100, 140, 180, 220];
    let mut next_target_cycle_idx = 0;
    for line in input.lines() {
        last_x = x;
        last_cycle = cycle;
        (x, cycle) = read_line(&line, x, cycle);
        if next_target_cycle_idx < target_cycles.len() {
            if cycle == target_cycles[next_target_cycle_idx] {
                pt1 += last_x * cycle;
                //println!("a {} * {} = {}", target_cycles[next_target_cycle_idx], last_x, last_x * cycle);
                next_target_cycle_idx += 1;
            } else if cycle > target_cycles[next_target_cycle_idx] &&
            last_cycle < target_cycles[next_target_cycle_idx] { // skipped over in last read
                pt1 += last_x * target_cycles[next_target_cycle_idx];
                //println!("b {} * {} = {}", target_cycles[next_target_cycle_idx], last_x, last_x * target_cycles[next_target_cycle_idx]);
                next_target_cycle_idx += 1;
            } 
        }

        for idx in 0..cycle as usize - last_cycle as usize {
            let cursor_pos = (last_cycle + idx as i32) % 40;
            let next_char = match (cursor_pos - last_x).abs() <= 1 {
                true => '#',
                false => '.'
            };
            pt2.push(next_char);
        }
    }
    println!("Pt1 {}", pt1);
    for (idx, c) in pt2.iter().enumerate() {
        print!("{}", c);
        if (idx + 1) % 40 == 0 {
            println!();
        }
    }
}

fn read_line(line:&str,x:i32, cycle:i32) -> (i32, i32) {
    let mut params = line.split(" ");
    let cmd = params.next().unwrap();
    if cmd == "noop" {
        return (x, cycle + 1);
    }
    let value = params.next().unwrap().parse::<i32>().unwrap();
    if cmd == "addx" {
        return (x + value, cycle + 2);
    }
    panic!("read_line");
}