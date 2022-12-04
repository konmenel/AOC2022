use aoc::*;


fn part1<T>(input: Vec<T>) {

}

fn main() {
    let input: Vec<String> = match read_inputs("data/XX.txt") {
        Ok(input) => input,
        Err(e) => panic!("Error parsing the inputs: {:?}", e)
    };
    part1(input);
}


#[cfg(test)]
mod test {
    use crate::read_inputs;
    use crate::part1;

    #[test]
    fn reading_test() {
        let input: Vec<i32> = match read_inputs("data/examples/XX.txt") {
            Ok(input) => input,
            Err(e) => panic!("Parsing input file error: {:?}", e),
        };

        part1(input);
    }
}