use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    time::Instant,
};

use anyhow::{Error, Result};

#[derive(Debug)]
struct Grid {
    pub elves: Vec<Elf>,
    pub elves_coord: HashSet<GridCoord>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct GridCoord {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone)]
struct Elf {
    pub dir_i: usize,
    pub coord: GridCoord,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut elves = vec![];
        let mut elves_coord = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            for (x, _) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
                let coord = GridCoord {
                    x: x as isize,
                    y: y as isize,
                };
                let elf = Elf {
                    dir_i: 0,
                    coord: coord.clone(),
                };
                elves.push(elf);
                elves_coord.insert(coord);
            }
        }

        Ok(Grid {
            elves,
            elves_coord,
        })
    }
}

impl Grid {
    pub fn step(&mut self) -> bool {
        let mut new_moves = HashMap::new();
        let mut to_remove = HashSet::new();
        for elf_i in 0..self.elves.len() {
            let new_c = self.elf_move(elf_i);
            let elf = &mut self.elves[elf_i];
            elf.dir_i = (elf.dir_i + 1) % 4;
            if let Some(new_c) = new_c {
                if new_moves.contains_key(&new_c) {
                    to_remove.insert(new_c);
                } else {
                    new_moves.insert(new_c, elf_i);
                }
            }
        }
        for c in to_remove {
            new_moves.remove(&c);
        }
        let moved = !new_moves.is_empty();
        for (new_c, elf_i) in new_moves {
            let elf = &mut self.elves[elf_i];
            self.elves_coord.remove(&elf.coord);
            elf.coord = new_c;
            self.elves_coord.insert(elf.coord.clone());
        }
        moved
    }

    pub fn elf_move(&self, elf_i: usize) -> Option<GridCoord> {
        let elf = &self.elves[elf_i];
        let move_coords = [
            [(0, -1), (-1, -1), (1, -1)], // North
            [(0, 1), (1, 1), (-1, 1)],    // South
            [(-1, 0), (-1, -1), (-1, 1)], // West
            [(1, 0), (1, -1), (1, 1)],    // East
        ];

        let mut fully_free = true;
        let mut first_c = None;
        for dir_moves in move_coords.iter().cycle().skip(elf.dir_i).take(4) {
            let mut free = true;
            for c in dir_moves {
                let new_c = GridCoord {
                    x: elf.coord.x + c.0,
                    y: elf.coord.y + c.1,
                };
                if self.elves_coord.contains(&new_c) {
                    free = false;
                    break;
                }
            }
            fully_free &= free;
            if free && first_c.is_none() {
                let c = GridCoord {
                    x: elf.coord.x + dir_moves[0].0,
                    y: elf.coord.y + dir_moves[0].1,
                };
                if !fully_free {
                    return Some(c);
                }
                first_c = Some(c);
            }
        }
        if fully_free {
            return None;
        }
        first_c
    }

    pub fn count_empty_tiles_in_rect(&self) -> usize {
        let (min_x, max_x, min_y, max_y) = self.get_min_coords();
        let r_width = max_x - min_x + 1;
        let r_height = max_y - min_y + 1;
        (r_width * r_height) as usize - self.elves.len()
    }

    pub fn get_min_coords(&self) -> (isize, isize, isize, isize) {
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        for elf in self.elves.iter() {
            min_x = elf.coord.x.min(min_x);
            max_x = elf.coord.x.max(max_x);
            min_y = elf.coord.y.min(min_y);
            max_y = elf.coord.y.max(max_y);
        }
        (min_x, max_x, min_y, max_y)
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x, min_y, max_y) = self.get_min_coords();
        let mut s = String::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c = GridCoord { x, y };
                if self.elves_coord.contains(&c) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        f.write_fmt(format_args!("{}\n", s))
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut grid: Grid = input.parse()?;
    for _ in 0..10 {
        grid.step();
    }
    Ok(grid.count_empty_tiles_in_rect())
}

fn part2(input: &str) -> Result<usize> {
    let mut grid: Grid = input.parse()?;
    let mut count = 1;
    while grid.step() {
        count += 1;
    }
    Ok(count)
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
