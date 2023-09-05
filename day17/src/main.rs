use std::time::Instant;
use std::collections::HashSet;

use anyhow::Result;

#[derive(Clone, Debug, PartialEq)]
pub struct Rock {
    pub coords: Vec<GridCoord>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RockShape {
    Plus,
    Minus,
    Stair,
    Line,
    Square,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridCoord {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug)]
pub struct RockQueue {
    pub current_shape: RockShape,
    pub resting_rocks: HashSet<GridCoord>,
    pub highest_y: usize,
    pub move_idx: usize,
    pub last_rocks: Vec<RockShape>,
}

impl PartialEq for RockQueue {
    // this comp is maybe a bit flimsy, I'm only comparing the last 500 placed rock shapes
    fn eq(&self, other: &Self) -> bool {
        let n_rocks = 500;
        return self.current_shape == other.current_shape
            && self.move_idx == other.move_idx
            && self
                .last_rocks
                .iter()
                .rev()
                .take(n_rocks)
                .eq(other.last_rocks.iter().rev().take(n_rocks));
    }
}

#[derive(Clone, Debug)]
pub enum JetMove {
    Left,
    Right,
}

impl Rock {
    pub fn new(shape: &RockShape, y: usize) -> Self {
        let coords = match shape {
            RockShape::Minus => vec![
                GridCoord { x: 2, y },
                GridCoord { x: 3, y },
                GridCoord { x: 4, y },
                GridCoord { x: 5, y },
            ],
            RockShape::Plus => vec![
                GridCoord { x: 3, y },
                GridCoord { x: 2, y: y + 1 },
                GridCoord { x: 3, y: y + 1 },
                GridCoord { x: 4, y: y + 1 },
                GridCoord { x: 3, y: y + 2 },
            ],
            RockShape::Stair => vec![
                GridCoord { x: 2, y },
                GridCoord { x: 3, y },
                GridCoord { x: 4, y },
                GridCoord { x: 4, y: y + 1 },
                GridCoord { x: 4, y: y + 2 },
            ],
            RockShape::Line => vec![
                GridCoord { x: 2, y },
                GridCoord { x: 2, y: y + 1 },
                GridCoord { x: 2, y: y + 2 },
                GridCoord { x: 2, y: y + 3 },
            ],
            RockShape::Square => vec![
                GridCoord { x: 2, y },
                GridCoord { x: 3, y },
                GridCoord { x: 2, y: y + 1 },
                GridCoord { x: 3, y: y + 1 },
            ],
        };
        Rock { coords }
    }

    pub fn move_vertical(&mut self) {
        for c in self.coords.iter_mut() {
            c.y -= 1;
        }
    }

    pub fn jet_move(&mut self, jet_move: &JetMove) {
        for c in self.coords.iter_mut() {
            if let JetMove::Left = jet_move {
                c.x -= 1;
            } else {
                c.x += 1;
            }
        }
    }
}

impl RockQueue {
    pub fn spawn_rock(&mut self) -> Rock {
        let rock = Rock::new(&self.current_shape, self.highest_y + 3);
        self.last_rocks.push(self.current_shape.clone());
        self.current_shape = self.current_shape.next();
        rock
    }

    pub fn turn(&mut self, jet_moves: &Vec<JetMove>) {
        let mut rock = self.spawn_rock();

        for jet_move in jet_moves.iter().cycle().skip(self.move_idx) {
            self.move_idx = (self.move_idx + 1) % jet_moves.len();
            if !self.horizontal_collision(&rock, &jet_move) {
                rock.jet_move(&jet_move);
            }

            if self.vertical_collision(&rock) {
                break;
            }
            rock.move_vertical();
        }

        self.update_state(rock);
    }

    pub fn vertical_collision(&self, rock: &Rock) -> bool {
        for c in rock.coords.iter() {
            if c.y.checked_sub(1).is_none() {
                return true;
            }
        }
        for c in rock.coords.iter() {
            let new_c = GridCoord { x: c.x, y: c.y - 1 };
            if self.resting_rocks.contains(&new_c) {
                return true;
            }
        }
        false
    }

