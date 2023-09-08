use std::{collections::HashMap, str::FromStr, time::Instant};

use anyhow::{anyhow, Context, Error, Result};

#[derive(Debug)]
struct State {
    pub board: Board,
    pub cur_pos: GridCoord,
    pub direction: Direction,
    pub is_cube: bool,
}

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    first_pos: GridCoord,
    tiles: HashMap<GridCoord, Tile>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct GridCoord {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
enum Tile {
    Wall,
    Free,
}

#[derive(Debug)]
enum Instruction {
    Forward(usize),
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Instructions(Vec<Instruction>);

impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let width = s.lines().map(|l| l.len()).max().context("no lines")?;
        let height = s.lines().count();

        let mut first_pos = None;
        let mut tiles = HashMap::new();
        let mut y = 0;
        for line in s.lines() {
            for (x, c) in line.chars().enumerate().filter(|(_, c)| *c != ' ') {
                let tile = match c {
                    '.' => Tile::Free,
                    '#' => Tile::Wall,
                    _ => return Err(anyhow!("invalid tile")),
                };
                if let Tile::Free = tile {
                    if first_pos.is_none() {
                        first_pos = Some(GridCoord { x, y });
                    }
                }
                let c = GridCoord { x, y };
                tiles.insert(c, tile);
            }
            y += 1;
        }
        Ok(Board {
            tiles,
            first_pos: first_pos.context("no free tile")?,
            width,
            height,
        })
    }
}

impl FromStr for Instructions {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut instructions = Vec::new();
        let mut cur_num = String::new();
        for c in s.trim().chars() {
            if !c.is_ascii_digit() && !cur_num.is_empty() {
                instructions.push(Instruction::Forward(cur_num.parse()?));
                cur_num.clear();
            }
            match c {
                '0'..='9' => cur_num.push(c),
                'R' => instructions.push(Instruction::Right),
                'L' => instructions.push(Instruction::Left),
                _ => return Err(anyhow!("invalid instruction")),
            }
        }
        if !cur_num.is_empty() {
            instructions.push(Instruction::Forward(cur_num.parse()?));
        }
        Ok(Instructions(instructions))
    }
}

impl GridCoord {
    pub fn advance(&self, d: &Direction) -> Self {
        match d {
            Direction::Up => GridCoord {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => GridCoord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => GridCoord {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => GridCoord {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl Direction {
    pub fn rotate_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

impl State {
    pub fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(n) => {
                for _ in 0..n {
                    let mut new_c = self.cur_pos.advance(&self.direction);
                    let mut new_dir = self.direction;
                    if self.is_cube {
                        self.wrap_around_cube(&mut new_c, &mut new_dir);
                    } else {
                        self.wrap_around_2d(&mut new_c);
                    }
                    let tile = &self.board.tiles[&new_c];
                    if let Tile::Free = tile {
                        self.cur_pos = new_c;
                        self.direction = new_dir;
                    } else {
                        break;
                    }
                }
            }
            Instruction::Left => {
                self.direction = self.direction.rotate_left();
            }
            Instruction::Right => {
                self.direction = self.direction.rotate_right();
            }
        };
    }

    pub fn wrap_around_2d(&self, c: &mut GridCoord) {
        if !self.board.tiles.contains_key(&c) {
            match self.direction {
                Direction::Left => {
                    let new_x = (0..self.board.width)
                        .rev()
                        .find(|x| self.board.tiles.contains_key(&GridCoord { x: *x, y: c.y }))
                        .unwrap();
                    c.x = new_x;
                }
                Direction::Right => {
                    let new_x = (0..self.board.width)
                        .find(|x| self.board.tiles.contains_key(&GridCoord { x: *x, y: c.y }))
                        .unwrap();
                    c.x = new_x;
                }
                Direction::Up => {
                    let new_y = (0..self.board.height)
                        .rev()
                        .find(|y| self.board.tiles.contains_key(&GridCoord { x: c.x, y: *y }))
                        .unwrap();
                    c.y = new_y;
                }
                Direction::Down => {
                    let new_y = (0..self.board.height)
                        .find(|y| self.board.tiles.contains_key(&GridCoord { x: c.x, y: *y }))
                        .unwrap();
                    c.y = new_y;
                }
            }
        }
    }

    pub fn wrap_around_cube(&self, c: &mut GridCoord, cur_dir: &mut Direction) {
        let cube_size = 50;

        let cur_sq = (self.cur_pos.x / cube_size, self.cur_pos.y / cube_size);
        let new_sq = (c.x / cube_size, c.y / cube_size);

        if cur_sq != new_sq {
            let (sq_y, sq_x, new_dir) = match (cur_sq.1, cur_sq.0, &self.direction) {
                (0, 1, Direction::Up) => (3, 0, Direction::Right),
                (0, 1, Direction::Left) => (2, 0, Direction::Right),
                (0, 2, Direction::Up) => (3, 0, Direction::Up),
                (0, 2, Direction::Right) => (2, 1, Direction::Left),
                (0, 2, Direction::Down) => (1, 1, Direction::Left),
                (1, 1, Direction::Right) => (0, 2, Direction::Up),
                (1, 1, Direction::Left) => (2, 0, Direction::Down),
                (2, 0, Direction::Up) => (1, 1, Direction::Right),
                (2, 0, Direction::Left) => (0, 1, Direction::Right),
                (2, 1, Direction::Right) => (0, 2, Direction::Left),
                (2, 1, Direction::Down) => (3, 0, Direction::Left),
                (3, 0, Direction::Right) => (2, 1, Direction::Up),
                (3, 0, Direction::Down) => (0, 2, Direction::Down),
                (3, 0, Direction::Left) => (0, 1, Direction::Down),
                _ => return,
            };
            let (mod_x, mod_y) = (self.cur_pos.x % 50, self.cur_pos.y % 50);
            let offset_val = match &self.direction {
                Direction::Up => mod_x,
                Direction::Right => mod_y,
                Direction::Down => (cube_size-1) - mod_x,
                Direction::Left => (cube_size-1) - mod_y,
            };
            let (new_x, new_y) = match new_dir {
                Direction::Up => (offset_val, cube_size - 1),
                Direction::Right => (0, offset_val),
                Direction::Down => ((cube_size-1)-offset_val, 0),
                Direction::Left => (cube_size - 1, (cube_size-1)-offset_val),
            };
            c.x = sq_x * cube_size + new_x;
            c.y = sq_y * cube_size + new_y;
            *cur_dir = new_dir;
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let (board, instructions) = input.split_once("\n\n").context("invalid input")?;
    let board = board.parse::<Board>()?;
    let instructions = instructions.parse::<Instructions>()?.0;
    let mut state = State {
        cur_pos: board.first_pos.clone(),
        board,
        direction: Direction::Right,
        is_cube: false,
    };
    for instruction in instructions {
        state.apply(instruction);
    }
    let password =
        1000 * (state.cur_pos.y + 1) + 4 * (state.cur_pos.x + 1) + state.direction.score();
    Ok(password)
}

fn part2(input: &str) -> Result<usize> {
    let (board, instructions) = input.split_once("\n\n").context("invalid input")?;
    let board = board.parse::<Board>()?;
    let instructions = instructions.parse::<Instructions>()?.0;
    let mut state = State {
        cur_pos: board.first_pos.clone(),
        board,
        direction: Direction::Right,
        is_cube: true,
    };
    for instruction in instructions {
        state.apply(instruction);
    }
    let password =
        1000 * (state.cur_pos.y + 1) + 4 * (state.cur_pos.x + 1) + state.direction.score();
    Ok(password)
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
