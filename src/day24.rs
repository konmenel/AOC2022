use aoc::read_inputs;
use num::complex::Complex;
use std::collections::{HashSet, HashMap, VecDeque};
use std::fmt;
use std::ops::Index;

type InputT = Vec<String>;

const UP: Complex<i32> = Complex::new(0, -1);
const DOWN: Complex<i32> = Complex::new(0, 1);
const LEFT: Complex<i32> = Complex::new(-1, 0);
const RIGHT: Complex<i32> = Complex::new(1, 0);

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    data: HashMap<Complex<i32>, usize>,
    bliz: Vec<(Complex<i32>, Complex<i32>)>, // pos, dir
    nrows: usize,
    ncols: usize,
    end_pos: Complex<i32>,
}

impl Grid {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            bliz: vec![],
            nrows: 0,
            ncols: 0,
            end_pos: Complex::new(0, 0),
        }
    }

    fn get(&self, x: usize, y: usize) -> &usize {
        &self.data[&Complex::new(x as i32, y as i32)]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut usize {
        self.data
            .get_mut(&Complex::new(x as i32, y as i32))
            .unwrap()
    }
}

impl Index<&Complex<i32>> for Grid {
    type Output = usize;

    fn index(&self, key: &Complex<i32>) -> &Self::Output {
        &self.data[key]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.nrows {
            for x in 0..self.ncols {
                if is_wall(
                    Complex {
                        re: x as i32,
                        im: y as i32,
                    },
                    self.ncols as i32,
                    self.nrows as i32,
                ) {
                    write!(f, "#")?;
                    continue;
                }
                let n = self.get(x, y);
                match n {
                    0 => write!(f, ".")?,
                    1 => {
                        for bliz in self.bliz.iter().filter(|&(p, _)| p.re == x as i32 && p.im == y as i32) { 
                            match bliz.1 {
                                UP => write!(f, "^")?,
                                DOWN => write!(f, "v")?,
                                RIGHT => write!(f, ">")?,
                                LEFT => write!(f, "<")?,
                                e => panic!("Imposible bliz direction `{e}` at pos `{:?}`", (x, y)),
                            }
                        }
                    },
                    mult => write!(f, "{mult}")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        Ok(())
    }
}


fn parse_grid(input: &InputT) -> Grid {
    let mut grid = Grid::new();
    grid.nrows = input.len();
    grid.ncols = input[0].len();
    grid.end_pos = Complex::new((grid.ncols - 2) as i32, (grid.nrows - 1) as i32);

    grid.data.insert(Complex::new(1, 0), 0);
    grid.data.insert(
        Complex::new((grid.ncols - 2) as i32, (grid.nrows - 1) as i32),
        0,
    );
    for (y, ln) in input.iter().enumerate() {
        for (x, c) in ln.chars().enumerate() {
            let pos = Complex::new(x as i32, y as i32);
            grid.data.insert(pos, 0);
            match c {
                '>' => {
                    grid.bliz.push((pos, RIGHT));
                    *grid.get_mut(x, y) += 1;
                },
                '<' => {
                    grid.bliz.push((pos, LEFT));
                    *grid.get_mut(x, y) += 1;
                },
                '^' => {
                    grid.bliz.push((pos, UP));
                    *grid.get_mut(x, y) += 1;
                },
                'v' => {
                    grid.bliz.push((pos, DOWN));
                    *grid.get_mut(x, y) += 1;
                },
                '.' | '#' => continue,
                _ => panic!("Unknown character during parsing `{c}`"),
            }
        }
    }
    grid
}

#[inline]
fn is_wall(val: Complex<i32>, w: i32, h: i32) -> bool {
    !(val == Complex::new(1, 0) || val == Complex::new(w - 2, h - 1))
        && (val.im == 0 || val.re == 0 || val.re == w - 1 || val.im == h - 1)
}

fn get_neighbors(pos: Complex<i32>, h: i32, w: i32) -> Vec<Complex<i32>> {
    let n = [pos + UP, pos + DOWN, pos + RIGHT, pos + LEFT];

    n.into_iter()
        .filter(|&x| x.re >= 0 && x.im >= 0 && x.re < w && x.im < h && !is_wall(x, w, h))
        .collect()
}

fn update_bliz(grid: &Grid) -> Grid {
    let pos: Vec<Complex<i32>> = grid
        .bliz
        .iter()
        .map(|(x, _)| x.to_owned())
        .collect();

    let mut ret = grid.clone();
    for (i, p) in pos.into_iter().enumerate() {
        let h = ret.nrows as i32;
        let w = ret.ncols as i32;

        let dir = grid.bliz[i].1;
        let mut new_pos = p + dir;
        if is_wall(new_pos, w, h) {
            match dir {
                UP => new_pos.im = h - 2,
                DOWN => new_pos.im = 1,
                RIGHT => new_pos.re = 1,
                LEFT => new_pos.re = w - 2,
                e => panic!("Imposible bliz direction `{e}` at pos `{p}`"),
            }
        }
        ret.bliz[i] = (new_pos, dir);
        *ret.data.get_mut(&p).unwrap() -= 1;
        *ret.data.get_mut(&new_pos).unwrap() += 1;
    }
    ret
}

fn find_all_grids(grid: Grid) -> Vec<Grid> {
    let repeat = (grid.nrows - 2) * (grid.ncols - 2);
    let mut ret = vec![grid];
    ret.reserve(repeat - 1);

    for i in 0..repeat - 1 {
        let new_grid = update_bliz(&ret[i]);
        ret.push(new_grid);
    }
    ret
}

fn dijkstra(grids: &Vec<Grid>, start: Complex<i32>, end: Complex<i32>, steps: usize) -> usize {
    let h = grids[0].nrows as i32;
    let w = grids[0].ncols as i32;
    
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let neighbors: Vec<Complex<i32>> = get_neighbors(start, h, w)
        .into_iter()
        .chain([start].into_iter())
        .filter(|x| grids[(steps + 1) % grids.len()][x] == 0)
        .collect();
    
    for n in neighbors {
        queue.push_back((n, steps + 1));
        seen.insert((n, steps + 1));
    }
    
    while let Some((pos, steps)) = queue.pop_front() {
        if pos == end {
            return steps;
        }

        let neighbors: Vec<Complex<i32>> = get_neighbors(pos, h, w)
            .into_iter()
            .chain([pos].into_iter())
            .filter(|x| grids[(steps + 1) % grids.len()][x] == 0)
            .collect();
        
        for n in neighbors {
            if seen.insert((n, steps + 1)) {
                queue.push_back((n, steps + 1));
            }
        }
    }

    steps
}

fn part1(input: &InputT) {
    let grid = parse_grid(input);
    let grids = find_all_grids(grid);
    let start = Complex::new(1, 0);

    let steps = dijkstra(&grids, start, grids[0].end_pos, 0);
    println!("{steps}");
}

fn part2(input: &InputT) {
    let grid = parse_grid(input);
    let grids = find_all_grids(grid);
    let start = Complex::new(1, 0);
    let end = grids[0].end_pos;

    let stepsp1 = dijkstra(&grids, start, end, 0);
    let stepsp2 =  dijkstra(&grids, end, start, stepsp1);
    let total = dijkstra(&grids, start, end, stepsp2);
    println!("{total}");
}

fn main() {
    let day: u32 = 24;
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
