use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

const MAX_STEPS: i32 = 30;

#[derive(Debug)]
struct Node {
    flow_rate: i32,
    connections: Vec<String>,
}

#[derive(Debug)]
struct State {
    pos: String,
    total_flow: i32,
    open_valves: HashSet<String>,
    previous_pos: String
}

pub fn run(input: String) {
    let map = parse_input(input);
    pt1(&map);
}

fn parse_input(input: String) -> HashMap<String, Node> {
    let rx = Regex::new(
        r"Valve (.+) has flow rate=(.+); (tunnel leads to valve|tunnels lead to valve)(s | )(.+)",
    )
    .unwrap();
    let mut map: HashMap<String, Node> = HashMap::new();

    for line in input.lines() {
        let matches = rx.captures(line).unwrap();
        let name = String::from(&matches[1]);
        let flow_rate = matches[2].parse::<i32>().unwrap();
        let connections: Vec<String> = matches[5].split(", ").map(|a| String::from(a)).collect();
        let node = Node {
            flow_rate,
            connections,
        };
        map.insert(name, node);
    }
    map
}

fn pt1(map: &HashMap<String, Node>) -> i32 {
    let state = State {
        pos: String::from("AA"),
        total_flow: 0,
        open_valves: HashSet::new(),
        previous_pos: String::from("AA")
    };
    let mut states:HashMap<String, Vec<State>> = HashMap::new();
    states.insert(String::from("AA"), vec![state]);

    let mut step = 0;
    while step < MAX_STEPS {
        let mut next_states: HashMap<String, Vec<State>> = HashMap::new();
        step += 1;
        println!("Step {step}");
        for (_, current_states) in states {
            for current_state in current_states {

            let node_flow_rate = map.get(&current_state.pos).unwrap().flow_rate;
            if node_flow_rate > 0 && !&current_state.open_valves.contains(&current_state.pos) {
                let mut next_open_valves = current_state.open_valves.clone();
                next_open_valves.insert(String::from(&current_state.pos));

                let next_total_flow = &current_state.total_flow + calc_total_flow(node_flow_rate, step);

                let mut found_better = false;
                if next_states.contains_key(&current_state.pos) {
                    for ns in next_states.get(&current_state.pos).unwrap() {
                        if ns.open_valves.is_superset(&next_open_valves) && ns.total_flow >= next_total_flow {
                            found_better = true;
                            break;
                        }
                    }
                }
                
                if !found_better {
                    let state_to_add = State {
                        pos: String::from(&current_state.pos),
                        total_flow: next_total_flow,
                        open_valves: next_open_valves,
                        previous_pos: String::from(&current_state.pos)
                    };
                    if !next_states.contains_key(&current_state.pos) {
                        next_states.insert(String::from(&current_state.pos), vec![state_to_add]);
                    } else {
                        next_states.get_mut(&current_state.pos).unwrap().push(state_to_add);
                    }
                    
                }
            }

            'checking_connections: for p in &map.get(&current_state.pos).unwrap().connections {
                if p == &current_state.previous_pos {
                    continue;
                }
                
                if next_states.contains_key(p) {
                    for ns in next_states.get(p).unwrap() {
                        if ns.open_valves.is_superset(&current_state.open_valves) && ns.total_flow >= current_state.total_flow {
                            continue 'checking_connections;
                        }
                    }
                }
                
                let state_to_add = State {
                    pos: String::from(p),
                    total_flow: current_state.total_flow,
                    open_valves: current_state.open_valves.clone(),
                    previous_pos: String::from(&current_state.pos)
                };

                if !next_states.contains_key(p) {
                    next_states.insert(String::from(p), vec![state_to_add]);
                } else {
                    next_states.get_mut(p).unwrap().push(state_to_add);
                }
            }
        }
        }
        
        states = dbg!(next_states);
        println!("Len: {}", states.len());
        println!("================");
    }

    let mut max = 0;
    for (_, states_for_pos) in states {
        for s in states_for_pos {
            if s.total_flow > max  {
                max = s.total_flow;
            }
        }
    }

    println!("Pt1: {max}");
    max
}

fn calc_total_flow(flow_rate: i32, step: i32) -> i32 {
    flow_rate * (MAX_STEPS - step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = String::from(
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II"
    );
        let map = parse_input(input);
        assert_eq!(pt1(&map), 1651);
    }
}