use aoc::*;

struct Range 
{
    start: u32,
    end:   u32 
}

fn find_bigger_range(range1: &Range, range2: &Range) -> usize
{
    let r1 = range1.end - range1.start;
    let r2 = range2.end - range2.start;

    if r1 > r2 { 0 } else { 1 }
}

fn is_overlapped(range1: &Range, range2: &Range) -> bool
{
    if range1.start <= range2.start && range2.start <= range1.end { return true; }
    if range1.start <= range2.end && range2.end <= range1.end { return true; }

    false
}

fn part1(input: &Vec<String>) {
    let mut contained_ranges: u32 = 0;
    
    for line in input.iter() {
        // println!("{}", line);
        let split = line.split(",");

        // Vec[start, end]
        let mut elves: Vec<Range> = Vec::new();
        for range_str in split {
            let ssplit: Vec<u32> = range_str.split("-").filter_map(|num| num.parse::<u32>().ok()).collect();
            elves.push(Range{start: ssplit[0], end: ssplit[1]});
        }

        let bigger = find_bigger_range(&elves[0], &elves[1]);
        let smaller = (bigger + 1) % 2;

        if elves[bigger].start <= elves[smaller].start && elves[bigger].end >= elves[smaller].end {
            contained_ranges += 1;
        }
    }

    println!("{}", contained_ranges);
}

fn part2(input: &Vec<String>) {
    let mut contained_ranges: u32 = 0;
    
    for line in input.iter() {
        // println!("{}", line);
        let split = line.split(",");

        // Vec[start, end]
        let mut elves: Vec<Range> = Vec::new();
        for range_str in split {
            let ssplit: Vec<u32> = range_str.split("-").filter_map(|num| num.parse::<u32>().ok()).collect();
            elves.push(Range{start: ssplit[0], end: ssplit[1]});
        }

        let bigger = find_bigger_range(&elves[0], &elves[1]);
        let smaller = (bigger + 1) % 2;

        if is_overlapped(&elves[bigger], &elves[smaller]) {
            contained_ranges += 1;
        }
    }

    println!("{}", contained_ranges);
}

fn main() {
    let day = 4;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: Vec<String> = match read_inputs(&file_path) {
        Ok(input) => input,
        Err(e) => panic!("Error parsing the inputs: {:?}", e)
    };
    println!("PART 1:");
    part1(&input);
    println!("PART 2:");
    part2(&input);
}
