use aoc::read_inputs;

type GridT = Vec<Vec<u32>>;


fn print_grid(grid: &GridT) {
    let nrow = grid.len();
    let ncol = grid[0].len();
    
    for row in 0..nrow {
        for col in 0..ncol {
            print!("{} ", grid[row][col]);
        }
        print!("\n");
    }
}


fn is_visible(grid: &GridT, row: usize, col: usize) -> bool {
    let nrow = grid.len();
    let ncol = grid[0].len();
    let tree_h = grid[row][col];

    let mut is_tallest = true;
    // check left
    for i in 0..row {
        is_tallest = is_tallest && (grid[i][col] < tree_h);
    }
    if is_tallest {
        return true;
    }

    let mut is_tallest = true;
    // check right
    for i in row+1..nrow {
        is_tallest = is_tallest && (grid[i][col] < tree_h);
    }
    if is_tallest {
        return true;
    }

    let mut is_tallest = true;
    // check top
    for i in 0..col {
        is_tallest = is_tallest && (grid[row][i] < tree_h);
    }
    if is_tallest {
        return true;
    }

    let mut is_tallest = true;
    // check left
    for i in col+1..ncol {
        is_tallest = is_tallest && (grid[row][i] < tree_h);
    }
    if is_tallest {
        return true;
    }

    false
}


fn get_scenic_score(grid: &GridT, row: usize, col: usize) -> u32 {
    let nrow = grid.len();
    let ncol = grid[0].len();
    let tree_h = grid[row][col];

    let mut total_score = 1;
    
    // look left
    let mut current_score = 0;
    for i in (0..row).rev() {
        current_score += 1;
        if grid[i][col] >= tree_h {
            break;
        }
    }
    total_score *= current_score;

    // look right
    let mut current_score = 0;
    for i in row+1..nrow {
        current_score += 1;
        if grid[i][col] >= tree_h {
            break;
        }
    }
    total_score *= current_score;

    // look top
    let mut current_score = 0;
    for i in (0..col).rev() {
        current_score += 1;
        if grid[row][i] >= tree_h {
            break;
        }
    }
    total_score *= current_score;

    // look bottom
    let mut current_score = 0;
    for i in col+1..ncol {
        current_score += 1;
        if grid[row][i] >= tree_h {
            break;
        }
    }
    total_score *= current_score;

    total_score
}


fn part1(grid: &GridT) {
    let nrow = grid.len();
    let ncol = grid[0].len();
    
    let mut sum = 2*nrow + 2*(ncol-2);
    for row in 1..nrow-1 {
        for col in 1..ncol-1 {
            // println!("{0}, is_visible={1}",grid[row][col],is_visible(&grid, row, col));
            if is_visible(&grid, row, col) {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}

fn part2(grid: &GridT) {
    let nrow = grid.len();
    let ncol = grid[0].len();
    
    let mut best_score = 0;
    for row in 1..nrow-1 {
        for col in 1..ncol-1 {
            let current_score = get_scenic_score(&grid, row, col);
            best_score = std::cmp::max(best_score, current_score);
            // println!("{0} score={1}", grid[row][col], current_score);
        }
    }

    println!("{}", best_score);
}

fn main() {
    let day = 08;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: Vec<String> = read_inputs(&file_path).unwrap();
    let mut grid: GridT = vec![];
    for item in input.iter() {
        grid.push(item.chars().filter_map(|c| c.to_digit(10)).collect())
    }
    // print_grid(&grid);
    println!("PART 1:");
    part1(&grid);
    println!("PART 2:");
    part2(&grid);
}
