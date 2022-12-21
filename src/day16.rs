use aoc::read_inputs;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::collections::{HashMap, VecDeque};

type InputT = Vec<String>;
type TunnelMapT = HashMap<String, Node>;
type BitsetT = i32; // < 32-bit set more than enough to store all the non-zero vavles

#[derive(Debug, Clone, PartialEq)]
struct Node {
    name: String,
    flow_rate: i32,
    neighbors: Vec<String>,
}

fn parse_scan(input: &InputT) -> TunnelMapT {
    let re: Regex = Regex::new(
        "Valve (?P<name>[A-Z]{2}) has flow rate=(?P<flow_rate>[0-9]+); tunnel[s]? lead[s]? to valve[s]? (?P<neighbors>[,/ A-Z]*)"
    ).unwrap();

    let mut tunnel_system = HashMap::new();

    for line in input.iter() {
        let matches = re.captures(line).unwrap();

        let node = Node {
            name: matches["name"].to_string(),
            flow_rate: matches["flow_rate"].parse().ok().unwrap(),
            neighbors: matches["neighbors"]
                .split(", ")
                .map(|x| x.to_string())
                .collect(),
        };

        tunnel_system.insert(matches["name"].to_string(), node);
    }

    tunnel_system
}

fn max_press(
    map: &TunnelMapT,
    indeces: &HashMap<String, i32>,
    nonzero_nodes: &Vec<&str>,
    head: &str,
    time: i32,
    opened_valves: BitsetT
) -> i32 {
    // use std::sync::Mutex;
    // lazy_static! {
    //     static ref CACHE_PRESS: Mutex<HashMap<(String, i32, BitsetT), i32>> = Mutex::new(HashMap::new());
    // }
    // {
    // let cache = CACHE_PRESS.lock().unwrap();
    // let key = (head.to_string(), time, opened_valves);
    // if cache.contains_key(&key) {
    //     return cache[&key];
    // }}
    let mut ret = 0;

    for &neighbor in nonzero_nodes.iter() {
        let valve = &map[neighbor];
        let dist = find_closest_dist(&map, head, neighbor);
        let bit = 1 << indeces[neighbor];
        if opened_valves & bit != 0 {
            continue;
        }

        let dt = dist + 1;
        if time < dt {
            continue;
        }

        let press = max_press(
            map,
            indeces,
            nonzero_nodes,
            neighbor,
            time - dt,
            opened_valves | bit,
        );

        ret = max(ret, press + (time - dt) * valve.flow_rate);
    }

    // {
    // let mut cache = CACHE_PRESS.lock().unwrap();
    // let key = (head.to_string(), time, opened_valves);
    // cache.insert(key, ret);
    // }
    ret
}

fn find_closest_dist(map: &TunnelMapT, start: &str, end: &str) -> i32 {
    use std::sync::Mutex;
    lazy_static! {
        static ref CACHE_DIST: Mutex<HashMap<(String, String), i32>> = Mutex::new(HashMap::new());
    }

    let mut cache = CACHE_DIST.lock().unwrap();
    if cache.contains_key(&(start.to_string(), end.to_string())) {
        return cache[&(start.to_string(), end.to_string())];
    }

    let mut ret = 0;
    let mut seen: Vec<&Node> = vec![];
    let mut queue: VecDeque<(i32, &Node)> = VecDeque::new();

    {
        let start = &map[start];
        let end = &map[end];

        queue.push_back((0, start));
        while let Some((dist, node)) = queue.pop_front() {
            if node == end {
                ret = dist;
                break;
            }

            for neighbor in node.neighbors.iter() {
                let neighbor = &map[neighbor];

                if !seen.contains(&neighbor) {
                    seen.push(neighbor);
                    queue.push_back((dist + 1, neighbor));
                }
            }
        }
    }

    cache.insert((start.to_string(), end.to_string()), ret);
    ret
}

fn part1(input: &InputT) {
    const TIME: i32 = 30;
    let map = parse_scan(input);

    let head = "AA";

    let mut nonzero_nodes = vec![];
    let mut indeces: HashMap<String, i32> = HashMap::new();

    // Get non-zero nodes
    let mut i = 0;
    for (key, val) in map.iter() {
        if val.flow_rate != 0 {
            nonzero_nodes.push(&key[..]);
            indeces.insert(key.to_string(), i);
            i += 1;
        }
    }
    let pressure = max_press(&map, &indeces, &nonzero_nodes, head, TIME, 0);

    println!("{}", pressure);
}

fn part2(input: &InputT) {
    const TIME: i32 = 26;
    let map = parse_scan(input);

    let head = "AA";

    let mut nonzero_nodes = vec![];
    let mut indeces: HashMap<String, i32> = HashMap::new();

    // Get non-zero nodes
    let mut i = 0;
    for (key, val) in map.iter() {
        if val.flow_rate != 0 {
            nonzero_nodes.push(&key[..]);
            indeces.insert(key.to_string(), i);
            i += 1;
        }
    }

    let max_bitset = (1 << nonzero_nodes.len()) - 1;
    let mut pressure = 0;

    for me in 0..max_bitset/2 {
        let eleph = max_bitset ^ me;
        let press_me = max_press(&map, &indeces, &nonzero_nodes, head, TIME, me);
        let press_eleph = max_press(&map, &indeces, &nonzero_nodes, head, TIME, eleph);
        pressure = max(pressure, press_eleph + press_me);
    }

    println!("{}", pressure);
}

fn main() {
    let day: u32 = 16;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: InputT = read_inputs(&file_path).unwrap();
    println!("PART 1:");
    part1(&input);
    println!("PART 2:");
    part2(&input);
}
