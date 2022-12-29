use aoc::read_inputs;
use std::collections::HashMap;

type InputT = Vec<String>;

type Grid = HashMap<(i32, i32), Option<(i32, i32)>>;

const PRIORITY_LIST: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[allow(dead_code)]
fn print_grid(grid: &Grid, bb: (i32, i32, i32, i32)) {
    let (x1, y1, x2, y2) = bb;

    for y in y1..=y2 {
        for x in x1..=x2 {
            if grid.contains_key(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse_grid(input: &InputT) -> Grid {
    let mut grid = Grid::new();

    for (y, ln) in input.iter().enumerate() {
        for (x, c) in ln.chars().enumerate() {
            match c {
                '.' => continue,
                '#' => {
                    grid.insert((x as i32, y as i32), None);
                }
                _ => panic!("Unknown character during parsing `{c}`"),
            }
        }
    }

    grid
}

fn get_neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y),
        (x + 1, y - 1),
        (x, y - 1),
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
    ]
}

fn first_half(grid: &mut Grid, priority_head: usize) {
    use Direction::*;

    let keys: Vec<(i32, i32)> = grid.keys().map(|x| x.clone()).collect();
    for (elfx, elfy) in keys.into_iter() {
        let neighbors = get_neighbors(elfx, elfy);

        if neighbors.iter().all(|n| !grid.contains_key(n)) {
            continue;
        }

        for offset in 0..4 {
            match PRIORITY_LIST[(offset + priority_head) % 4] {
                North => {
                    if neighbors
                        .iter()
                        .filter(|&(_, y)| *y < elfy)
                        .all(|n| !grid.contains_key(n))
                    {
                        grid.insert((elfx, elfy), Some((elfx, elfy - 1)));
                        break;
                    }
                }
                South => {
                    if neighbors
                        .iter()
                        .filter(|&(_, y)| *y > elfy)
                        .all(|n| !grid.contains_key(n))
                    {
                        grid.insert((elfx, elfy), Some((elfx, elfy + 1)));
                        break;
                    }
                }
                West => {
                    if neighbors
                        .iter()
                        .filter(|&(x, _)| *x < elfx)
                        .all(|n| !grid.contains_key(n))
                    {
                        grid.insert((elfx, elfy), Some((elfx - 1, elfy)));
                        break;
                    }
                }
                East => {
                    if neighbors
                        .iter()
                        .filter(|&(x, _)| *x > elfx)
                        .all(|n| !grid.contains_key(n))
                    {
                        grid.insert((elfx, elfy), Some((elfx + 1, elfy)));
                        break;
                    }
                }
            }
        }
    }
}

fn second_half(grid: &mut Grid) {
    let clone = grid.clone();
    for (&(elfx, elfy), &pos) in clone.iter() {
        if pos == None {
            continue;
        }

        let is_unique = !clone
            .iter()
            .filter(|(key, _)| *key != &(elfx, elfy))
            .any(|(_, &v)| v == pos);
        if is_unique {
            let new_pos = pos.unwrap();
            grid.remove(&(elfx, elfy));
            grid.insert(new_pos, None);
        }
    }

    // cleanup
    for (_, val) in grid.iter_mut() {
        *val = None
    }
}

fn bounding_box(grid: &Grid) -> (i32, i32, i32, i32) {
    let x1 = grid.iter().map(|(&(x, _), _)| x).min().unwrap();
    let y1 = grid.iter().map(|(&(_, y), _)| y).min().unwrap();
    let x2 = grid.iter().map(|(&(x, _), _)| x).max().unwrap();
    let y2 = grid.iter().map(|(&(_, y), _)| y).max().unwrap();

    (x1, y1, x2, y2)
}

fn part1(input: &InputT) {
    let mut grid = parse_grid(input);

    #[cfg(debug_assertions)]
    const BB: (i32, i32, i32, i32) = (0, 0, 13, 11);

    let num_elves = grid.len();

    #[cfg(debug_assertions)]
    print_grid(&grid, BB);

    for i in 0..10 {
        first_half(&mut grid, i);
        second_half(&mut grid);

        #[cfg(debug_assertions)]
        {
            println!("\n== End of Round {} ==", i + 1);
            print_grid(&grid, BB);
            println!();
        }
    }

    let (x1, y1, x2, y2) = bounding_box(&grid);
    let area = (x2 - x1 + 1) *  (y2 - y1 + 1);
    let sum = area - num_elves as i32;
    #[cfg(debug_assertions)]
    {
        print_grid(&grid, (x1, y1, x2, y2));
        println!("{:?}", (x1, y1, x2, y2));
    }
    println!("{sum}");
}

fn keys_match(
    map1: &HashMap<(i32, i32), Option<(i32, i32)>>, 
    map2: &HashMap<(i32, i32), Option<(i32, i32)>>, 
) -> bool {
    map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
}

fn part2(input: &InputT) {
    let mut grid = parse_grid(input);

    #[cfg(debug_assertions)]
    const BB: (i32, i32, i32, i32) = (0, 0, 13, 11);

    #[cfg(debug_assertions)]
    print_grid(&grid, BB);

    let mut round = 0;
    let mut old;

    loop {
        old = grid.clone();
        first_half(&mut grid, round);
        second_half(&mut grid);

        #[cfg(debug_assertions)]
        {
            println!("\n== End of Round {} ==", round);
            print_grid(&grid, BB);
            println!();
        }
        round += 1;
        if keys_match(&old, &grid) {
            break;
        }
    }

    println!("{round}");
}

fn main() {
    let day: u32 = 23;
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
