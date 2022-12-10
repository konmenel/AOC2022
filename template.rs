use aoc::read_inputs;


fn part1(input: &Vec<String>) {

}

fn main() {
    let day = XX;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: Vec<String> = read_inputs(&file_path).unwrap();
    println!("PART 1:");
    part1(&input);
    println!("PART 2:");
}
