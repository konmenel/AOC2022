use std::str::FromStr;

pub fn read_inputs<T>(file_path: &str) -> std::io::Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(file_path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

#[cfg(test)]
mod test {
    use crate::read_inputs;

    #[test]
    fn reading_test() {
        let input: Vec<i32> = match read_inputs("data/examples/01.txt") {
            Ok(input) => input,
            Err(e) => panic!("Parsing 01.txt error: {:?}", e),
        };

        println!("{:?}", input);
    }
}
