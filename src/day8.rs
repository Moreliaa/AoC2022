use std::vec::Vec;

pub fn run(input: String) {
    let mut pt1 = 0;
    let mut pt2 = 0;
    let trees = parse_lines_into_grid(&input);
    for row in 0..trees.len() {
        for col in 0..trees[0].len() {
            pt1 += match is_visible(row, col, &trees) {
                true => 1,
                false => 0,
            };
            let pt2_candidate = calc_score(row, col, &trees);
            if pt2_candidate > pt2 {
                pt2 = pt2_candidate;
            }
        }
    }
    println!("Pt1: {}", pt1);
    println!("Pt2: {}", pt2);
}

fn is_visible(target_row: usize, target_col: usize, trees: &Vec<Vec<i32>>) -> bool {
    let target = trees[target_row][target_col];
    let mut blocked = false;
    for row in 0..target_row {
        // top
        if trees[row][target_col] >= target {
            blocked = true;
            break;
        }
    }
    if !blocked {
        return true;
    }
    blocked = false;
    for row in target_row + 1..trees.len() {
        // bottom
        if trees[row][target_col] >= target {
            blocked = true;
            break;
        }
    }
    if !blocked {
        return true;
    }
    blocked = false;
    for col in 0..target_col {
        // left
        if trees[target_row][col] >= target {
            blocked = true;
            break;
        }
    }
    if !blocked {
        return true;
    }
    blocked = false;
    for col in target_col + 1..trees[0].len() {
        // right
        if trees[target_row][col] >= target {
            blocked = true;
            break;
        }
    }
    return !blocked;
}

fn calc_score(target_row: usize, target_col: usize, trees: &Vec<Vec<i32>>) -> i32 {
    let target = trees[target_row][target_col];
    let mut viewing_distances = (0, 0, 0, 0);
    let mut next_dist = 0;
    for row in (0..target_row).rev() {
        // top
        next_dist += 1;
        if trees[row][target_col] >= target {
            break;
        }
    }
    viewing_distances.0 = next_dist;
    next_dist = 0;
    for row in target_row + 1..trees.len() {
        // bottom
        next_dist += 1;
        if trees[row][target_col] >= target {
            break;
        }
    }
    viewing_distances.1 = next_dist;
    next_dist = 0;
    for col in (0..target_col).rev() {
        // left
        next_dist += 1;
        if trees[target_row][col] >= target {
            break;
        }
    }
    viewing_distances.2 = next_dist;
    next_dist = 0;
    for col in target_col + 1..trees[0].len() {
        // right
        next_dist += 1;
        if trees[target_row][col] >= target {
            break;
        }
    }
    viewing_distances.3 = next_dist;
    /*println!(
        "{} {}, {:?} - {}",
        target_row,
        target_col,
        viewing_distances,
        viewing_distances.0 * viewing_distances.1 * viewing_distances.2 * viewing_distances.3
    );*/
    return viewing_distances.0 * viewing_distances.1 * viewing_distances.2 * viewing_distances.3;
}

fn parse_lines_into_grid(input: &String) -> Vec<Vec<i32>> {
    let input_split = input.lines();
    let mut grid = vec![vec![0; 0]; 0];
    for line in input_split {
        let mut vector: Vec<i32> = Vec::new();
        for c in line.chars() {
            vector.push(String::from(c).parse::<i32>().unwrap());
        }
        grid.push(vector);
    }
    return grid;
}
