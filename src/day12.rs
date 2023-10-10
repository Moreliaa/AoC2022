use std::collections::HashMap;
use std::vec::Vec;

struct Node {
    x: usize,
    y: usize,
    visited: bool,
    distance: usize,
}

pub fn run(input: String) {
    let mut x_target = 0;
    let mut y_target = 0;
    let chars = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut starting_positions: Vec<(usize, usize)> = Vec::new();

    let mut grid = vec![vec![0; 0]; 0];
    for (i_line, line) in input.lines().enumerate() {
        let mut vector: Vec<i32> = Vec::new();
        for (i_chara, chara) in line.chars().enumerate() {
            let value;
            if chara == 'S' {
                starting_positions.push((i_chara, i_line));
                value = chars.iter().position(|&c| c == 'a').unwrap();
            } else if chara == 'E' {
                x_target = i_chara;
                y_target = i_line;
                value = chars.iter().position(|&c| c == 'z').unwrap();
            } else {
                if chara == 'a' {
                    starting_positions.push((i_chara, i_line));
                }
                value = chars.iter().position(|&c| c == chara).unwrap();
            }
            vector.push(value as i32);
        }
        grid.push(vector);
    }

    let mut min_distance = 999999999999;
    for pos in starting_positions {
        let distance = dijkstra(pos.0, pos.1, x_target, y_target, &grid);
        if distance < min_distance {
            min_distance = distance;
        }
    }
    println!("Pt2: {}", min_distance);
}

fn dijkstra(
    x_start: usize,
    y_start: usize,
    x_target: usize,
    y_target: usize,
    grid: &Vec<Vec<i32>>,
) -> usize {
    let mut x = x_start;
    let mut y = y_start;
    let key_start = format!("{},{}", x, y);
    let mut nodes: HashMap<String, Node> = HashMap::new();
    nodes.insert(
        key_start,
        Node {
            x: x,
            y: y,
            distance: 0,
            visited: false,
        },
    );

    // dijkstra
    while x != x_target || y != y_target {
        let key = format!("{},{}", x, y);
        let next_distance;
        {
            let node = nodes.get_mut(&key).unwrap();
            node.visited = true;
            next_distance = node.distance + 1;
        }
        // update nodes
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for dir in directions {
            let x_next = dir.0 + x as i32;
            let y_next = dir.1 + y as i32;
            if x_next < 0
                || y_next < 0
                || x_next as usize >= grid[0].len()
                || y_next as usize >= grid.len()
            {
                continue; // out of bounds
            }
            if (grid[y_next as usize][x_next as usize] - grid[y][x]) > 1 {
                continue; // height difference
            }
            let next_key = format!("{},{}", x_next, y_next);
            if !nodes.contains_key(&next_key) {
                let next_node = Node {
                    x: x_next as usize,
                    y: y_next as usize,
                    distance: next_distance,
                    visited: false,
                };
                nodes.insert(next_key, next_node);
            } else {
                let next_node = nodes.get_mut(&next_key).unwrap();
                if next_distance < next_node.distance {
                    next_node.distance = next_distance;
                }
            }
        }

        {
            // select next node
            let mut x_next = x;
            let mut y_next = y;
            let mut min_distance = 99999999999;
            for (_, node) in &nodes {
                if !node.visited && node.distance <= min_distance {
                    x_next = node.x;
                    y_next = node.y;
                    min_distance = node.distance;
                }
            }
            if min_distance == 99999999999 {
                break;
            }
            x = x_next;
            y = y_next;
        }
    }

    let target_key = format!("{},{}", x_target, y_target);
    if nodes.contains_key(&target_key) {
        println!(
            "Result for starting position: {},{} = {}",
            x_start,
            y_start,
            nodes.get(&target_key).unwrap().distance
        );
        return nodes.get(&target_key).unwrap().distance;
    } else {
        return 99999999999999;
    }
}
