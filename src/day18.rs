use aoc::read_inputs;
use std::collections::HashSet;

type InputT = Vec<String>;

const GSIZE: usize = 25;
type GridT = [[[bool; GSIZE]; GSIZE]; GSIZE];

fn part1(input: &InputT) {
    let mut grid: GridT = [[[false; GSIZE]; GSIZE]; GSIZE];

    for line in input {
        let cube: Vec<usize> = line
            .split(',')
            .filter_map(|num| num.parse::<usize>().ok())
            .collect();
        let x = cube[0] + 1;
        let y = cube[1] + 1;
        let z = cube[2] + 1;

        grid[x][y][z] = true;
    }

    let mut exposed = 0;

    for x in 1..GSIZE - 2 {
        for y in 1..GSIZE - 2 {
            for z in 1..GSIZE - 2 {
                if !grid[x][y][z] {
                    continue;
                }

                if !grid[x - 1][y][z] {
                    exposed += 1;
                }
                if !grid[x + 1][y][z] {
                    exposed += 1;
                }
                if !grid[x][y - 1][z] {
                    exposed += 1;
                }
                if !grid[x][y + 1][z] {
                    exposed += 1;
                }
                if !grid[x][y][z - 1] {
                    exposed += 1;
                }
                if !grid[x][y][z + 1] {
                    exposed += 1;
                }
            }
        }
    }
    println!("{exposed}");
}

fn is_valid(grid: &GridT, x: usize, y: usize, z: usize) -> bool {
    x < GSIZE - 1 && y < GSIZE - 1 && z < GSIZE - 1 && !grid[x][y][z]
}

fn get_neighbors(x: usize, y: usize, z: usize) -> Vec<(usize, usize, usize)> {
    let mut ret = vec![(x + 1, y, z), (x, y + 1, z), (x, y, z + 1)];

    if x != 0 {
        ret.push((x - 1, y, z));
    }

    if y != 0 {
        ret.push((x, y - 1, z));
    }

    if z != 0 {
        ret.push((x, y, z - 1));
    }

    ret
}

fn part2(input: &InputT) {
    let mut grid: GridT = [[[false; GSIZE]; GSIZE]; GSIZE];

    for line in input {
        let cube: Vec<usize> = line
            .split(',')
            .filter_map(|num| num.parse::<usize>().ok())
            .collect();
        let x = cube[0] + 1;
        let y = cube[1] + 1;
        let z = cube[2] + 1;

        grid[x][y][z] = true;
    }

    let mut queue = vec![(0, 0, 0)];
    let mut seen = HashSet::new();
    let mut exposed = 0;

    // Flood fill algorithm
    while let Some((x, y, z)) = queue.pop() {
        let neighbors = get_neighbors(x, y, z);
        for &(nx, ny, nz) in neighbors.iter() {
            if is_valid(&grid, nx, ny, nz) && !seen.contains(&(nx, ny, nz)) {
                queue.push((nx, ny, nz));
            }
            else if grid[nx][ny][nz] {
                exposed += 1;
            }
            seen.insert((nx, ny, nz));
        }
    }
    println!("{}", exposed);
}

fn main() {
    let day: u32 = 18;
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
