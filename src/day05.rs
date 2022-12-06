use aoc::read_inputs;


fn part1(input: &Vec<String>) 
{
    let mut boxes: Vec<String> = Vec::new();
    let mut moves_idx = 0;
    for (i, line) in input.iter().enumerate() {
        if line == "" {
            moves_idx = i + 1;
            break;
        }
        boxes.push(line.to_string());
    }

    let nbox: usize = boxes.last().unwrap()
            .chars().rev().nth(1).unwrap()
            .to_digit(10).unwrap()
            .try_into().unwrap();
    
    let mut box_stacks: Vec<Vec<char>> = Vec::with_capacity(nbox);
    for _ in 0..nbox {
        box_stacks.push(vec![]);
    }

    for row in (0..boxes.len()-1).rev() {
        let mut i = 0;
        while i < (boxes[row].len() + 1)/4 {
            let box_letter = boxes[row].chars().nth(4*i + 1).unwrap();
            if box_letter != ' ' {
                box_stacks[i].push(box_letter);
            }
            i += 1;
        }
    }
    // println!("{:?}", box_stacks);

    for line in input[moves_idx..].iter() {
        let move_op: Vec<usize> = line.split(" ")
                .filter_map(|num| usize::from_str_radix(num, 10).ok())
                .collect();
        
        // println!("{:?}", move_op);

        for _ in 0..move_op[0] {
            let box_unloaded = box_stacks[move_op[1]-1].pop().unwrap();
            box_stacks[move_op[2]-1].push(box_unloaded);
        }
        
        // println!("{:?}", box_stacks)
    }

    for i in box_stacks.iter() {
        print!("{0}", i.last().unwrap())
    }
    println!();
}


fn part2(input: &Vec<String>) 
{
    let mut boxes: Vec<String> = Vec::new();
    let mut moves_idx = 0;
    for (i, line) in input.iter().enumerate() {
        if line == "" {
            moves_idx = i + 1;
            break;
        }
        boxes.push(line.to_string());
    }

    let nbox: usize = boxes.last().unwrap()
            .chars().rev().nth(1).unwrap()
            .to_digit(10).unwrap()
            .try_into().unwrap();
    
    let mut box_stacks: Vec<Vec<char>> = Vec::with_capacity(nbox);
    for _ in 0..nbox {
        box_stacks.push(vec![]);
    }

    for row in (0..boxes.len()-1).rev() {
        let mut i = 0;
        while i < (boxes[row].len() + 1)/4 {
            let box_letter = boxes[row].chars().nth(4*i + 1).unwrap();
            if box_letter != ' ' {
                box_stacks[i].push(box_letter);
            }
            i += 1;
        }
    }
    // println!("{:?}", box_stacks);

    for line in input[moves_idx..].iter() {
        let move_op: Vec<usize> = line.split(" ")
                .filter_map(|num| usize::from_str_radix(num, 10).ok())
                .collect();
        
        // println!("{:?}", move_op);

        let unload_range = (box_stacks[move_op[1]-1].len()-move_op[0])..;
        let boxes_unloaded: Vec<char> = box_stacks[move_op[1]-1].drain(unload_range).collect();
        box_stacks[move_op[2]-1].extend_from_slice(&boxes_unloaded);
        
        // println!("{:?}", box_stacks)
    }

    for i in box_stacks.iter() {
        print!("{0}", i.last().unwrap())
    }
    println!();
}

fn main() {
    let day = 5;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: Vec<String> = match read_inputs(&file_path) {
        Ok(input) => input,
        Err(e) => panic!("Error parsing the inputs: {:?}", e)
    };
    println!("PART 1:");
    part1(&input);
    println!("PART 2:");
    part2(&input);
}
