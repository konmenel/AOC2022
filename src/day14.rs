use aoc::read_inputs;
use std::cmp::{max, min};

type InputT = Vec<String>;

type GridT = Vec<Sediment>;

const START_SAND: [usize; 2] = [500, 0];

#[derive(Debug)]
struct Grid {
    leftedge: usize,
    size: [usize; 2],
    grid: GridT,
}

impl Grid {
    fn print_grid(&self) {
        use Sediment::*;
        for (i, elem) in self.grid.iter().enumerate() {
            if i % self.size[0] == 0 && i > 0 {
                println!();
            }

            match elem {
                Air => print!("."),
                Rock => print!("#"),
                Sand => print!("o"),
            }
        }
        println!();
    }

    fn add_sand_unit(&mut self) -> Option<[usize; 2]> {
        let mut sand_pos = [
            START_SAND[0] - self.leftedge as usize,
            START_SAND[1] as usize,
        ];

        loop {
            let down = match self.get(sand_pos[0], sand_pos[1] + 1) {
                Some(elem) => elem,
                None => return None,
            };
            if *down == Sediment::Air {
                sand_pos[1] += 1;
                continue;
            }

            if sand_pos[0] == 0 {
                return None;
            }
            let downleft = match self.get(sand_pos[0] - 1, sand_pos[1] + 1) {
                Some(elem) => elem,
                None => return None,
            };
            if *downleft == Sediment::Air {
                sand_pos[0] -= 1;
                sand_pos[1] += 1;
                continue;
            }

            let downright = match self.get(sand_pos[0] + 1, sand_pos[1] + 1) {
                Some(elem) => elem,
                None => return None,
            };
            if *downright == Sediment::Air {
                sand_pos[0] += 1;
                sand_pos[1] += 1;
                continue;
            }
            break;
        }

        let idx = self.index(sand_pos[0], sand_pos[1]);
        let elem = self.grid.get_mut(idx).unwrap();
        *elem = Sediment::Sand;
        return Some(sand_pos);
    }

    fn get(&self, x: usize, y: usize) -> Option<&Sediment> {
        self.grid.get(x + y * self.size[0])
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.size[0]
    }
}

#[derive(Debug, PartialEq)]
enum Sediment {
    Air,
    Rock,
    Sand,
}

fn create_grid(input: &InputT) -> Grid {
    use Sediment::*;

    let mut ret = Grid {
        leftedge: 500,
        size: [0, 0],
        grid: Vec::new(),
    };
    let mut minx = 500usize;
    let mut maxx = 500usize;
    let mut maxy = 0usize;
    let mut lines: Vec<(Vec<usize>, Vec<usize>)> = vec![];

    for segment in input.iter() {
        let mut split = segment.split(" -> ");
        let point = split.next().unwrap();
        let mut start: Vec<usize> = point.split(',').filter_map(|x| x.parse().ok()).collect();

        while let Some(point) = split.next() {
            let end: Vec<usize> = point.split(',').filter_map(|x| x.parse().ok()).collect();

            minx = min(minx, *start.first().unwrap());
            minx = min(minx, *end.first().unwrap());
            maxx = max(maxx, *start.first().unwrap());
            maxx = max(maxx, *end.first().unwrap());
            maxy = max(maxy, *start.last().unwrap());
            maxy = max(maxy, *end.last().unwrap());

            lines.push((start, end.clone()));
            start = end;
        }
    }

    // create grid
    ret.leftedge = minx;
    ret.size = [(maxx - minx + 1) as usize, (maxy + 1) as usize];
    for _ in minx..=maxx {
        for _ in 0..=maxy {
            ret.grid.push(Air);
        }
    }

    // populate grid
    for (start, end) in lines {
        let (&x1, &x2) = (start.first().unwrap(), end.first().unwrap());
        let (&y1, &y2) = (start.last().unwrap(), end.last().unwrap());
        if x1 != x2 {
            let a = min(x1, x2);
            let b = max(x1, x2);
            let start = a - ret.leftedge + y1 * ret.size[0];
            let end = b - ret.leftedge + y1 * ret.size[0];

            for i in start..=end {
                let x = ret.grid.get_mut(i).unwrap();
                *x = Rock;
            }
        } else {
            let a = min(y1, y2);
            let b = max(y1, y2);
            let start = x1 - ret.leftedge + a * ret.size[0];
            let end = x1 - ret.leftedge + b * ret.size[0];
            for i in (start..=end).step_by(ret.size[0]) {
                let x = ret.grid.get_mut(i).unwrap();
                *x = Rock;
            }
        }
    }
    ret
}

