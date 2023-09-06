use std::{str::FromStr, time::Instant};

use anyhow::{Context, Error, Result};

#[derive(Debug, Clone)]
struct Blueprint {
    pub id: usize,
    pub robots: Vec<Money>,
}

#[derive(Debug, Clone)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone)]
struct Money {
    pub typ: RobotType,
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
    pub geode: usize,
}

#[derive(Debug, Clone)]
struct State {
    pub blueprint: Blueprint,
    pub money: Money,
    pub turn: usize,
    pub ore_robots: usize,
    pub clay_robots: usize,
    pub obsidian_robots: usize,
    pub geode_robots: usize,
}

#[derive(Debug, Clone)]
pub enum Move {
    Wait,
    Pay(Money),
}

impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = regex::Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian").unwrap();
        let caps = re.captures(s).unwrap();
        let id = caps[1].parse::<usize>()?;
        let ore = Money {
            typ: RobotType::Ore,
            ore: caps[2].parse::<usize>()?,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let clay = Money {
            typ: RobotType::Clay,
            ore: caps[3].parse::<usize>()?,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let obsidian = Money {
            typ: RobotType::Obsidian,
            ore: caps[4].parse::<usize>()?,
            clay: caps[5].parse::<usize>()?,
            obsidian: 0,
            geode: 0,
        };
        let geode = Money {
            typ: RobotType::Geode,
            ore: caps[6].parse::<usize>()?,
            clay: 0,
            obsidian: caps[7].parse::<usize>()?,
            geode: 0,
        };
        Ok(Blueprint {
            id,
            robots: vec![ore, clay, obsidian, geode],
        })
    }
}

impl Money {
    pub fn can_afford(&self, other: &Self) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }

    pub fn pay(&mut self, other: &Self) {
        self.ore -= other.ore;
        self.clay -= other.clay;
        self.obsidian -= other.obsidian;
        self.geode -= other.geode;
    }
}

impl State {
    pub fn apply(&self, mv: &Move) -> Self {
        let mut new_state = self.clone();
        if let Move::Pay(robot) = mv {
            new_state.money.pay(robot);
            new_state.mine();
            match robot.typ {
                RobotType::Ore => new_state.ore_robots += 1,
                RobotType::Clay => new_state.clay_robots += 1,
                RobotType::Obsidian => new_state.obsidian_robots += 1,
                RobotType::Geode => new_state.geode_robots += 1,
            };
        } else {
            new_state.mine();
        }
        new_state.turn += 1;

        new_state
    }

    pub fn mine(&mut self) {
        self.money.ore += self.ore_robots;
        self.money.clay += self.clay_robots;
        self.money.obsidian += self.obsidian_robots;
        self.money.geode += self.geode_robots;
    }

    pub fn moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        for robot in self.blueprint.robots.iter() {
            if self.money.can_afford(&robot) {
                moves.push(Move::Pay(robot.clone()));
            }
        }
        moves.push(Move::Wait);
        moves
    }
}

/// Get the best geode for a given starting state and number of turns.
/// Not my cleanest solution, we truncate the queue manually to avoid having
/// to go through low score states. I guess that's the way to do this iteratively
/// The other option would have been to do a recursive pruning DFS
fn get_best_geode(starting_state: State, turns: usize) -> usize {
    let mut queue: Vec<State> = vec![starting_state];
    for _ in 0..turns {
        let mut new_queue = vec![];
        for state in queue.iter() {
            let moves = state.moves();
            for mv in moves {
                let new_state = state.apply(&mv);
                new_queue.push(new_state);
            }
        }
        new_queue.sort_by_key(|s| {
            (
                s.money.geode + s.geode_robots,
                s.money.obsidian + s.obsidian_robots,
                s.money.clay + s.clay_robots,
                s.money.ore + s.ore_robots,
                s.geode_robots,
                s.obsidian_robots,
                s.clay_robots,
                s.ore_robots,
            )
        });
        new_queue.reverse();
        new_queue.truncate(1000);
        queue = new_queue;
    }
    queue.sort_by_key(|s| s.money.geode);
    queue.reverse();
    queue[0].money.geode
}

fn part1(input: &str) -> Result<usize> {
    let blueprints = input
        .lines()
        .map(|l| l.parse::<Blueprint>())
        .collect::<Result<Vec<Blueprint>>>()?;

    let mut quality = 0;
    for blueprint in blueprints {
        let id = blueprint.id;
        let starting_state = State {
            blueprint,
            money: Money {
                typ: RobotType::Geode,
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            turn: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        };
        let best_geode = get_best_geode(starting_state, 24);
        quality += id * best_geode;
    }

    Ok(quality)
}

fn part2(input: &str) -> Result<usize> {
    let blueprints = input
        .lines()
        .map(|l| l.parse::<Blueprint>())
        .collect::<Result<Vec<Blueprint>>>()?;

    let mut res = 1;
    for blueprint in blueprints.into_iter().take(3) {
        let starting_state = State {
            blueprint,
            money: Money {
                typ: RobotType::Geode,
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            turn: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        };
        let best_geode = get_best_geode(starting_state, 32);
        res *= best_geode;
    }

    Ok(res)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let instant = Instant::now();
    let max_geodes = part1(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 1: {} ({:?})", max_geodes, time);

    let instant = Instant::now();
    let mult_res = part2(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 2: {} ({:?})", mult_res, time);

    Ok(())
}
