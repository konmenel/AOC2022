use aoc::read_inputs;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;

type InputT = Vec<String>;

#[derive(Debug)]
enum RobotType {
    Ore,
    Clay,
    Obs,
    Geo,
}

#[derive(Debug)]
struct Blueprint {
    ore: i32,      // ore
    clay: i32,     // ore
    obs: [i32; 2], // ore, clay
    geo: [i32; 2], // ore, obsidian
}

impl Blueprint {
    fn from_string(txt: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "Blueprint \\d+: \
                Each ore robot costs (?P<ore>\\d+) ore. \
                Each clay robot costs (?P<clay>\\d+) ore. \
                Each obsidian robot costs (?P<obs0>\\d+) ore and (?P<obs1>\\d+) clay. \
                Each geode robot costs (?P<geo0>\\d+) ore and (?P<geo1>\\d+) obsidian."
            )
            .unwrap();
        }

        let matches = RE.captures(txt).unwrap();
        Self {
            ore: matches["ore"].parse().ok().unwrap(),
            clay: matches["clay"].parse().ok().unwrap(),
            obs: [
                matches["obs0"].parse().ok().unwrap(),
                matches["obs1"].parse().ok().unwrap(),
            ],
            geo: [
                matches["geo0"].parse().ok().unwrap(),
                matches["geo1"].parse().ok().unwrap(),
            ],
        }
    }
}

fn time_to_build(
    blueprint: &Blueprint,
    resources: &[i32; 4],
    robots: &[i32; 4],
    robot_type: RobotType,
) -> i32 {
    use RobotType::*;
    match robot_type {
        Ore => {
            let rem_ore = blueprint.ore - resources[0];
            if rem_ore < 0 {
                return 0;
            }
            (rem_ore as f32 / robots[0] as f32).ceil() as i32
        }
        Clay => {
            let rem_ore = blueprint.clay - resources[0];
            if rem_ore < 0 {
                return 0;
            }
            (rem_ore as f32 / robots[0] as f32).ceil() as i32
        }
        Obs => {
            if robots[1] == 0 {
                return i32::MAX;
            }

            let rem_ore = blueprint.obs[0] - resources[0];
            let rem_clay = blueprint.obs[1] - resources[1];
            if rem_ore < 0 && rem_clay < 0 {
                return 0;
            }
            let ore_time = (rem_ore as f32 / robots[0] as f32).ceil() as i32;
            let clay_time = (rem_clay as f32 / robots[1] as f32).ceil() as i32;

            max(ore_time, clay_time)
        }
        Geo => {
            if robots[2] == 0 {
                return i32::MAX;
            }

            let rem_ore = blueprint.geo[0] - resources[0];
            let rem_obs = blueprint.geo[1] - resources[2];
            if rem_ore < 0 && rem_obs < 0 {
                return 0;
            }
            let ore_time = (rem_ore as f32 / robots[0] as f32).ceil() as i32;
            let obs_time = (rem_obs as f32 / robots[2] as f32).ceil() as i32;

            max(ore_time, obs_time)
        }
    }
}

