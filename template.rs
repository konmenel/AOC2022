use aoc::read_inputs;

type InputT = Vec<String>;

fn part1(input: &InputT) {
    
}

fn main() {
    let day: u32 = XX;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: InputT = read_inputs(&file_path).unwrap();
    println!("PART 1:");
    part1(&input);
    println!("PART 2:");
    // part2(&input);
}
