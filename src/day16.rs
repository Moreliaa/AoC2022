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
    };
    let mut states = vec![state];

    let mut step = 0;
    while step < MAX_STEPS {
        let mut next_states: Vec<State> = Vec::new();
        step += 1;
        println!("Step {step}");
        for s in states {
            let node_flow_rate = map.get(&s.pos).unwrap().flow_rate;
            if node_flow_rate > 0 && !&s.open_valves.contains(&s.pos) {
                let mut next_open_valves = s.open_valves.clone();
                next_open_valves.insert(String::from(&s.pos));
                next_states.push(State {
                    pos: String::from(&s.pos),
                    total_flow: &s.total_flow + calc_total_flow(node_flow_rate, step),
                    open_valves: next_open_valves,
                });
            }

            'checking_connections: for p in &map.get(&s.pos).unwrap().connections {
                for ns in &next_states {
                    if &ns.pos == p {
                        if ns.total_flow > s.total_flow || (ns.total_flow == s.total_flow && ns.open_valves == s.open_valves) {
                            continue 'checking_connections;
                        }
                    }
                }
                next_states.push(State {
                    pos: String::from(p),
                    total_flow: s.total_flow,
                    open_valves: s.open_valves.clone(),
                });
            }
        }
        states = dbg!(next_states);
        println!("================");
    }

    let mut max = 0;
    for s in states {
        if s.total_flow > max  {
            max = s.total_flow;
        }
    }

    println!("Pt1: {max}"); // 1591 is too low
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