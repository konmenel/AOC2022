use aoc::read_inputs;
use num::complex::Complex;
use std::collections::HashMap;
use std::fmt;
use std::ops::Index;

type InputT = Vec<String>;

const UP: Complex<i32> = Complex::new(0, -1);
const DOWN: Complex<i32> = Complex::new(0, 1);
const RIGHT: Complex<i32> = Complex::new(1, 0);
const LEFT: Complex<i32> = Complex::new(-1, 0);

const ROT0: Complex<i32> = Complex::new(1, 0);
const ROT90: Complex<i32> = Complex::new(0, 1);
const ROT180: Complex<i32> = Complex::new(-1, 0);
const ROT270: Complex<i32> = Complex::new(0, -1);

#[cfg(not(debug_assertions))]
const FACE_SZ: usize = 50;
#[cfg(debug_assertions)]
const FACE_SZ: usize = 4;

#[derive(Debug, PartialEq, Clone)]
enum Square {
    Air,
    Rock,
    Edge,
}

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<Square>,
    nrows: usize,
    ncols: usize,
}

#[allow(dead_code)]
impl Grid {
    fn new() -> Self {
        Self {
            data: vec![],
            nrows: 0,
            ncols: 0,
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.nrows, self.ncols)
    }

    fn push(&mut self, val: Square) {
        self.data.push(val);
    }

    fn get(&self, x: usize, y: usize) -> &Square {
        &self.data[(self.ncols + 2) * (y + 1) + x + 1]
    }

    fn get_cmx(&self, num: Complex<i32>) -> &Square {
        let x = num.re + 1;
        let y = num.im + 1;
        &self.data[(self.ncols + 2) * y as usize + x as usize]
    }

