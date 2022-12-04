use aoc::*;

fn get_priority(item: char) -> i32
{
    let item_ascii: i32 = item as i32;
    if item_ascii >= 97 {
        return item_ascii - 96
    }
    return item_ascii - 64 + 26
}

fn part1(input: &Vec<String>) 
{
    let mut priorities: Vec<i32> = Vec::new();
    for line in input.iter() {
        let (left, right) = line.split_at(line.len()/2);

        for i in left.chars() {
            if right.contains(i) {
                priorities.push(get_priority(i));
                break;
            }
        }
    }

    // println!("{:?}", priorities);
    let sum: i32 = priorities.iter().sum();
    println!("{0}", sum);
}

fn part2(input: &Vec<String>) 
{
    let mut priorities: Vec<i32> = Vec::new();
    for group in (0..input.len()).step_by(3) {
        // if group + 2 > input.len() { break; }

        let first_elf = &input[group];

        for i in first_elf.chars() {
            if input[group+1].contains(i) && input[group+2].contains(i) {
                priorities.push(get_priority(i));
                break;
            }
        }
    }

    // println!("{:?}", priorities);
    let sum: i32 = priorities.iter().sum();
    println!("{0}", sum);
}

fn main() {
    let day = 3;
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

    println!("PART 1:");
    part2(&input);
}
