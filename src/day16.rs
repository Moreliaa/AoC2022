use regex::Regex;
use core::num;
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

fn pt1_pt2(map: &HashMap<String, HashMap<String, i32>>, flow_rates: &HashMap<String, Node>, number_of_actors: i32) -> i32 {
    let mut states:Vec<NewState> = Vec::new();
    let mut initial_actors:Vec<Actor> = Vec::new();
    for _ in 0..number_of_actors {
        initial_actors.push(Actor {
            last_pos: String::from("AA"),
            target_pos: String::from("AA"),
            steps_until_arrival: 0
        });
    }

    let mut initial_state = NewState {
        actors: initial_actors,
        total_flow: 0,
        open_valves: HashSet::new()
    };
    for (node_name, node) in flow_rates {
        if node.flow_rate == 0 && node_name != "AA" {
            initial_state.open_valves.insert(String::from(node_name));
        }
    } 

    states.push(initial_state);

    let mut step = 0;
    while step < MAX_STEPS {
        step += 1;
        println!("Step: {step}");
        let mut next_states:Vec<NewState> = Vec::new();
        for s in &states {
            let mut next_valves = s.open_valves.clone();
            let mut next_flow = s.total_flow;
            let mut next_actors: Vec<Vec<Actor>> = Vec::new();
            let mut first_valve_turned = false; // account for both actors starting on AA on step 1

            for a in &s.actors {
                if !next_valves.contains(&a.target_pos) && a.steps_until_arrival == 0 && !first_valve_turned {
                        let flow_rate_target = flow_rates.get(&a.target_pos).unwrap().flow_rate;
                        next_flow += calc_total_flow(flow_rate_target, step);
                        next_valves.insert(String::from(&a.target_pos));
                        if step == 1 {
                            first_valve_turned = true;
                        }
                }
            }

            let mut is_second_actor = false;
            for a in &s.actors {
                let mut possible_actor_states: Vec<Actor> = Vec::new();
                if a.steps_until_arrival > 0 {
                    possible_actor_states.push(Actor {
                        last_pos: String::from(&a.last_pos),
                        target_pos: String::from(&a.target_pos),
                        steps_until_arrival: a.steps_until_arrival - 1
                    });
                } else {
                    // arrived, turn valve and look for next connections
                    let distance_offset = if step == 1 && ((next_flow == 0 && first_valve_turned) || (first_valve_turned && is_second_actor)) {-1} else {0}; // take into account first node being 0 pressure
                    is_second_actor = true;
                    for (next_node, distance) in map.get(&a.target_pos).unwrap() {
                        if next_valves.contains(next_node) {
                            continue;
                        }
                        possible_actor_states.push(Actor {
                                last_pos: String::from(&a.target_pos),
                                target_pos: String::from(next_node),
                                steps_until_arrival: *distance + distance_offset // + 1 to account for time taken for the valve
                        });
                    }

                    if next_valves.len() == map.len() {
                        possible_actor_states.push(Actor {
                            last_pos: String::from(&a.last_pos),
                            target_pos: String::from(&a.target_pos),
                            steps_until_arrival: 0 // + 1 to account for time taken for the valve
                         });
                    }
                }
                next_actors.push(possible_actor_states);
            }

            if next_actors.len() == 1 {
                for a in &next_actors[0] {
                    let a_new = Actor {
                        last_pos: String::from(&a.last_pos),
                        target_pos: String::from(&a.target_pos),
                        steps_until_arrival: a.steps_until_arrival
                    };
                    dbg!(next_flow);
                    next_states.push(NewState {
                        actors: vec![a_new],
                        total_flow: next_flow,
                        open_valves: next_valves.clone()
                    });
                }
            } else {
                
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
    pt1_pt2(&distance_map, &map, 1);
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
            assert_eq!(pt1_pt2(&other_map, &map, 1), 1651);
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
            assert_eq!(pt1_pt2(&other_map, &map, 2), 1707);
    }
}