    fn iter(&self) -> std::slice::Iter<Square> {
        self.data.iter()
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Square;

    fn index(&self, indices: (usize, usize)) -> &Self::Output {
        let (x, y) = indices;
        self.get(x, y)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.nrows {
            for x in 0..self.ncols {
                match self[(x, y)] {
                    Square::Air => write!(f, ".")?,
                    Square::Rock => write!(f, "#")?,
                    Square::Edge => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Cube {
    faces: [Grid; 6],

    // Hashmap of key UP,DOWN,LEFT,RIGHT and index of neighbor and rotation (complex)
    neighbors: [HashMap<Complex<i32>, (usize, Complex<i32>)>; 6],
}

impl Cube {
    fn new() -> Self {
        let grid = Grid::new();
        let neigh = HashMap::new();
        Self {
            faces: [
                grid.clone(),
                grid.clone(),
                grid.clone(),
                grid.clone(),
                grid.clone(),
                grid.clone(),
            ],
            neighbors: [
                neigh.clone(),
                neigh.clone(),
                neigh.clone(),
                neigh.clone(),
                neigh.clone(),
                neigh.clone(),
            ],
        }
    }
}

fn create_grid(input: &InputT) -> Grid {
    let mut grid = Grid::new();

    grid.nrows = input.len() - 2;
    grid.ncols = input[0..=grid.nrows].iter().map(|x| x.len()).max().unwrap();

    grid.data.extend((0..grid.ncols + 2).map(|_| Square::Edge));

    for ln in input[0..=grid.nrows].iter() {
        grid.push(Square::Edge);
        for c in ln.chars() {
            match c {
                '.' => grid.push(Square::Air),
                '#' => grid.push(Square::Rock),
                ' ' => grid.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        grid.push(Square::Edge);

        // Fill with Edges
        while grid.data.len() % (grid.ncols + 2) > 0 {
            grid.push(Square::Edge);
        }
    }
    grid.data.extend((0..grid.nrows + 2).map(|_| Square::Edge));

    grid
}

fn parse_path(path: &str) -> (Vec<usize>, Vec<Complex<i32>>) {
    let mut steps = vec![];
    let mut turns = vec![];
    let mut i_last = 0;
    for (i, c) in path.chars().enumerate() {
        match c {
            'R' | 'L' => {
                let substr = &path[i_last..i];
                turns.push(Complex::new(0, 1 - 2 * ('L' == c) as i32));
                steps.push(substr.parse::<usize>().unwrap());
                i_last = i + 1;
            }
            _ => continue,
        }
    }

    let substr = &path[i_last..];
    match substr.parse::<usize>() {
        Ok(num) => steps.push(num),
        Err(_) => (),
    };
    (steps, turns)
}

fn update_pos(
    grid: &Grid,
    pos: Complex<i32>,
    direction: Complex<i32>,
    steps: usize,
) -> Complex<i32> {
    use Square::*;

    let mut pos = pos;

    for _ in 0..steps {
        // println!("{pos}");
        let mut new_pos = direction + pos;
        match grid.get_cmx(new_pos) {
            Air => pos = new_pos,
            Rock => break,
            Edge => {
                match direction {
                    UP => {
                        let x = new_pos.re as usize;
                        let mut i = grid.nrows - 1;
                        while grid[(x, i)] == Edge {
                            i -= 1;
                        }

                        if grid[(x, i)] == Rock {
                            break;
                        }
                        new_pos.im = i as i32;
                    }
                    DOWN => {
                        let x = new_pos.re as usize;
                        let mut i = 0;
                        while grid[(x, i)] == Edge {
                            i += 1;
                        }

                        if grid[(x, i)] == Rock {
                            break;
                        }
                        new_pos.im = i as i32;
                    }
                    RIGHT => {
                        let y = new_pos.im as usize;
                        let mut i = 0;
                        while grid[(i, y)] == Edge {
                            i += 1;
                        }

                        if grid[(i, y)] == Rock {
                            break;
                        }
                        new_pos.re = i as i32;
                    }
                    LEFT => {
                        let y = new_pos.im as usize;
                        let mut i = grid.ncols;
                        while grid[(i, y)] == Edge {
                            i -= 1;
                        }

                        if grid[(i, y)] == Rock {
                            break;
                        }
                        new_pos.re = i as i32;
                    }
                    e => panic!("Impossible direction `{}`", e),
                }
                pos = new_pos;
            }
        }
    }
    pos
}

fn part1(input: &InputT) {
    let grid = create_grid(input);
    #[cfg(debug_assertions)]
    println!("{}", grid);

    let mut direction = Complex::new(1, 0);
    let start = grid.iter().position(|x| x == &Square::Air).unwrap() - grid.ncols - 3;

    // Position: x + yj (real part == x, imag part == y)
    let mut pos = Complex::new(start as i32, 0);

    let path = &input[input.len() - 1];
    let (steps, turns) = parse_path(path);

    let mut turn_iter = turns.iter();
    for step in steps {
        pos = update_pos(&grid, pos, direction, step);

        if let Some(&turn) = turn_iter.next() {
            direction *= turn;
        }
        #[cfg(debug_assertions)]
        {
            println!("pos = {}", pos + Complex::new(1, 1));
            println!("direction = {}", direction);
        }
    }

    let mut sum = 0;
    if direction == DOWN {
        sum = 1;
    } else if direction == LEFT {
        sum = 2;
    } else if direction == UP {
        sum = 3;
    }

    sum += 1000 * (pos.im + 1) + 4 * (pos.re + 1);

    #[cfg(debug_assertions)]
    {
        println!("final pos = {}", pos + Complex::new(1, 1));
        println!("final direction = {}", direction);
    }

    println!("{sum}");
}

#[cfg(debug_assertions)]
fn create_cube(input: &InputT) -> Cube {
    let mut cube = Cube::new();

    // FACE 0
    let mut face = Grid::new();
    face.ncols = FACE_SZ;
    face.nrows = FACE_SZ;
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    for ln in &input[..FACE_SZ] {
        face.push(Square::Edge);
        for c in (&ln[2 * FACE_SZ..3 * FACE_SZ]).chars() {
            match c {
                '.' => face.push(Square::Air),
                '#' => face.push(Square::Rock),
                ' ' => face.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        face.push(Square::Edge);
    }
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    cube.faces[0] = face;
    let mut neighbors = HashMap::new();
    neighbors.insert(UP, (1, DOWN));
    neighbors.insert(DOWN, (3, DOWN));
    neighbors.insert(RIGHT, (5, DOWN));
    neighbors.insert(LEFT, (2, DOWN));
    cube.neighbors[0] = neighbors;

    // FACE 1
    let mut face = Grid::new();
    face.ncols = FACE_SZ;
    face.nrows = FACE_SZ;
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    for ln in &input[FACE_SZ..2 * FACE_SZ] {
        face.push(Square::Edge);
        for c in (&ln[..FACE_SZ]).chars() {
            match c {
                '.' => face.push(Square::Air),
                '#' => face.push(Square::Rock),
                ' ' => face.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        face.push(Square::Edge);
    }
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    cube.faces[1] = face;
    let mut neighbors = HashMap::new();
    neighbors.insert(UP, (0, RIGHT));
    neighbors.insert(DOWN, (5, UP));
    neighbors.insert(RIGHT, (2, RIGHT));
    neighbors.insert(LEFT, (5, DOWN));
    cube.neighbors[1] = neighbors;

    // FACE 2
    let mut face = Grid::new();
    face.ncols = FACE_SZ;
    face.nrows = FACE_SZ;
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    for ln in &input[FACE_SZ..2 * FACE_SZ] {
        face.push(Square::Edge);
        for c in (&ln[FACE_SZ..2 * FACE_SZ]).chars() {
            match c {
                '.' => face.push(Square::Air),
                '#' => face.push(Square::Rock),
                ' ' => face.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        face.push(Square::Edge);
    }
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    cube.faces[2] = face;
    let mut neighbors = HashMap::new();
    neighbors.insert(UP, (0, RIGHT));
    neighbors.insert(DOWN, (4, LEFT));
    neighbors.insert(RIGHT, (3, RIGHT));
    neighbors.insert(LEFT, (1, LEFT));
    cube.neighbors[2] = neighbors;

    // FACE 3
    let mut face = Grid::new();
    face.ncols = FACE_SZ;
    face.nrows = FACE_SZ;
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    for ln in &input[FACE_SZ..2 * FACE_SZ] {
        face.push(Square::Edge);
        for c in (&ln[2 * FACE_SZ..3 * FACE_SZ]).chars() {
            match c {
                '.' => face.push(Square::Air),
                '#' => face.push(Square::Rock),
                ' ' => face.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        face.push(Square::Edge);
    }
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    cube.faces[3] = face;
    let mut neighbors = HashMap::new();
    neighbors.insert(UP, (0, UP));
    neighbors.insert(DOWN, (4, UP));
    neighbors.insert(RIGHT, (5, DOWN));
    neighbors.insert(LEFT, (2, LEFT));
    cube.neighbors[3] = neighbors;

    // FACE 4
    let mut face = Grid::new();
    face.ncols = FACE_SZ;
    face.nrows = FACE_SZ;
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    for ln in &input[2 * FACE_SZ..3 * FACE_SZ] {
        face.push(Square::Edge);
        for c in (&ln[2 * FACE_SZ..3 * FACE_SZ]).chars() {
            match c {
                '.' => face.push(Square::Air),
                '#' => face.push(Square::Rock),
                ' ' => face.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        face.push(Square::Edge);
    }
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    cube.faces[4] = face;
    let mut neighbors = HashMap::new();
    neighbors.insert(UP, (3, UP));
    neighbors.insert(DOWN, (1, UP));
    neighbors.insert(RIGHT, (5, RIGHT));
    neighbors.insert(LEFT, (2, UP));
    cube.neighbors[4] = neighbors;

    // FACE 5
    let mut face = Grid::new();
    face.ncols = FACE_SZ;
    face.nrows = FACE_SZ;
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    for ln in &input[2 * FACE_SZ..3 * FACE_SZ] {
        face.push(Square::Edge);
        for c in (&ln[3 * FACE_SZ..4 * FACE_SZ]).chars() {
            match c {
                '.' => face.push(Square::Air),
                '#' => face.push(Square::Rock),
                ' ' => face.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        face.push(Square::Edge);
    }
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    cube.faces[5] = face;
    let mut neighbors = HashMap::new();
    neighbors.insert(UP, (3, LEFT));
    neighbors.insert(DOWN, (1, RIGHT));
    neighbors.insert(RIGHT, (0, LEFT));
    neighbors.insert(LEFT, (4, LEFT));
    cube.neighbors[5] = neighbors;

    cube
}

#[cfg(not(debug_assertions))]
fn create_cube(input: &InputT) -> Cube {
    let mut cube = Cube::new();

    // FACE 0
    let mut face = Grid::new();
    face.ncols = FACE_SZ;
    face.nrows = FACE_SZ;
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    for ln in &input[..FACE_SZ] {
        face.push(Square::Edge);
        for c in (&ln[FACE_SZ..2 * FACE_SZ]).chars() {
            match c {
                '.' => face.push(Square::Air),
                '#' => face.push(Square::Rock),
                ' ' => face.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        face.push(Square::Edge);
    }
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    cube.faces[0] = face;
    let mut neighbors = HashMap::new();
    neighbors.insert(UP, (5, RIGHT));
    neighbors.insert(DOWN, (2, DOWN));
    neighbors.insert(RIGHT, (1, RIGHT));
    neighbors.insert(LEFT, (4, RIGHT));
    cube.neighbors[0] = neighbors;

    // FACE 1
    let mut face = Grid::new();
    face.ncols = FACE_SZ;
    face.nrows = FACE_SZ;
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    for ln in &input[..FACE_SZ] {
        face.push(Square::Edge);
        for c in (&ln[2 * FACE_SZ..3 * FACE_SZ]).chars() {
            match c {
                '.' => face.push(Square::Air),
                '#' => face.push(Square::Rock),
                ' ' => face.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        face.push(Square::Edge);
    }
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    cube.faces[1] = face;
    let mut neighbors = HashMap::new();
    neighbors.insert(UP, (5, UP));
    neighbors.insert(DOWN, (2, LEFT));
    neighbors.insert(RIGHT, (3, LEFT));
    neighbors.insert(LEFT, (0, LEFT));
    cube.neighbors[1] = neighbors;

    // FACE 2
    let mut face = Grid::new();
    face.ncols = FACE_SZ;
    face.nrows = FACE_SZ;
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    for ln in &input[FACE_SZ..2 * FACE_SZ] {
        face.push(Square::Edge);
        for c in (&ln[FACE_SZ..2 * FACE_SZ]).chars() {
            match c {
                '.' => face.push(Square::Air),
                '#' => face.push(Square::Rock),
                ' ' => face.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        face.push(Square::Edge);
    }
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    cube.faces[2] = face;
    let mut neighbors = HashMap::new();
    neighbors.insert(UP, (0, UP));
    neighbors.insert(DOWN, (3, DOWN));
    neighbors.insert(RIGHT, (1, UP));
    neighbors.insert(LEFT, (4, DOWN));
    cube.neighbors[2] = neighbors;

    // FACE 3
    let mut face = Grid::new();
    face.ncols = FACE_SZ;
    face.nrows = FACE_SZ;
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    for ln in &input[2 * FACE_SZ..3 * FACE_SZ] {
        face.push(Square::Edge);
        for c in (&ln[FACE_SZ..2 * FACE_SZ]).chars() {
            match c {
                '.' => face.push(Square::Air),
                '#' => face.push(Square::Rock),
                ' ' => face.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        face.push(Square::Edge);
    }
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    cube.faces[3] = face;
    let mut neighbors = HashMap::new();
    neighbors.insert(UP, (2, UP));
    neighbors.insert(DOWN, (5, LEFT));
    neighbors.insert(RIGHT, (1, LEFT));
    neighbors.insert(LEFT, (4, LEFT));
    cube.neighbors[3] = neighbors;

    // FACE 4
    let mut face = Grid::new();
    face.ncols = FACE_SZ;
    face.nrows = FACE_SZ;
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    for ln in &input[2 * FACE_SZ..3 * FACE_SZ] {
        face.push(Square::Edge);
        for c in (&ln[..FACE_SZ]).chars() {
            match c {
                '.' => face.push(Square::Air),
                '#' => face.push(Square::Rock),
                ' ' => face.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        face.push(Square::Edge);
    }
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    cube.faces[4] = face;
    let mut neighbors = HashMap::new();
    neighbors.insert(UP, (2, RIGHT));
    neighbors.insert(DOWN, (5, DOWN));
    neighbors.insert(RIGHT, (3, RIGHT));
    neighbors.insert(LEFT, (0, RIGHT));
    cube.neighbors[4] = neighbors;

    // FACE 5
    let mut face = Grid::new();
    face.ncols = FACE_SZ;
    face.nrows = FACE_SZ;
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    for ln in &input[3 * FACE_SZ..4 * FACE_SZ] {
        face.push(Square::Edge);
        for c in (&ln[..FACE_SZ]).chars() {
            match c {
                '.' => face.push(Square::Air),
                '#' => face.push(Square::Rock),
                ' ' => face.push(Square::Edge),
                _ => panic!("Unknown character `{c}` when parsing grid"),
            }
        }
        face.push(Square::Edge);
    }
    face.data.extend((0..face.ncols + 2).map(|_| Square::Edge));
    cube.faces[5] = face;
    let mut neighbors = HashMap::new();
    neighbors.insert(UP, (4, UP));
    neighbors.insert(DOWN, (1, DOWN));
    neighbors.insert(RIGHT, (3, UP));
    neighbors.insert(LEFT, (0, DOWN));
    cube.neighbors[5] = neighbors;

    cube
}

#[cfg(not(debug_assertions))]
fn rel_to_abs(pos: Complex<i32>, face_idx: usize) -> Complex<i32> {
    match face_idx {
        0 => pos + Complex::new(FACE_SZ as i32, 0),
        1 => pos + Complex::new(2 * FACE_SZ as i32, 0),
        2 => pos + Complex::new(FACE_SZ as i32, FACE_SZ as i32),
        3 => pos + Complex::new(FACE_SZ as i32, 2 * FACE_SZ as i32),
        4 => pos + Complex::new(0, 2 * FACE_SZ as i32),
        5 => pos + Complex::new(0, 3 * FACE_SZ as i32),
        e => panic!("Impossible face `{e}`"),
    }
}

#[cfg(debug_assertions)]
fn rel_to_abs(pos: Complex<i32>, face_idx: usize) -> Complex<i32> {
    match face_idx {
        0 => pos + Complex::new(2 * FACE_SZ as i32, 0),
        1 => pos + Complex::new(0, FACE_SZ as i32),
        2 => pos + Complex::new(FACE_SZ as i32, FACE_SZ as i32),
        3 => pos + Complex::new(2 * FACE_SZ as i32, FACE_SZ as i32),
        4 => pos + Complex::new(2 * FACE_SZ as i32, 2 * FACE_SZ as i32),
        5 => pos + Complex::new(2 * FACE_SZ as i32, 3 * FACE_SZ as i32),
        e => panic!("Impossible face `{e}`"),
    }
}

fn update_cube_pos(
    cube: &Cube,
    face_idx: usize,
    pos: Complex<i32>,
    direction: &mut Complex<i32>,
    steps: usize,
) -> (Complex<i32>, usize) {
    use Square::*;

    let mut pos = pos;
    let mut face_idx = face_idx;

    for _ in 0..steps {
        // println!("{pos}");
        let face = &cube.faces[face_idx];
        let mut new_pos = *direction + pos;
        match face.get_cmx(new_pos) {
            Air => pos = new_pos,
            Rock => break,
            Edge => {
                let (neighbor, new_dir) = (&cube.neighbors[face_idx])[&*direction];
                let rot = *direction / new_dir;
                match rot {
                    ROT0 => match *direction {
                        UP => new_pos.im = (FACE_SZ - 1) as i32,
                        DOWN => new_pos.im = 0i32,
                        RIGHT => new_pos.re = 0i32,
                        LEFT => new_pos.re = (FACE_SZ - 1) as i32,
                        e => panic!("Impossible direction `{e}`"),
                    },
                    ROT90 => match *direction {
                        UP => {
                            new_pos.im = (FACE_SZ - 1) as i32 - new_pos.re;
                            new_pos.re = (FACE_SZ - 1) as i32;
                        }
                        DOWN => {
                            new_pos.im = (FACE_SZ - 1) as i32 - new_pos.re;
                            new_pos.re = 0i32;
                        }
                        RIGHT => {
                            new_pos.re = new_pos.im;
                            new_pos.im = (FACE_SZ - 1) as i32;
                        }
                        LEFT => {
                            new_pos.re = new_pos.im;
                            new_pos.im = 0i32;
                        }
                        e => panic!("Impossible direction `{e}`"),
                    },
                    ROT180 => match *direction {
                        UP => {
                            new_pos.re = (FACE_SZ - 1) as i32 - new_pos.re;
                            new_pos.im = 0i32;
                        }
                        DOWN => {
                            new_pos.re = (FACE_SZ - 1) as i32 - new_pos.re;
                            new_pos.im = (FACE_SZ - 1) as i32;
                        }
                        RIGHT => {
                            new_pos.re = (FACE_SZ - 1) as i32;
                            new_pos.im = (FACE_SZ - 1) as i32 - new_pos.im;
                        }
                        LEFT => {
                            new_pos.re = 0i32;
                            new_pos.im = (FACE_SZ - 1) as i32 - new_pos.im;
                        }
                        e => panic!("Impossible direction `{e}`"),
                    },
                    ROT270 => match *direction {
                        UP => {
                            new_pos.im = new_pos.re;
                            new_pos.re = 0i32;
                        }
                        DOWN => {
                            new_pos.im = new_pos.re;
                            new_pos.re = (FACE_SZ - 1) as i32;
                        }
                        RIGHT => {
                            new_pos.re = (FACE_SZ - 1) as i32 - new_pos.im;
                            new_pos.im = 0i32;
                        }
                        LEFT => {
                            new_pos.re = (FACE_SZ - 1) as i32 - new_pos.im;
                            new_pos.im = (FACE_SZ - 1) as i32;
                        }
                        e => panic!("Impossible direction `{e}`"),
                    },
                    e => panic!("Impossible rotation `{e}`"),
                }

                if *cube.faces[neighbor].get_cmx(new_pos) == Rock {
                    break;
                }

                face_idx = neighbor;
                *direction = new_dir;
                pos = new_pos;
            }
        }
    }
    (pos, face_idx)
}

fn part2(input: &InputT) {
    let cube = create_cube(input);

    let mut direction = Complex::new(1, 0);
    let mut face = 0;

    // Position: x + yj (real part == x, imag part == y)
    let mut pos = Complex::new(0, 0);

    let path = &input[input.len() - 1];
    let (steps, turns) = parse_path(path);

    let mut turn_iter = turns.iter();
    for step in steps {
        #[cfg(debug_assertions)]
        {
            println!("\nsteps = {}", step);
            println!("face = {}", face);
            println!("pos = {}", pos + Complex::new(1, 1));
            println!("direction = {}", direction);
        }

        (pos, face) = update_cube_pos(&cube, face, pos, &mut direction, step);

        if let Some(&turn) = turn_iter.next() {
            direction *= turn;
        }
    }

    let mut sum = 0;
    if direction == DOWN {
        sum = 1;
    } else if direction == LEFT {
        sum = 2;
    } else if direction == UP {
        sum = 3;
    }

    pos = rel_to_abs(pos, face);
    sum += 1000 * (pos.im + 1) + 4 * (pos.re + 1);

    #[cfg(debug_assertions)]
    {
        println!("\nfinal pos = {}", pos + Complex::new(1, 1));
        println!("final direction = {}", direction);
        println!("final face = {}", face);
    }

    println!("{sum}");
}

fn main() {
    let day: u32 = 22;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: InputT = read_inputs(&file_path).unwrap();
    println!("PART 1:");
    part1(&input);
    println!("\nPART 2:");
    part2(&input);
}
