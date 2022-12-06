use aoc::read_inputs;

fn are_all_char_different(slice: &str) -> bool {
    for i in 0..slice.len() {
        for j in i+1..slice.len() {
            if slice.chars().nth(i).unwrap() == slice.chars().nth(j).unwrap() { 
                return false; 
            }
        }
    }

    return true;
}


fn part1(input: &String) {
    let mut char_idx: usize = 4;
    while char_idx < input.len() {
        if are_all_char_different(&input[char_idx-4..char_idx]) {
            break;
        }
        char_idx += 1;
    }

    println!("{}", char_idx);
}

fn part2(input: &String) {
    let mut char_idx: usize = 14;
    while char_idx < input.len() {
        if are_all_char_different(&input[char_idx-14..char_idx]) {
            break;
        }
        char_idx += 1;
    }

    println!("{}", char_idx);
}

fn main() {
    let day = 06;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: Vec<String> = read_inputs(&file_path).unwrap();
    println!("PART 1:");
    part1(&input[0]);
    println!("PART 2:");
    part2(&input[0]);
}
