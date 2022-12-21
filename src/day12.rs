use aoc::read_inputs;
use std::collections::VecDeque;
use std::vec;

type InputT = Vec<String>;

fn get_neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut ret = vec![];

    if x + 1 < width {
        ret.push(((x + 1), (y + 0)));
    }
    if x > 0 {
        ret.push(((x - 1), (y - 0)));
    }
    if y + 1 < height {
        ret.push(((x + 0), (y + 1)));
    }
    if y > 0 {
        ret.push(((x - 0), (y - 1)));
    }

    ret
}

fn part1(input: &InputT) {
    let mut queue: VecDeque<(u32, usize, usize)> = VecDeque::new();
    let mut seen: Vec<(usize, usize)> = vec![];
    let mut grid: Vec<Vec<i8>> = vec![];

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for (i, row) in input.iter().enumerate() {
        grid.push(vec![]);
        for (j, col) in row.chars().enumerate() {
            let mut height = col;
            if height == 'S' {
                start = (i, j);
                height = 'a';
            } else if height == 'E' {
                end = (i, j);
                height = 'z';
            }
            grid[i].push(height as i8);
        }
    }

    queue.push_back((0, start.0, start.1));

    while let Some((distance, i, j)) = queue.pop_front() {
        if (i, j) == end {
            println!("{}", distance);
            break;
        }

        for neigh in get_neighbors(i, j, grid.len(), grid[0].len()) {
            let my_height = grid[i][j];
            let neighbor_height = grid[neigh.0][neigh.1];

            if neighbor_height - my_height <= 1 && !seen.contains(&(neigh.0, neigh.1)) {
                seen.push((neigh.0, neigh.1));
                queue.push_back((distance + 1, neigh.0, neigh.1));
            }
        }
    }
}

fn part2(input: &InputT) {
    let mut queue: VecDeque<(u32, usize, usize)> = VecDeque::new();
    let mut seen: Vec<(usize, usize)> = vec![];
    let mut grid: Vec<Vec<i8>> = vec![];

    let mut starts: Vec<(usize, usize)> = vec![];
    let mut end: (usize, usize) = (0, 0);
    let mut min_steps = u32::MAX;

    for (i, row) in input.iter().enumerate() {
        grid.push(vec![]);
        for (j, col) in row.chars().enumerate() {
            let mut height = col;
            if height == 'S' || height == 'a' {
                starts.push((i, j));
                height = 'a';
            } else if height == 'E' {
                end = (i, j);
                height = 'z';
            }

            grid[i].push(height as i8);
        }
    }

    // println!("{:?}", starts);

    for start in starts {
        seen.clear();
        queue.clear();
        queue.push_back((0, start.0, start.1));

        while let Some((distance, i, j)) = queue.pop_front() {
            if (i, j) == end {
                min_steps = std::cmp::min(min_steps, distance);
                break;
            }

            for neigh in get_neighbors(i, j, grid.len(), grid[0].len()) {
                let my_height = grid[i][j];
                let neighbor_height = grid[neigh.0][neigh.1];

                if neighbor_height - my_height <= 1 && !seen.contains(&(neigh.0, neigh.1)) {
                    seen.push((neigh.0, neigh.1));
                    queue.push_back((distance + 1, neigh.0, neigh.1));
                }
            }
        }
    }
    println!("{}", min_steps);
}

fn main() {
    let day: u32 = 12;
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
