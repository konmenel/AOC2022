use aoc::read_inputs;


fn part1<T>(input: &Vec<T>) {

}

fn main() {
    let day = XX;
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
}
