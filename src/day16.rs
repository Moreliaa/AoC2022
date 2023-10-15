use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

const MAX_STEPS: i32 = 30;
const MAX_STEPS_PT2: i32 = 26;

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

#[derive(Debug)]
struct NewState {
    actors: Vec<Actor>,
    total_flow: i32,
    open_valves: HashSet<String>
}

#[derive(Debug)]
struct Actor {
    last_pos: String,
    target_pos: String,
    steps_until_arrival: i32,
}

fn pt1_new(map: &HashMap<String, HashMap<String, i32>>, flow_rates: &HashMap<String, Node>) -> i32 {
    let mut states:Vec<NewState> = Vec::new();

    let initial_state = NewState {
        actors: vec![Actor {
            last_pos: String::from("AA"),
            target_pos: String::from("AA"),
            steps_until_arrival: 0
        }],
        total_flow: 0,
        open_valves: HashSet::new()
    };
    states.push(initial_state);

    let mut step = 0;
    while step < MAX_STEPS {
        step += 1;
        println!("Step: {step}");
        let mut next_states:Vec<NewState> = Vec::new();
        for s in &states {
            if s.actors[0].steps_until_arrival > 0 {
                // moving, push state
                let new_state = NewState {
                    actors: vec![Actor {
                        last_pos: String::from(&s.actors[0].last_pos),
                        target_pos: String::from(&s.actors[0].target_pos),
                        steps_until_arrival: s.actors[0].steps_until_arrival - 1
                    }],
                    total_flow: s.total_flow,
                    open_valves: s.open_valves.clone()
                };
                next_states.push(new_state);
            } else {
                // arrived, turn valve and look for next connections
                let flow_rate_target = flow_rates.get(&s.actors[0].target_pos).unwrap().flow_rate;
                let next_flow = s.total_flow + calc_total_flow(flow_rate_target, step);

                let mut next_valves = s.open_valves.clone();
                next_valves.insert(String::from(&s.actors[0].target_pos));

                let distance_offset = if step == 1 && next_flow == 0 {-1} else {0}; // take into account first node being 0 pressure
                for (next_node, distance) in map.get(&s.actors[0].target_pos).unwrap() {
                    if next_valves.contains(next_node) {
                        continue;
                    }
                    let new_state = NewState {
                        actors: vec![Actor {
                            last_pos: String::from(&s.actors[0].target_pos),
                            target_pos: String::from(next_node),
                            steps_until_arrival: *distance + distance_offset // + 1 to account for time taken for the valve
                        }],
                        total_flow: next_flow,
                        open_valves: next_valves.clone()
                    };
                    next_states.push(new_state);
                }
            }
        }
        states = next_states;
    }

    let mut max = 0;
    for s in states {
        if s.total_flow > max  {
            max = s.total_flow;
        }
    }

    println!("Pt1 New: {max}");
    max
}

pub fn run(input: String) {
    let map = parse_input(input);
    let distance_map = build_distance_map(&map);
    pt1_new(&distance_map, &map);
    pt1(&map);
    pt2(&distance_map, &map);
}



fn build_distance_map(map: &HashMap<String, Node>) -> HashMap<String, HashMap<String, i32>> {
    let mut result = HashMap::new();
    for (key, _) in map {
        result.insert(String::from(key), build_distances_for_node(map, key));
    }
    result
}

fn build_distances_for_node(map: &HashMap<String, Node>, key: &String) -> HashMap<String, i32> {
    let mut result = HashMap::new();
    let mut seen: HashSet<String> = HashSet::new();

    let mut pos = String::from(key);

    result.insert(String::from(&pos), 0);

    while result.len() < map.len() {
        let steps = *result.get(&pos).unwrap() + 1;
        let current_node = map.get(&pos).unwrap();
        seen.insert(String::from(&pos));

        for n in &current_node.connections {
            if !result.contains_key(n) {
                result.insert(String::from(n), steps);
            }
        }

        // get next node
        let mut lowest_unseen_steps = -1;
        for (k, val) in &result {
            if seen.contains(k) {
                continue;
            }
            if lowest_unseen_steps < 0 || *val < lowest_unseen_steps {
                lowest_unseen_steps = *val;
                pos = String::from(k);
            }
        }
    }
    result
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
                        if ns.open_valves.is_subset(&next_open_valves) && ns.total_flow >= next_total_flow {
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
                        if ns.open_valves.is_subset(&current_state.open_valves) && ns.total_flow >= current_state.total_flow {
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
        
        states = next_states;
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

fn pt2(map: &HashMap<String, HashMap<String, i32>>, flow_rates: &HashMap<String, Node>) -> i32 {
        0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_new() {
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
            let other_map = build_distance_map(&map);
            assert_eq!(pt1_new(&other_map, &map), 1651);
    }

    #[test]
    fn test_pt1() {
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

    #[test]
    fn test_pt2() {
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
            let other_map = build_distance_map(&map);
            assert_eq!(pt2(&other_map, &map), 1707);
    }
}