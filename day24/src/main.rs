use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
    time::Instant,
};

use anyhow::{Context, Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    pub width: usize,
    pub height: usize,
    pub blizzards: Vec<Blizzard>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Blizzard {
    pub coord: GridCoord,
    pub direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pub turn: usize,
    pub player: GridCoord,
    pub end: GridCoord,
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.turn.hash(state);
        self.player.hash(state);
        self.end.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GridCoord {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone)]
enum Move {
    Dir(Direction),
    Wait,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let width = s.lines().next().context("no lines")?.len() - 2;
        let height = s.lines().count() - 2;

        let mut blizzards = vec![];
        for (y, line) in s.lines().skip(1).enumerate() {
            for (x, c) in line
                .chars()
                .filter(|c| *c != '#')
                .enumerate()
                .filter(|(_, c)| *c != '.')
            {
                let direction = match c {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => unreachable!(),
                };
                let c = GridCoord {
                    x: x as isize,
                    y: y as isize,
                };
                let blizzard = Blizzard {
                    coord: c,
                    direction,
                };
                blizzards.push(blizzard);
            }
        }

        Ok(Grid {
            width,
            height,
            blizzards,
        })
    }
}

impl Direction {
    pub fn as_coord(&self) -> GridCoord {
        match self {
            Self::Right => GridCoord { x: 1, y: 0 },
            Self::Left => GridCoord { x: -1, y: 0 },
            Self::Up => GridCoord { x: 0, y: -1 },
            Self::Down => GridCoord { x: 0, y: 1 },
        }
    }
}

impl Grid {
    pub fn blizzard_cycles(&self) -> Vec<HashSet<GridCoord>> {
        let mut cur_blizzards = self.blizzards.clone();
        let mut first_cycle = HashSet::new();
        for b in cur_blizzards.iter() {
            first_cycle.insert(b.coord.clone());
        }
        let mut cycles = vec![first_cycle];

        for _ in 0..self.height * self.width {
            let mut cycle = HashSet::new();
            for blizzard in cur_blizzards.iter_mut() {
                let c = blizzard.direction.as_coord();
                let new_c = GridCoord {
                    x: blizzard.coord.x + c.x,
                    y: blizzard.coord.y + c.y,
                };
                if self.valid_coord(&new_c) {
                    blizzard.coord = new_c;
                } else {
                    blizzard.coord = match blizzard.direction {
                        Direction::Left => GridCoord {
                            x: self.width as isize - 1,
                            y: blizzard.coord.y,
                        },
                        Direction::Right => GridCoord {
                            x: 0,
                            y: blizzard.coord.y,
                        },
                        Direction::Up => GridCoord {
                            x: blizzard.coord.x,
                            y: self.height as isize - 1,
                        },
                        Direction::Down => GridCoord {
                            x: blizzard.coord.x,
                            y: 0,
                        },
                    };
                }
                cycle.insert(blizzard.coord.clone());
            }
            cycles.push(cycle);
        }
        cycles
    }

    pub fn valid_coord(&self, c: &GridCoord) -> bool {
        c.x >= 0 && c.x < self.width as isize && c.y >= 0 && c.y < self.height as isize
            || c.x == 0 && c.y == -1
            || c.x == self.width as isize - 1 && c.y == self.height as isize
    }
}

impl State {
    const MOVES: [Move; 5] = [
        Move::Dir(Direction::Left),
        Move::Dir(Direction::Right),
        Move::Dir(Direction::Up),
        Move::Dir(Direction::Down),
        Move::Wait,
    ];

    pub fn apply(&self, mv: Move) -> Self {
        let mut new_state = self.clone();
        match mv {
            Move::Dir(d) => {
                let c = d.as_coord();
                new_state.player.x += c.x;
                new_state.player.y += c.y;
            }
            Move::Wait => {}
        };
        new_state.turn += 1;
        new_state
    }

    pub fn moves(&self, grid: &Grid, cycles: &Vec<HashSet<GridCoord>>) -> Vec<Self> {
        let mut moves = vec![];
        for mv in Self::MOVES {
            let new_state = self.apply(mv);
            if !grid.valid_coord(&new_state.player) {
                continue;
            }
            let cycle = &cycles[new_state.turn % cycles.len()];
            if !cycle.contains(&new_state.player) {
                moves.push(new_state);
            }
        }
        moves
    }
}

fn shortest_path(init_state: State, grid: &Grid, cycles: &Vec<HashSet<GridCoord>>) -> Option<State> {
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(init_state);

    let mut final_state = None;
    while let Some(state) = queue.pop_front() {
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state.clone());
        if state.player == state.end {
            final_state = Some(state);
            break;
        }
        let moves = state.moves(&grid, &cycles);
        queue.extend(moves);
    }
    final_state
}

fn part1(input: &str) -> Result<usize> {
    let grid: Grid = input.parse()?;
    let cycles = grid.blizzard_cycles();
    let init_state = State {
        player: GridCoord { x: 0, y: -1 },
        end: GridCoord {
            x: grid.width as isize - 1,
            y: grid.height as isize,
        },
        turn: 0,
    };
    let final_state = shortest_path(init_state, &grid, &cycles).context("no path found")?;
    Ok(final_state.turn)
}

fn part2(input: &str) -> Result<usize> {
    let grid: Grid = input.parse()?;
    let cycles = grid.blizzard_cycles();
    let init_state = State {
        player: GridCoord { x: 0, y: -1 },
        end: GridCoord {
            x: grid.width as isize - 1,
            y: grid.height as isize,
        },
        turn: 0,
    };
    let mut state = shortest_path(init_state, &grid, &cycles).context("no path found")?;
    state.end = GridCoord {
        x: 0, y: -1,
    };
    let mut state = shortest_path(state, &grid, &cycles).context("no path found")?;
    state.end = GridCoord {
        x: grid.width as isize - 1,
        y: grid.height as isize,
    };
    let state = shortest_path(state, &grid, &cycles).context("no path found")?;
    Ok(state.turn)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let instant = Instant::now();
    let res = part1(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 1: {} ({:?})", res, time);

    let instant = Instant::now();
    let res = part2(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 2: {} ({:?})", res, time);

    Ok(())
}
