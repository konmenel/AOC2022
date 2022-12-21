use aoc::read_inputs;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};

type InputT = Vec<String>;

#[derive(Debug)]
struct Signal {
    pos: [i32; 2],
    closes_beacon: Beacon,
    distance: i32,
}

impl Signal {
    fn new(pos: [i32; 2], beacon: Beacon) -> Self {
        let dist = (&pos[0] - &beacon.pos[0]).abs() + (&pos[1] - &beacon.pos[1]).abs();
        Self {
            pos: pos,
            closes_beacon: beacon,
            distance: dist,
        }
    }

    fn dist_from(&self, x: i32, y: i32) -> i32 {
        (&self.pos[0] - x).abs() + (&self.pos[1] - y).abs()
    }
}

#[derive(Debug)]
struct Beacon {
    pos: [i32; 2],
}

fn parse_report(input: &InputT) -> (Vec<Signal>, ([i32; 2], [i32; 2])) {
    lazy_static! {
        static ref RE_POS: Regex = Regex::new(
            "Sensor at x=(?P<sx>[-0-9]+), y=(?P<sy>[-0-9]+): closest beacon is at x=(?P<bx>[-0-9]+), y=(?P<by>[-0-9]+)"
        ).unwrap();
    }

    let mut minx = i32::MAX;
    let mut maxx = i32::MIN;
    let mut miny = i32::MAX;
    let mut maxy = i32::MIN;
    let mut signals = vec![];

    for line in input {
        let matches: Vec<_> = RE_POS.captures_iter(line).collect();

        let bpos = [
            matches[0]["bx"].parse::<i32>().ok().unwrap(),
            matches[0]["by"].parse::<i32>().ok().unwrap(),
        ];

        let spos = [
            matches[0]["sx"].parse::<i32>().ok().unwrap(),
            matches[0]["sy"].parse::<i32>().ok().unwrap(),
        ];

        let beacon = Beacon { pos: bpos };
        let signal = Signal::new(spos, beacon);

        minx = min(minx, min(spos[0] - signal.distance, bpos[0]));
        maxx = max(maxx, max(spos[0] + signal.distance, bpos[0]));
        miny = min(miny, min(spos[1] - signal.distance, bpos[1]));
        maxy = max(maxy, max(spos[1] + signal.distance, bpos[1]));

        signals.push(signal);
    }
    (signals, ([minx, maxx], [miny, maxy]))
}

fn part1(input: &InputT) {
    #[cfg(debug_assertions)]
    const ROW: i32 = 10;

    #[cfg(not(debug_assertions))]
    const ROW: i32 = 2000000;

    let (signals, ranges) = parse_report(input);

    let mut sum = 0;

    for x in ranges.0[0]..ranges.0[1] {
        for signal in signals.iter() {
            if signal.closes_beacon.pos == [x, ROW] {
                continue;
            }

            let dist = signal.dist_from(x, ROW);
            if dist <= signal.distance {
                sum += 1;
                break;
            }
        }
    }

    println!("{}", sum);
}

fn part2(input: &InputT) {
    #[cfg(debug_assertions)]
    const MAX_COORD: i32 = 20;

    #[cfg(not(debug_assertions))]
    const MAX_COORD: i32 = 4_000_000;

    let (signals, _) = parse_report(input);

    let mut freq: u64 = 0;

    let mut search_pos = vec![];

    // look only at points on 'circles' of radius `distance + 1`
    for signal in signals.iter() {
        let r = signal.distance + 1;
        let xmin = max(0, signal.pos[0] - r);
        let xmax = min(MAX_COORD, signal.pos[0] + r);

        for x in xmin..xmax {
            let y1 = signal.pos[1] - (r - (signal.pos[0] - x).abs());
            let y2 = signal.pos[1] + (r - (signal.pos[0] - x).abs());

            if y1 >= 0 {
                search_pos.push([x, y1]);
            }
            if y2 <= MAX_COORD {
                search_pos.push([x, y2]);
            }
        }
    }

    for [x, y] in search_pos.iter() {
        let mut broken = false;
        for signal in signals.iter() {
            if signal.closes_beacon.pos == [*x, *y] {
                broken = true;
                break;
            }

            if signal.pos == [*x, *y] {
                broken = true;
                break;
            }

            let dist = signal.dist_from(*x, *y);
            if dist <= signal.distance {
                broken = true;
                break;
            }
        }
        if !broken {
            println!("x={x}, y={y}");
            freq = *x as u64 * 4000000 + *y as u64;
            break;
        }
    }

    println!("{:?}", freq);
}

fn main() {
    let day: u32 = 15;
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
