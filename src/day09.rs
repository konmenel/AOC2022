use aoc::read_inputs;

#[derive(PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}

fn fix_tail_pos(tail: &mut Position, head: &Position) {
    let rel_posx = head.x - tail.x;
    let rel_posy = head.y - tail.y;
    
    if tail.y == head.y {
        if rel_posx > 1 {
            tail.x += 1
        } 
        else if rel_posx < -1 {
            tail.x -= 1;
        }
    }
    else if tail.x == head.x {
        if rel_posy > 1 {
            tail.y += 1;
        }
        else if rel_posy < -1 {
            tail.y -= 1;
        }
    }
    else if rel_posx.abs() > 1 || rel_posy.abs() > 1{
        tail.x += ((rel_posx > 0) as i32) * 2 - 1;
        tail.y += ((rel_posy > 0) as i32) * 2 - 1;
    }
}


fn fix_rope_pos(rope: &mut Vec<Position>) {
    for i in 1..rope.len() {
        let rel_posx = rope.get(i-1).unwrap().x 
            - rope.get(i).unwrap().x;
        let rel_posy = rope.get(i-1).unwrap().y
            - rope.get(i).unwrap().y;
        
        if rope.get(i).unwrap().y == rope.get(i-1).unwrap().y {
            if rel_posx > 1 {
                rope.get_mut(i).unwrap().x += 1
            } 
            else if rel_posx < -1 {
                rope.get_mut(i).unwrap().x -= 1;
            }
        }
        else if rope.get(i).unwrap().x == rope.get(i-1).unwrap().x {
            if rel_posy > 1 {
                rope.get_mut(i).unwrap().y += 1;
            }
            else if rel_posy < -1 {
                rope.get_mut(i).unwrap().y -= 1;
            }
        }
        else if rel_posx.abs() > 1 || rel_posy.abs() > 1{
            rope.get_mut(i).unwrap().x += ((rel_posx > 0) as i32) * 2 - 1;
            rope.get_mut(i).unwrap().y += ((rel_posy > 0) as i32) * 2 - 1;
        }
    }
}


fn visualize_grid(rope: &Vec<Position>) {
    for col in 0..20 {
        for row in 0..20 {
            if rope.contains(&Position::new(row-10, col-10)) {
                print!("R");
            }
            else {
                print!(".");
            }
        }
        print!("\n");
    }
}


fn part1(input: &Vec<String>) {
    // Position (x, y)
    let mut visited: Vec<Position> = vec![Position::new(0, 0)];
    let mut head = Position::new(0, 0);  
    let mut tail = Position::new(0, 0);

    for mov in input.iter() {
        let mut split = mov.split(' ');
        let direction = split.next().unwrap();
        let steps = split.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..steps {
            if direction == "R" {
                head.x += 1;
            }
            else if direction == "L" {
                head.x -= 1;
            }
            else if direction == "U" {
                head.y += 1;
            }
            else if direction == "D" {
                head.y -= 1;
            }

            fix_tail_pos(&mut tail, &head);

            if !visited.contains(&tail) {
                visited.push(tail);
            }
        }
    }
    println!("{}", visited.len())
}


fn part2(input: &Vec<String>) {
    // Position (x, y)
    let mut visited: Vec<Position> = vec![Position::new(0, 0)];
    let mut rope: Vec<Position> = Vec::with_capacity(10);
    for _ in 0..10 {
        rope.push(Position::new(0, 0));
    }

    for mov in input.iter() {
        let mut split = mov.split(' ');
        let direction = split.next().unwrap();
        let steps = split.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..steps {
            if direction == "R" {
                rope.first_mut().unwrap().x += 1;
            }
            else if direction == "L" {
                rope.first_mut().unwrap().x -= 1;
            }
            else if direction == "U" {
                rope.first_mut().unwrap().y += 1;
            }
            else if direction == "D" {
                rope.first_mut().unwrap().y -= 1;
            }

            fix_rope_pos(&mut rope);

            if !visited.contains(rope.last().unwrap()) {
                visited.push(rope.last().unwrap().clone());
            }

            // visualize_grid(&rope);
            // println!("------------------------------");
        }
    }
    println!("{}", visited.len())
}

fn main() {
    let day = 9;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: Vec<String> = read_inputs(&file_path).unwrap();
    println!("PART 1:");
    part1(&input);
    println!("PART 2:");
    part2(&input);
}
