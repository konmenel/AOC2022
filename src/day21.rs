use aoc::read_inputs;
use std::collections::HashMap;

type InputT = Vec<String>;

#[derive(Debug)]
enum Op<'a> {
    Num(f64),
    Expr(&'a str, char, &'a str)
}

fn create_map<'a>(input: &'a InputT) -> HashMap<&'a str, Op> {
    let mut map = HashMap::new();

    for ln in input {
        let (name, val) = ln.split_once(": ").unwrap();

        let val = match val.parse::<f64>() {
            Ok(num) => Op::Num(num),
            Err(_) => {
                let mut split = val.split(' ');
                let lhs = split.next().unwrap();
                let operation = split.next().unwrap().chars().nth(0).unwrap();
                let rhs = split.next().unwrap();
                Op::Expr(lhs, operation, rhs)
            }
        };
        map.insert(name, val);
    }
    map
}

fn find_val(map: &HashMap<&str, Op>, name: &str) -> f64 {
    let val = &map[name];

    match val {
        Op::Num(num) => *num,
        Op::Expr(lhs, op, rhs) => {
            match op {
                '+' => find_val(map, *lhs) + find_val(map, *rhs),
                '-' => find_val(map, *lhs) - find_val(map, *rhs),
                '*' => find_val(map, *lhs) * find_val(map, *rhs),
                '/' => find_val(map, *lhs) / find_val(map, *rhs),
                c => panic!("Unknown arithmetic oparation `{}`", c)
            }
        }
    }
}

fn part1(input: &InputT) {
    let map = create_map(input);

    let root = find_val(&map, "root");
    println!("{}", root);
}

fn contains_me(map: &HashMap<&str, Op>, root: &str) -> bool {
    if root == "humn" {
        return true;
    } 

    let val = &map[root];

    match val {
        Op::Num(_) => false,
        Op::Expr(rhs, _, lhs) => contains_me(map, *rhs) || contains_me(map, *lhs)
    }
}

fn find_my_val(map: &HashMap<&str, Op>, name: &str, target: f64) -> f64 {
    if name == "humn" {
        return target;
    }
    let val = &map[name];

    match val {
        Op::Num(num) => *num,
        Op::Expr(lhs, op, rhs) => {
            let is_lhs_me = contains_me(map, *lhs);
            match op {
                '+' => {
                    if is_lhs_me {
                        let new_target = target - find_val(map, *rhs);
                        find_my_val(map, *lhs, new_target)
                    } else {
                        let new_target = target - find_val(map, *lhs);
                        find_my_val(map, *rhs, new_target)
                    }
                },
                '-' => {
                    if is_lhs_me {
                        let new_target = target + find_val(map, *rhs);
                        find_my_val(map, *lhs, new_target)
                    } else {
                        let new_target = find_val(map, *lhs) - target;
                        find_my_val(map, *rhs, new_target)
                    }
                },
                '*' => {
                    if is_lhs_me {
                        let new_target = target / find_val(map, *rhs);
                        find_my_val(map, *lhs, new_target)
                    } else {
                        let new_target = target / find_val(map, *lhs);
                        find_my_val(map, *rhs, new_target)
                    }
                },
                '/' => {
                    if is_lhs_me {
                        let new_target = target * find_val(map, *rhs);
                        find_my_val(map, *lhs, new_target)
                    } else {
                        let new_target = find_val(map, *lhs) / target;
                        find_my_val(map, *rhs, new_target)
                    }
                },
                c => panic!("Unknown arithmetic oparation `{}`", c)
            }
        }
    }
}

fn part2(input: &InputT) {
    let mut map = create_map(input);
    let root = &map["root"];

    if let Op::Expr(rhs, _, lhs) = root {
        map.insert("root", Op::Expr(*rhs, '-', *lhs));
    }

    let me = find_my_val(&map, "root", 0.0);
    println!("{}", me);
}

fn main() {
    let day: u32 = 21;
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

