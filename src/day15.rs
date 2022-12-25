use regex::Regex;
use std::{vec::Vec, cmp::Ordering};
use std::collections::HashSet;

pub fn run(input: String) {
    let rx = Regex::new(r"Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)").unwrap();
    let mut ranges_pt1:Vec<(i32, i32)> = Vec::new();
    //let target_y = 10;
    let target_y = 2000000;

    let mut beacons:HashSet<(i32, i32)> = HashSet::new();

    for line in input.lines() {
        let matches = rx.captures(line).unwrap();
        let x_sensor = matches[1].parse::<i32>().unwrap();
        let y_sensor = matches[2].parse::<i32>().unwrap();
        let x_beacon = matches[3].parse::<i32>().unwrap();
        let y_beacon = matches[4].parse::<i32>().unwrap();
        beacons.insert((x_beacon, y_beacon));
        let range_pt1 = get_range_where_you_cannot_possibly_find_a_beacon_on_y_coordinate(x_sensor, y_sensor, x_beacon, y_beacon, target_y);
        if range_pt1 == None {
            continue;
        }
        ranges_pt1.push(range_pt1.unwrap());
    }
    let joint_ranges_pt1 = join_ranges(&mut ranges_pt1);
    println!("Pt1: {}", count_joint_total(&joint_ranges_pt1, &beacons, target_y)); // TODO subtract number of beacons within the ranges
    
    /*for y in -2..23 {
        let range = get_range_where_you_cannot_possibly_find_a_beacon_on_y_coordinate(8, 7, 2, 10, y);
        for x in -2..26 {
            if range == None {
                print!(".");
            } else {
                let result = range.unwrap();
                if x >= result.0 && x <= result.1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!("");
    }*/
}

fn manh(x1:i32, y1:i32, x2:i32, y2:i32) -> i32 {
    return (x1-x2).abs() + (y1-y2).abs();
}

fn count_joint_total(ranges:&Vec<(i32,i32)>, beacons:&HashSet<(i32, i32)>, y_coord: i32) -> i32 {
    let mut total = 0;
    for r in ranges.iter() {
        total += r.1 - r.0 + 1 - get_number_of_beacons_in_range(*r, &beacons, y_coord);
    }
    return total;
}

fn get_number_of_beacons_in_range(range:(i32, i32), beacons:&HashSet<(i32, i32)>, y_coord: i32) -> i32 {
    let mut total = 0;
    for b in beacons.iter() {
        if b.1 != y_coord {
            continue;
        }
        if b.0 >= range.0 && b.0 <= range.1 {
            total += 1;
        }
    }
    return total;
}

fn join_ranges(ranges: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    ranges.sort_by(|a, b| compare_ranges(a, b));
    let mut new_ranges:Vec<(i32, i32)> = Vec::new();
    let mut to_skip:HashSet<usize> = HashSet::new();

    for i in 0..ranges.len() - 1 {
        if to_skip.contains(&i) {
            continue;
        }
        let r0 = ranges.get(i).unwrap();
        let mut next_range = (r0.0, r0.1);
        for j in i + 1..ranges.len() {
            let r1 = ranges.get(j).unwrap();
            if r1.0 <= next_range.1 { // overlap
                if r1.1 > next_range.1 {
                    next_range.1 = r1.1;
                }
                to_skip.insert(j);
            } else {
                break;
            }
        }
        new_ranges.push(next_range);
    }
    return new_ranges;
}

fn compare_ranges(a:&(i32, i32), b:&(i32, i32)) -> Ordering {
    if a.0 < b.0 {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

fn get_range_where_you_cannot_possibly_find_a_beacon_on_y_coordinate(x_sensor: i32, y_sensor: i32, x_beacon: i32, y_beacon: i32, y_coord: i32) -> Option<(i32, i32)> {
    let manh_to_closest_beacon = manh(x_sensor, y_sensor, x_beacon, y_beacon);
    let distance_to_y_coord = (y_sensor - y_coord).abs();
    let x_range = manh_to_closest_beacon - distance_to_y_coord;
    if x_range < 0 { // off by one error?
        return None;
    }
    return Some((x_sensor - x_range, x_sensor + x_range));
}