    pub fn horizontal_collision(&self, rock: &Rock, jet_move: &JetMove) -> bool {
        for c in rock.coords.iter() {
            match jet_move {
                JetMove::Left => {
                    if c.x.checked_sub(1).is_none() {
                        return true;
                    }
                    let new_c = GridCoord { x: c.x - 1, y: c.y };
                    if self.resting_rocks.contains(&new_c) {
                        return true;
                    }
                }
                JetMove::Right => {
                    if c.x == 6 {
                        return true;
                    }
                    let new_c = GridCoord { x: c.x + 1, y: c.y };
                    if self.resting_rocks.contains(&new_c) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn update_state(&mut self, rock: Rock) {
        let mut best_y = self.highest_y;
        for c in rock.coords {
            if c.y >= best_y {
                best_y = c.y + 1;
            }
            self.resting_rocks.insert(c);
        }
        if best_y != self.highest_y {
            self.highest_y = best_y;
        }
    }
}

impl RockShape {
    pub fn next(&self) -> Self {
        match self {
            RockShape::Minus => RockShape::Plus,
            RockShape::Plus => RockShape::Stair,
            RockShape::Stair => RockShape::Line,
            RockShape::Line => RockShape::Square,
            RockShape::Square => RockShape::Minus,
        }
    }
}

impl std::fmt::Display for RockQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = String::new();
        for _ in 0..self.highest_y + 8 {
            grid += ".......\n";
        }
        for c in self.resting_rocks.iter() {
            let i = c.x + c.y * 8;
            grid.replace_range(i..i + 1, "#");
        }
        f.write_fmt(format_args!("{}", grid))
    }
}

fn part1(input: &str) -> Result<usize> {
    let jet_moves: Vec<JetMove> = input
        .chars()
        .filter_map(|c| match c {
            '>' => Some(JetMove::Right),
            '<' => Some(JetMove::Left),
            _ => None,
        })
        .collect();

    let mut queue = RockQueue {
        current_shape: RockShape::Minus,
        highest_y: 0,
        resting_rocks: HashSet::new(),
        move_idx: 0,
        last_rocks: vec![],
    };

    let n_rocks = 2022;
    for _ in 0..n_rocks {
        queue.turn(&jet_moves);
    }

    Ok(queue.highest_y)
}

fn part2(input: &str) -> Result<usize> {
    let jet_moves: Vec<JetMove> = input
        .chars()
        .filter_map(|c| match c {
            '>' => Some(JetMove::Right),
            '<' => Some(JetMove::Left),
            _ => None,
        })
        .collect();

    let n_rocks: i64 = 1000000000000;

    let init_state = RockQueue {
        current_shape: RockShape::Minus,
        highest_y: 0,
        resting_rocks: HashSet::new(),
        move_idx: 0,
        last_rocks: vec![],
    };

    let mut tortoise = init_state.clone();
    let mut hare = tortoise.clone();

    tortoise.turn(&jet_moves);
    hare.turn(&jet_moves);
    hare.turn(&jet_moves);
    while tortoise != hare {
        tortoise.turn(&jet_moves);
        hare.turn(&jet_moves);
        hare.turn(&jet_moves);
    }

    // Find "mu", the start of the cycle
    let mut mu = 0;
    tortoise = init_state.clone();
    while tortoise != hare {
        tortoise.turn(&jet_moves);
        hare.turn(&jet_moves);
        mu += 1;
    }

    // Find the cycle len
    let mut cycle_len = 1;
    let mut hare = tortoise.clone();
    hare.turn(&jet_moves);
    while tortoise != hare {
        hare.turn(&jet_moves);
        cycle_len += 1;
    }

    // Reach the start of the cycle
    let mut queue = init_state.clone();
    for _ in 0..mu {
        queue.turn(&jet_moves);
    }

    // Compute the number of cycles we can skip
    let mut n_cycles = (n_rocks - mu) / cycle_len;
    let cycles = cycle_len * n_cycles;

    // Compute the highest_y difference to compute the simulated_score later
    let cur_y = queue.highest_y;
    for _ in 0..cycle_len {
        queue.turn(&jet_moves);
    }
    n_cycles -= 1;
    let diff_y = queue.highest_y - cur_y;

    let simulated_score = queue.highest_y + diff_y * n_cycles as usize;
    let rem_turns = n_rocks - cycles - mu;

    // Play the remaining non-cycle turns
    let cur_y = queue.highest_y;
    for _ in 0..rem_turns {
        queue.turn(&jet_moves);
    }
    let diff_y = queue.highest_y - cur_y;

    Ok(simulated_score + diff_y)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let instant = Instant::now();
    let highest_y = part1(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 1: {} ({:?})", highest_y, time);

    let instant = Instant::now();
    let highest_y = part2(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 1: {} ({:?})", highest_y, time);

    Ok(())
}