fn create_grid2(input: &InputT) -> Grid {
    use Sediment::*;

    let mut ret = Grid {
        leftedge: 500,
        size: [0, 0],
        grid: Vec::new(),
    };
    let mut minx = 500usize;
    let mut maxx = 500usize;
    let mut maxy = 0usize;
    let mut lines: Vec<(Vec<usize>, Vec<usize>)> = vec![];

    for segment in input.iter() {
        let mut split = segment.split(" -> ");
        let point = split.next().unwrap();
        let mut start: Vec<usize> = point.split(',').filter_map(|x| x.parse().ok()).collect();

        while let Some(point) = split.next() {
            let end: Vec<usize> = point.split(',').filter_map(|x| x.parse().ok()).collect();

            minx = min(minx, *start.first().unwrap());
            minx = min(minx, *end.first().unwrap());
            maxx = max(maxx, *start.first().unwrap());
            maxx = max(maxx, *end.first().unwrap());
            maxy = max(maxy, *start.last().unwrap());
            maxy = max(maxy, *end.last().unwrap());

            lines.push((start, end.clone()));
            start = end;
        }
    }

    maxy += 2;
    minx = 0;
    maxx = 1500;
    lines.push((vec![minx, maxy], vec![maxx, maxy]));

    // create grid
    ret.leftedge = minx;
    ret.size = [maxx - minx + 1, maxy + 1];
    for _ in minx..=maxx {
        for _ in 0..=maxy {
            ret.grid.push(Air);
        }
    }

    // populate grid
    for (start, end) in lines {
        let (&x1, &x2) = (start.first().unwrap(), end.first().unwrap());
        let (&y1, &y2) = (start.last().unwrap(), end.last().unwrap());
        if x1 != x2 {
            let a = min(x1, x2);
            let b = max(x1, x2);
            let start = ret.index(a - ret.leftedge, y1);
            let end = ret.index(b - ret.leftedge, y1);

            for i in start..=end {
                let x = ret.grid.get_mut(i).unwrap();
                *x = Rock;
            }
        } else {
            let a = min(y1, y2);
            let b = max(y1, y2);
            let start = x1 - ret.leftedge + a * ret.size[0];
            let end = x1 - ret.leftedge + b * ret.size[0];
            for i in (start..=end).step_by(ret.size[0]) {
                let x = ret.grid.get_mut(i).unwrap();
                *x = Rock;
            }
        }
    }
    ret
}

fn part1(input: &InputT) {
    let mut grid = create_grid(input);
    let mut i = 0;

    while let Some(_) = grid.add_sand_unit() {
        i += 1;
        // println!("Sand unit: {}", i);
        // grid.print_grid();
    }
    println!("{}", i);
}

fn part2(input: &InputT) {
    let mut grid = create_grid2(input);
    let mut i = 0;

    while let Some(new_sand) = grid.add_sand_unit() {
        i += 1;

        // if i % 50 == 0 {
        //     grid.print_grid();
        // }

        if new_sand == [START_SAND[0] - grid.leftedge, START_SAND[1]] {
            grid.print_grid();
            println!("Last sand: {:?}", new_sand);
            break;
        }
    }
    println!("{}", i);
}

fn main() {
    let day: u32 = 14;
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
