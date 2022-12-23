use std::collections::HashSet;

use aoc::read_inputs;

type InputT = Vec<String>;

#[allow(dead_code)]
fn print_rock(rock: &HashSet<(i64, i64)>, grid: &HashSet<(i64, i64)>) {
    let max_y = rock.iter().map(|&(_, y)| y).max().unwrap();
    for y in (0..=max_y).rev() {
        for x in 0..7 {
            if rock.contains(&(x, y)) {
                print!("@");
            } else if grid.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn side_mv(grid: &HashSet<(i64, i64)>, jet: char, rock: &mut HashSet<(i64, i64)>) {
    let mv = match jet {
        '>' => 1,
        '<' => -1,
        c => panic!("Unknown move character `{c}`"),
    };

    let mut moved_rock = HashSet::new();

    for r in rock.iter() {
        let x = r.0 + mv;
        let y = r.1;
        moved_rock.insert((x, y));
    }

    if moved_rock.iter().any(|&(x, _)| x < 0 || x > 6) {
        return;
    }

    if !grid.is_disjoint(&moved_rock) {
        return;
    }

    *rock = moved_rock;
}

fn down_mv(grid: &HashSet<(i64, i64)>, rock: &mut HashSet<(i64, i64)>) -> bool {
    let mut moved_rock = HashSet::new();

    for r in rock.iter() {
        let x = r.0;
        let y = r.1 - 1;
        moved_rock.insert((x, y));
    }

    if moved_rock.iter().any(|&(_, y)| y < 0) {
        return false;
    }

    if !grid.is_disjoint(&moved_rock) {
        return false;
    }

    *rock = moved_rock;
    true
}

fn offset_rock(rock: &mut HashSet<(i64, i64)>, x_off: i64, y_off: i64) {
    let new_rock: HashSet<(i64, i64)> = rock.iter().map(|(x, y)| (x + x_off, y + y_off)).collect();
    *rock = new_rock;
}

fn part1(input: &InputT) {
    // Shapes order = _, +, L,, I, []
    let shapes: [HashSet<(i64, i64)>; 5] = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)].into_iter().collect(),
        vec![(1, 0), (0, 1), (2, 1), (1, 2)].into_iter().collect(),
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]
            .into_iter()
            .collect(),
        vec![(0, 0), (0, 1), (0, 2), (0, 3)].into_iter().collect(),
        vec![(0, 0), (1, 0), (0, 1), (1, 1)].into_iter().collect(),
    ];
    const END: usize = 2022;
    const NSHAPES: usize = 5;

    let jets = &input[0];
    let jets_sz = jets.len();

    let mut stopped_rocks = 0;
    let mut grid: HashSet<(i64, i64)> = HashSet::new();
    let mut rock = shapes[stopped_rocks % NSHAPES].clone();
    offset_rock(&mut rock, 2, 3);

    let mut i = 0;
    let mut highest_point = 0;

    while stopped_rocks < END {
        let jet = jets.chars().nth(i % jets_sz).unwrap();
        side_mv(&grid, jet, &mut rock);

        if !down_mv(&grid, &mut rock) {
            stopped_rocks += 1;
            grid.extend(&rock);
            highest_point = grid.iter().map(|&(_, y)| y).max().unwrap() + 1;
            rock = shapes[stopped_rocks % NSHAPES].clone();
            offset_rock(&mut rock, 2, highest_point + 3);

            // print_rock(&rock, &grid);
            // println!();
        }
        i += 1;
    }
    println!("{highest_point}");
}

fn are_silces_eq(s1: &[i64], s2: &[i64]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut iter1 = s1.iter();
    let mut iter2 = s2.iter();

    while let (Some(&i), Some(&j)) = (iter1.next(), iter2.next()) {
        if i != j {
            return false;
        }
    }

    true
}

fn part2(input: &InputT) {
    // Shapes order = _, +, L,, I, []
    let shapes: [HashSet<(i64, i64)>; 5] = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)].into_iter().collect(),
        vec![(1, 0), (0, 1), (2, 1), (1, 2)].into_iter().collect(),
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]
            .into_iter()
            .collect(),
        vec![(0, 0), (0, 1), (0, 2), (0, 3)].into_iter().collect(),
        vec![(0, 0), (1, 0), (0, 1), (1, 1)].into_iter().collect(),
    ];
    const NSHAPES: usize = 5;
    const END: usize = 1000000000000;

    let jets = &input[0];
    let jets_sz = jets.len();

    let mut stopped_rocks = 0;
    let mut grid: HashSet<(i64, i64)> = HashSet::new();
    let mut rock = shapes[stopped_rocks % NSHAPES].clone();
    offset_rock(&mut rock, 2, 3);

    let mut i = 0;
    let mut highest_point = 0;
    let mut dh = vec![];
    let mut extra = 0;
    let mut repeat_len = 0;

    'outer: while stopped_rocks < END {
        let jet = jets.chars().nth(i % jets_sz).unwrap();
        side_mv(&grid, jet, &mut rock);

        if !down_mv(&grid, &mut rock) {
            stopped_rocks += 1;
            grid.extend(&rock);
            let h1 = highest_point;
            let h2 = grid.iter().map(|&(_, y)| y).max().unwrap() + 1;
            dh.push(h2 - h1);
            highest_point = h2;
            rock = shapes[stopped_rocks % NSHAPES].clone();
            offset_rock(&mut rock, 2, highest_point + 3);
        }

        let max_slice_len = dh.len() / 3;
        for k in 0..max_slice_len {
            let hsize = (dh.len() - k) / 2;
            if are_silces_eq(&dh[k..k+hsize], &dh[k+hsize..]) {
                repeat_len = hsize;
                extra = k;
                break 'outer ;
            }
        }
        i += 1;
    }

    let repeat = ((END - extra) / repeat_len) as i64;
    let mod_extra = (END - extra) % repeat_len;

    let initial_height = dh[..extra].iter().sum::<i64>();
    let repeated_height = dh[extra..extra+repeat_len].iter().sum::<i64>() * repeat;
    let mod_height = dh[extra..extra+mod_extra].iter().sum::<i64>();

    highest_point = initial_height + repeated_height + mod_height;

    println!("{highest_point}");
}

fn main() {
    let day: u32 = 17;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: InputT = read_inputs(&file_path).unwrap();
    println!("PART 1:");
    part1(&input);
    println!("PART 2:");
    part2(&input)
}
