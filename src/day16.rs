use regex::Regex;
use std::collections::HashMap;

const MAX_STEPS: i32 = 30;

#[derive(Debug)]
struct Node {
    flow_rate: i32,
    connections: Vec<String>
}

pub fn run(input: String) {
    let rx = Regex::new(r"Valve (.+) has flow rate=(.+); (tunnel leads to valve|tunnels lead to valve)(s | )(.+)").unwrap();
    let mut map:HashMap<String, Node> = HashMap::new();

    for line in input.lines() {
        let matches = rx.captures(line).unwrap();
        let name = String::from(&matches[1]);
        let flow_rate = matches[2].parse::<i32>().unwrap();
        let connections:Vec<String> = matches[5].split(", ").map(|a| String::from(a)).collect();
        let node = Node {
            flow_rate: flow_rate,
            connections: connections
        };
        map.insert(name, node);
    }

    let mut max_pressure_release_and_flow_rate_for_step: HashMap<i32, (i32, i32)> = HashMap::new();
    let open_valves:Vec<String> = Vec::new();
    step(&map, &mut max_pressure_release_and_flow_rate_for_step, &open_valves, "AA".to_string(), &None, 0, 0, 0);
    println!("Pt1: {:?}", max_pressure_release_and_flow_rate_for_step.get(&29).unwrap());
}

fn step(map:&HashMap<String, Node>, max_pressure_release_and_flow_rate_for_step:&mut HashMap<i32, (i32, i32)>, open_valves:&Vec<String>, current_valve: String, last_valve: &Option<String>, mut num_of_steps: i32, total_pressure_release: i32, total_flow_rate: i32) {
    let current_node = map.get(&current_valve).unwrap();

    if num_of_steps == MAX_STEPS {
        if !should_drop_path(max_pressure_release_and_flow_rate_for_step, num_of_steps, total_pressure_release) {
            let max = max_pressure_release_and_flow_rate_for_step.get(&num_of_steps);
            if max == None || total_pressure_release > (*max.unwrap()).0 {
                max_pressure_release_and_flow_rate_for_step.insert(num_of_steps, (total_pressure_release, total_flow_rate));
            }
            //max_pressure_release_for_step.insert(num_of_steps, total_pressure_release);
        }
        return;
    }

    // open valve
    if current_node.flow_rate > 0 && !open_valves.contains(&current_valve) {
        let num_of_steps_valve = num_of_steps + 1;      
        let pressure_release = total_pressure_release + calc_pressure_release_for_valve(num_of_steps_valve, current_node.flow_rate);
        let flow_rate = total_flow_rate + current_node.flow_rate;
        if !should_drop_path(max_pressure_release_and_flow_rate_for_step, num_of_steps_valve, pressure_release) {
            let max = max_pressure_release_and_flow_rate_for_step.get(&num_of_steps_valve);
            if max == None || total_pressure_release > (*max.unwrap()).0 {
                max_pressure_release_and_flow_rate_for_step.insert(num_of_steps_valve, (pressure_release, flow_rate));
            }
            //max_pressure_release_for_step.insert(num_of_steps_valve, pressure_release);
            
            let mut cloned_open_valves = open_valves.clone();
            cloned_open_valves.push(current_valve.to_string());
            for c in &current_node.connections {
                // in this case, going back to the same room we were just in is legitimate
                step(map, max_pressure_release_and_flow_rate_for_step, &cloned_open_valves, c.to_string(), &Some(current_valve.to_string()), num_of_steps_valve + 1, pressure_release, flow_rate);
            }
        }
    }

    // go into a tunnel without turning valve
    for c in &current_node.connections {
        match last_valve {
            &None => (),
            Some(value) => {
                if value == c {
                    continue;
                }
            }
        };
        step(map, max_pressure_release_and_flow_rate_for_step, open_valves, c.to_string(), &Some(current_valve.to_string()), num_of_steps + 1, total_pressure_release, total_flow_rate);
    }
}

fn calc_pressure_release_for_valve(num_of_steps: i32, flow_rate: i32) -> i32 {
    return (MAX_STEPS - num_of_steps) * flow_rate;
}

fn should_drop_path(max_pressure_release_for_step:&HashMap<i32, (i32, i32)>, num_of_steps: i32, total_pressure_release: i32) -> bool {
    let max = max_pressure_release_for_step.get(&num_of_steps);
    if max == None {
        return false;
    } else {
        return total_pressure_release < (*max.unwrap()).0;
    }
}