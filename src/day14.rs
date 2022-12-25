use std::collections::HashMap;
use std::vec::Vec;

pub fn run(input: String) {
    let mut map = build_map(String::from(&input));
    let mut pt1 = 0;
    while drop_sand(&mut map.0, map.1) {
        pt1 += 1;
    }
    let mut map_pt2 = build_map(input);
    let mut pt2 = 0;
    while drop_sand_pt2(&mut map_pt2.0, map_pt2.1) {
        pt2 += 1;
    }

    for y in 0..12 {
        for x in 480..520 {
            let key = format!("{},{}", x, y);
            if map_pt2.0.contains_key(&key) {
                print!("{}", map_pt2.0.get(&key).unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("Pt1: {}", pt1);
    println!("Pt2: {}", pt2);
}

fn drop_sand(map: &mut HashMap<String, char>, y_max: i32) -> bool {
    let mut x = 500;
    let mut y = 0;
    loop {
        let straight = format!("{},{}", x, y + 1);
        if map.contains_key(&straight) {
            let left = format!("{},{}", x - 1, y + 1);
            if map.contains_key(&left) {
                let right = format!("{},{}", x + 1, y + 1);
                if map.contains_key(&right) {
                    let current = format!("{},{}", x, y);
                    map.insert(current, 'o');
                    return true;
                } else {
                    x += 1;
                    y += 1;
                }
            } else {
                x -= 1;
                y += 1;
            }
        } else {
            y += 1;
        }
        if y > y_max {
            return false;
        }
    }
}

fn drop_sand_pt2(map: &mut HashMap<String, char>, y_max: i32) -> bool {
    let mut x = 500;
    let mut y = -1;
    loop {
        let straight = format!("{},{}", x, y + 1);
        if map.contains_key(&straight) {
            if x == 500 && y == -1 {
                return false;
            }
            let left = format!("{},{}", x - 1, y + 1);
            if map.contains_key(&left) {
                let right = format!("{},{}", x + 1, y + 1);
                if map.contains_key(&right) {
                    let current = format!("{},{}", x, y);
                    map.insert(current, 'o');
                    return true;
                } else {
                    x += 1;
                    y += 1;
                }
            } else {
                x -= 1;
                y += 1;
            }
        } else {
            y += 1;
        }
        if y > y_max {
            let current = format!("{},{}", x, y);
            map.insert(current, 'o');
            return true;
        }
    }
}

fn build_map(input: String) -> (HashMap<String, char>, i32) {
    let mut map: HashMap<String, char> = HashMap::new();
    let mut y_max = 0;
    for line in input.lines() {
        let mut last_x: Option<i32> = None;
        let mut last_y: Option<i32> = None;

        for coord in line.split(" -> ") {
            let mut coord_split: Vec<i32> = coord
                .split(',')
                .map(|a| a.parse::<i32>().unwrap())
                .collect();
            let x = *coord_split.get_mut(0).unwrap();
            let y = *coord_split.get_mut(1).unwrap();

            if last_x != None {
                if last_x.unwrap() == x && last_y.unwrap() == y {
                    panic!();
                }
                if last_x.unwrap() == x {
                    let min = if last_y.unwrap() < y {
                        last_y.unwrap() + 1
                    } else {
                        y + 1
                    };
                    let max = if last_y.unwrap() > y {
                        last_y.unwrap()
                    } else {
                        y
                    };
                    for next_y in min as usize..max as usize {
                        let key = format!("{},{}", x, next_y);
                        if !map.contains_key(&key) {
                            map.insert(key, '#');
                        }
                    }
                } else if last_y.unwrap() == y {
                    let min = if last_x.unwrap() < x {
                        last_x.unwrap() + 1
                    } else {
                        x + 1
                    };
                    let max = if last_x.unwrap() > x {
                        last_x.unwrap()
                    } else {
                        x
                    };
                    for next_x in min as usize..max as usize {
                        let key = format!("{},{}", next_x, y);
                        if !map.contains_key(&key) {
                            map.insert(key, '#');
                        }
                    }
                } else {
                    panic!();
                }
            }

            let key = String::from(coord);
            if !map.contains_key(&key) {
                map.insert(key, '#');
            }

            if y > y_max {
                y_max = y;
            }

            last_x = Some(x);
            last_y = Some(y);
        }
    }
    return (map, y_max);
}
