use aoc::read_inputs;


fn part1(input: &Vec<String>) {
    let mut x = 1;
    let mut signal_strength_sum   = 0;
    let mut input_iter = input.iter();
    let mut instruction = input_iter.next().unwrap();
    let mut cycle_to_read: Option<i32> = None;
    let mut cycle_to_add: Option<i32> = None;
    let mut add_number: Option<i32> = None;
    
    for cycle in 1..221 {
        if (cycle + 20) % 40 == 0 {
            // println!("Cycle {}: signal strenth = {}", cycle, x*cycle);
            signal_strength_sum += x * cycle;
        }
        
        if Some(cycle) == cycle_to_read {
            instruction = input_iter.next().unwrap();
        }
        
        // println!("Cycle {}: x={}    {}", cycle, x, instruction);
        let mut split = instruction.split(' ');
        let command = split.next().unwrap();

        if Some(cycle) == cycle_to_add {
            x += add_number.unwrap();
            add_number = None;
            cycle_to_add = None;
            cycle_to_read = Some(cycle + 1);
            continue;
        }

        if command == "noop" {
            cycle_to_read = Some(cycle + 1);
        }

        if command == "addx" {
            if add_number.is_none() {
                add_number = Some(split.next().unwrap().parse::<i32>().unwrap());
                cycle_to_add = Some(cycle+1);
            }
        }
    }
    println!("{}", signal_strength_sum);
}


fn part2(input: &Vec<String>) {
    let mut x = 1;
    let mut input_iter = input.iter();
    let mut instruction = input_iter.next().unwrap();
    let mut cycle_to_read: Option<i32> = None;
    let mut cycle_to_add: Option<i32> = None;
    let mut add_number: Option<i32> = None;
    let mut crt_pos = 0;
    
    for cycle in 1..40*6+1 {
        if x-1 <= crt_pos && crt_pos <= x +1 {
            print!("#");
        }
        else {
            print!(".");
        }        
        crt_pos = (crt_pos + 1) % 40;
        if crt_pos % 40 == 0 {
            println!();
        }
        
        if Some(cycle) == cycle_to_read {
            instruction = input_iter.next().unwrap();
        }
        
        // println!("Cycle {}: x={}    {}", cycle, x, instruction);
        let mut split = instruction.split(' ');
        let command = split.next().unwrap();

        if Some(cycle) == cycle_to_add {
            x += add_number.unwrap();
            add_number = None;
            cycle_to_add = None;
            cycle_to_read = Some(cycle + 1);
            continue;
        }

        if command == "noop" {
            cycle_to_read = Some(cycle + 1);
        }

        if command == "addx" {
            if add_number.is_none() {
                add_number = Some(split.next().unwrap().parse::<i32>().unwrap());
                cycle_to_add = Some(cycle+1);
            }
        }
    }
}


fn main() {
    let day: u32 = 10;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: Vec<String> = read_inputs(&file_path).unwrap();
    println!("PART 1:");
    part1(&input);
    println!("PART 2:");
    part2(&input);
}

