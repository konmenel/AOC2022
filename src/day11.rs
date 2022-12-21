use aoc::read_inputs;

type InputT = Vec<String>;

#[derive(Debug)]
struct Monkey<T> {
    items: Vec<T>,    // Items' held worry level
    op: (char, u32),  // Operation and number (e.g. ('*', 19))
    test_div: u32,    // Number to test divisibility
    if_true: usize,   // Index of monkey to throw to if true
    if_false: usize,  // Index of monkey to throw to if false
    inspections: u64, // Number of inspections
}

impl<T> Monkey<T> {
    fn new() -> Self {
        Self {
            items: vec![],
            op: ('*', 0),
            test_div: 0,
            if_true: 0,
            if_false: 0,
            inspections: 0,
        }
    }
}

fn create_monkey_list<T: std::str::FromStr>(input: &InputT) -> Vec<Monkey<T>> {
    let mut monkeys: Vec<Monkey<T>> = vec![];
    let mut iter = input.iter();

    while let Some(line) = iter.next() {
        if line.contains("Monkey") {
            let mut monkey = Monkey::new();

            // Starting items parse
            let mut line = iter.next().unwrap();
            let (_, items_str) = line.split_once(": ").unwrap();
            for item in items_str.split(", ") {
                monkey.items.push(item.parse::<T>().ok().unwrap());
            }

            // Operation parse
            line = iter.next().unwrap();
            let (_, op_str) = line.split_once("old ").unwrap();
            let split = op_str.split_once(' ').unwrap();
            monkey.op.0 = split.0.chars().nth(0).unwrap();
            if let Some(num) = split.1.parse::<u32>().ok() {
                monkey.op.1 = num;
            } else {
                // `new = old * old` case (square)
                monkey.op.0 = '^';
                monkey.op.1 = 2;
            }

            // Test parse
            line = iter.next().unwrap();
            let (_, test_div_str) = line.split_once("by ").unwrap();
            monkey.test_div = test_div_str.parse().ok().unwrap();

            // If true parse
            line = iter.next().unwrap();
            let (_, if_true_str) = line.split_once("monkey ").unwrap();
            monkey.if_true = if_true_str.parse().ok().unwrap();

            // If false parse
            line = iter.next().unwrap();
            let (_, if_false_str) = line.split_once("monkey ").unwrap();
            monkey.if_false = if_false_str.parse().ok().unwrap();

            monkeys.push(monkey)
        }
    }
    monkeys
}

fn part1(input: &InputT) {
    let mut monkeys: Vec<Monkey<u32>> = create_monkey_list(input);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys.get(i).unwrap().items.is_empty() {
                let mut item = monkeys.get_mut(i).unwrap().items.remove(0);

                {
                    let monkey = monkeys.get(i).unwrap();
                    item = match monkey.op.0 {
                        '+' => item + monkey.op.1 as u32,
                        '*' => item * monkey.op.1 as u32,
                        '/' => item / monkey.op.1 as u32,
                        '^' => item * item,
                        _ => 0,
                    };

                    item /= 3;
                }
                let test_div = monkeys.get(i).unwrap().test_div;
                let if_true = monkeys.get(i).unwrap().if_true;
                let if_false = monkeys.get(i).unwrap().if_false;

                if item % test_div == 0 {
                    monkeys.get_mut(if_true).unwrap().items.push(item);
                } else {
                    monkeys.get_mut(if_false).unwrap().items.push(item);
                }
                monkeys.get_mut(i).unwrap().inspections += 1;
            }
        }
    }

    let mut inspections = Vec::from_iter(monkeys.iter().map(|mon| mon.inspections));
    inspections.sort_by(|a, b| b.cmp(a));
    println!("{}", inspections[0] * inspections[1]);
}

fn part2(input: &InputT) {
    let mut monkeys: Vec<Monkey<u64>> = create_monkey_list(input);

    #[cfg(debug_assertions)]
    let mut largest_num: u64 = 1;
    #[cfg(debug_assertions)]
    for monkey in monkeys.iter() {
        largest_num *= monkey.test_div as u64;
    }

    #[cfg(not(debug_assertions))]
    const largest_num: u64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19; // <-- release only

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while !monkeys.get(i).unwrap().items.is_empty() {
                let mut item = monkeys.get_mut(i).unwrap().items.remove(0);

                {
                    let monkey = monkeys.get(i).unwrap();
                    item = match monkey.op.0 {
                        '+' => item + monkey.op.1 as u64,
                        '*' => item * monkey.op.1 as u64,
                        '^' => item * item,
                        _ => 0,
                    };

                    item = item % largest_num;
                }
                let test_div = monkeys.get(i).unwrap().test_div;
                let if_true = monkeys.get(i).unwrap().if_true;
                let if_false = monkeys.get(i).unwrap().if_false;

                if item % test_div as u64 == 0 {
                    monkeys.get_mut(if_true).unwrap().items.push(item);
                } else {
                    monkeys.get_mut(if_false).unwrap().items.push(item);
                }
                monkeys.get_mut(i).unwrap().inspections += 1;
            }
        }
    }

    let mut inspections = Vec::from_iter(monkeys.iter().map(|mon| mon.inspections));
    // println!("{:?}", inspections);
    inspections.sort_by(|a, b| b.cmp(a));
    println!("{}", inspections[0] * inspections[1]);
}

fn main() {
    let day: u32 = 11;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: InputT = read_inputs(&file_path).unwrap();
    println!("PART 1:");
    part1(&input);
    println!("PART 2:");
    part2(&input)
}
