use std::cmp::Ordering;
use std::str::Split;

use aoc::read_inputs;

type InputT = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Number(i32),
    List(Vec<Value>),
}

fn parse_packet(string: &str) -> Vec<Value> {
    let mut split = string[1..string.len() - 1].split(',');
    let mut ret = vec![];
    while let Some(token) = split.next() {
        if token.is_empty() {
            break;
        }
        ret.push(parse_value(&mut split, token));
    }
    ret
}

fn parse_value(split: &mut Split<char>, first_token: &str) -> Value {
    if first_token.starts_with('[') {
        return Value::List(parse_list(split, &first_token[1..]));
    }

    match first_token.parse::<i32>() {
        Ok(num) => Value::Number(num),
        Err(_) => Value::List(vec![]),
    }
}

fn parse_list(split: &mut Split<char>, first_token: &str) -> Vec<Value> {
    let mut ret = vec![];
    if first_token.starts_with('[') {
        ret.push(parse_value(split, first_token));
    } else if first_token.contains(']') {
        let end = first_token.find(']').unwrap();
        ret.push(parse_value(split, &first_token[..end]));
        return ret;
    } else {
        ret.push(parse_value(split, &first_token));
    }

    while let Some(token) = split.next() {
        if token.contains(']') {
            let end = token.find(']').unwrap();
            ret.push(parse_value(split, &token[..end]));
            break;
        }

        ret.push(parse_value(split, &token));
    }
    ret
}

fn check_right_order(left: &Vec<Value>, right: &Vec<Value>) -> Option<bool> {
    // println!("{:?}", left);
    // println!("{:?}", right);
    // println!();
    let mut right_iter = right.iter();

    for lval in left.iter() {
        let rval = right_iter.next();
        match rval {
            None => return Some(false),
            Some(rval) => match (lval, rval) {
                (Value::Number(lnum), Value::Number(rnum)) => {
                    if lnum < rnum {
                        return Some(true);
                    } else if lnum > rnum {
                        return Some(false);
                    }
                }
                (Value::List(llist), Value::List(rlist)) => {
                    if let Some(comp) = check_right_order(&llist, &rlist) {
                        return Some(comp);
                    }
                }
                (Value::List(llist), rnum) => {
                    let rlist = vec![rnum.clone()];
                    if let Some(comp) = check_right_order(&llist, &rlist) {
                        return Some(comp);
                    }
                }
                (lnum, Value::List(rlist)) => {
                    let llist = vec![lnum.clone()];
                    if let Some(comp) = check_right_order(&llist, &rlist) {
                        return Some(comp);
                    }
                }
            },
        }
    }
    if left.len() == right.len() {
        return None;
    }

    Some(true)
}

fn part1(input: &InputT) {
    let mut iter = input.iter();
    let mut sum = 0;
    let mut i = 1;

    while let (Some(left), Some(right)) = (iter.next(), iter.next()) {
        iter.next(); // empty line
        let left = parse_packet(&left);
        let right = parse_packet(&right);

        if check_right_order(&left, &right) == Some(true) {
            // println!("{}", i);
            // println!("{:?}", left);
            // println!("{:?}", right);
            sum += i;
        }
        i += 1;
    }
    println!("{}", sum);
}

fn part2(input: &InputT) {
    let mut iter = input.iter();
    let mut pockets = vec![];

    let distress2 = vec![Value::List(vec![Value::Number(2)])];
    let distress6 = vec![Value::List(vec![Value::Number(6)])];

    pockets.push(distress2.clone());
    pockets.push(distress6.clone());

    while let (Some(left), Some(right)) = (iter.next(), iter.next()) {
        iter.next(); // empty line
        let left = parse_packet(&left);
        let right = parse_packet(&right);

        pockets.push(left);
        pockets.push(right);
    }
    pockets.sort_by(|a, b| {
        if check_right_order(a, b).unwrap() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut decoder: i32 = pockets.iter().position(|x| *x == distress2).unwrap() as i32 + 1;
    decoder *= pockets.iter().position(|x| *x == distress6).unwrap() as i32 + 1;

    println!("{}", decoder);
}

fn main() {
    let day: u32 = 13;
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