fn dfs(blueprint: &Blueprint, resources: &[i32; 4], robots: &[i32; 4], time: i32) -> i32 {
    use RobotType::*;

    if time == 0 {
        return resources[3];
    }

    let mut max_geo = 0;

    let wait_ore = time_to_build(blueprint, resources, robots, Ore);
    let wait_clay = time_to_build(blueprint, resources, robots, Clay);
    let wait_obs = time_to_build(blueprint, resources, robots, Obs);
    let wait_geo = time_to_build(blueprint, resources, robots, Geo);
    let max_need_ore = max(blueprint.clay, max(blueprint.obs[0], blueprint.geo[0]));

    if wait_geo < time {
        let mut new_robots = robots.clone();
        let mut new_resources = resources.clone();
        new_resources[0] -= blueprint.geo[0];
        new_resources[2] -= blueprint.geo[1];

        new_resources[0] += new_robots[0] * (wait_geo + 1);
        new_resources[1] += new_robots[1] * (wait_geo + 1);
        new_resources[2] += new_robots[2] * (wait_geo + 1);
        new_resources[3] += new_robots[3] * (wait_geo + 1);
        new_robots[3] += 1;
        max_geo = max(
            max_geo,
            dfs(blueprint, &new_resources, &new_robots, time - 1 - wait_geo),
        );
    }
    if wait_obs < time && robots[2] * time + resources[2] < time * blueprint.geo[1] {
        let mut new_robots = robots.clone();
        let mut new_resources = resources.clone();
        new_resources[0] -= blueprint.obs[0];
        new_resources[1] -= blueprint.obs[1];

        new_resources[0] += new_robots[0] * (wait_obs + 1);
        new_resources[1] += new_robots[1] * (wait_obs + 1);
        new_resources[2] += new_robots[2] * (wait_obs + 1);
        new_resources[3] += new_robots[3] * (wait_obs + 1);
        new_robots[2] += 1;
        max_geo = max(
            max_geo,
            dfs(blueprint, &new_resources, &new_robots, time - 1 - wait_obs),
        );
    }
    if wait_clay < time && robots[1] * time + resources[1] < time * blueprint.obs[1] {
        let mut new_robots = robots.clone();
        let mut new_resources = resources.clone();
        new_resources[0] -= blueprint.clay;

        new_resources[0] += new_robots[0] * (wait_clay + 1);
        new_resources[1] += new_robots[1] * (wait_clay + 1);
        new_resources[2] += new_robots[2] * (wait_clay + 1);
        new_resources[3] += new_robots[3] * (wait_clay + 1);
        new_robots[1] += 1;
        max_geo = max(
            max_geo,
            dfs(blueprint, &new_resources, &new_robots, time - 1 - wait_clay),
        );
    }
    if wait_ore < time && robots[0] * time + resources[0] < time * max_need_ore {
        let mut new_robots = robots.clone();
        let mut new_resources = resources.clone();
        new_resources[0] -= blueprint.ore;

        new_resources[0] += new_robots[0] * (wait_ore + 1);
        new_resources[1] += new_robots[1] * (wait_ore + 1);
        new_resources[2] += new_robots[2] * (wait_ore + 1);
        new_resources[3] += new_robots[3] * (wait_ore + 1);
        new_robots[0] += 1;
        max_geo = max(max_geo, dfs(blueprint, &new_resources, &new_robots, time - 1 - wait_ore));
    }

    max(max_geo, resources[3] + robots[3] * time)
}

fn part1(input: &InputT) {
    let mut geodes = vec![];

    for txt in input {
        let blueprint = Blueprint::from_string(txt);
        let resources = [0; 4];     // [ore, clay, obs geo]
        let robots = [1, 0, 0, 0];  // number of robots [ore, clay, obs, geo]

        geodes.push(dfs(&blueprint, &resources, &robots, 24));
    }

    let quality_levels: i32 = geodes
        .iter()
        .enumerate()
        .map(|(i, &x)| (i + 1) as i32 * x)
        .sum();
    println!("{}", quality_levels);
}

fn part2(input: &InputT) {
    #[cfg(debug_assertions)]
    const MAX_BLUEPRINT: usize = 2;
    #[cfg(not(debug_assertions))]
    const MAX_BLUEPRINT: usize = 3;

    let mut geodes = vec![];

    for txt in &input[..MAX_BLUEPRINT] {
        let blueprint = Blueprint::from_string(txt);
        let resources = [0; 4]; // [ore, clay, obs geo]
        let robots = [1, 0, 0, 0]; // number of robots [ore, clay, obs, geo]

        geodes.push(dfs(&blueprint, &resources, &robots, 32));
    }

    let quality_levels: i32 = geodes
        .iter()
        .copied()
        .reduce(|a, b| a * b)
        .unwrap();
    println!("{}", quality_levels);
}

fn main() {
    let day: u32 = 19;
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
