use std::collections::btree_set::Difference;

use aoc::read_inputs;

type InputT = Vec<String>;

#[derive(Debug)]
struct File {
    sequence: Vec<(usize, i64)>,
}

impl File {
    fn from_vec(vec: &InputT) -> Self {
        Self {
            sequence: (0..vec.len())
                .zip(vec.iter().filter_map(|x| x.parse().ok()))
                .collect(),
        }
    }

    fn get_idx(&self, idx: usize) -> Option<usize> {
        for (i, &(j, _)) in self.sequence.iter().enumerate() {
            if j == idx {
                return Some(i);
            }
        }
        None
    }
}

fn part1(input: &InputT) {
    let mut file = File::from_vec(&input);
    let len = file.sequence.len();

    for i in 0..file.sequence.len() {
        let idx = file.get_idx(i).unwrap();
        let elem = file.sequence.remove(idx);

        let mut new_pos = (idx as i64 + elem.1) % (len - 1) as i64;
        if new_pos <= 0 {
            new_pos = len as i64 + new_pos - 1;
        }
        let new_pos = new_pos as usize;
        file.sequence.insert(new_pos, elem);
        // println!("{:?}", file);
    }


    let zero_val = file.sequence.iter().position(|&(_, x)| x == 0).unwrap();

    let digit1 = file.sequence[(zero_val + 1000) % file.sequence.len()].1;
    let digit2 = file.sequence[(zero_val + 2000) % file.sequence.len()].1;
    let digit3 = file.sequence[(zero_val + 3000) % file.sequence.len()].1;

    // println!("{:?}", [digit1, digit2, digit3]);
    println!("{}", digit1 + digit2 + digit3);
}

fn part2(input: &InputT) {
    const KEY: i64 = 811589153;
    let mut file = File::from_vec(&input);
    let len = file.sequence.len();
    file.sequence = file.sequence.iter().map(|&(i, x)| (i, KEY*x)).collect();

    for _ in 0..10 {
        for i in 0..file.sequence.len() {
            let idx = file.get_idx(i).unwrap();
            let elem = file.sequence.remove(idx);
    
            let mut new_pos = (idx as i64 + elem.1) % (len - 1) as i64;
            if new_pos <= 0 {
                new_pos = len as i64 + new_pos - 1;
            }
            let new_pos = new_pos as usize;
            file.sequence.insert(new_pos, elem);
            // println!("{:?}", file);
        }
    }

    let zero_val = file.sequence.iter().position(|&(_, x)| x == 0).unwrap();

    let digit1 = file.sequence[(zero_val + 1000) % file.sequence.len()].1;
    let digit2 = file.sequence[(zero_val + 2000) % file.sequence.len()].1;
    let digit3 = file.sequence[(zero_val + 3000) % file.sequence.len()].1;

    // println!("{:?}", [digit1, digit2, digit3]);
    println!("{}", digit1 + digit2 + digit3);
}

fn main() {
    let day: u32 = 20;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: InputT = read_inputs(&file_path).unwrap();
    println!("PART 1:");
    part1(&input);
    println!("PART 2:");
    part2(&input);
}
