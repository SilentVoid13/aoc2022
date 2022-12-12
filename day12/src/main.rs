use std::{str::FromStr, collections::VecDeque};
use std::collections::{HashSet, vec_deque};

use anyhow::{anyhow, Context, Error, Result};

pub struct Grid {
    data: Vec<Vec<u8>>,
    height: usize,
    width: usize,
    starting: u8,
    ending: u8,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct GridPos(usize, usize);

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<_>> = s
            .lines()
            .map(|l| l.as_bytes().iter().copied().collect())
            .collect();
        let height = data.len();
        let width = data.first().unwrap().len();
        Ok(Grid {
            data,
            width,
            height,
            starting: 'S' as u8,
            ending: 'E' as u8,
        })
    }
}

impl GridPos {
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }
}

impl Grid {
    pub fn get(&self, pos: &GridPos) -> Result<u8> {
        Ok(*self
            .data
            .get(pos.1)
            .ok_or(anyhow!("invalid pos"))?
            .get(pos.0)
            .ok_or(anyhow!("invalid pos"))?)
    }

    pub fn starting_pos(&self, part1: bool) -> Vec<GridPos> {
        let mut pos = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let gridpos = GridPos::new(x, y);
                let v = self.get(&gridpos).unwrap();
                if v == self.starting || (!part1 && v == 'a' as u8) {
                    pos.push(gridpos);
                }
            }
        }
        pos
    }

    pub fn reachable_squares(&self, pos: &GridPos) -> Vec<GridPos> {
        let mut reachable = vec![];
        let mut cur_v = self.get(&pos).unwrap();
        if cur_v == self.starting {
            cur_v = 'a' as u8;
        }

        let coords: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        for (x, y) in coords {
            let new_x = pos.0 as isize + x;
            let new_y = pos.1 as isize + y;
            if new_x < 0 || new_y < 0 {
                continue;
            }

            let new_pos = GridPos::new(new_x as usize, new_y as usize);
            if let Ok(mut new_v) = self.get(&new_pos) {
                if new_v == self.ending {
                    new_v = 'z' as u8;
                }
                let r = new_v as isize - cur_v as isize;
                if r <= 1 {
                    reachable.push(new_pos);
                }
            }
        }
        reachable
    }
}

/// Simple bruteforce BFS, could easily switch to Dijkstra
pub fn shortest_path(grid: &Grid, part1: bool) -> Option<Vec<GridPos>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let starts = grid.starting_pos(part1);
    for start in starts {
        let reachable = grid.reachable_squares(&start);
        for node in reachable.into_iter() {
            let mut path = vec![];
            path.push(node.clone());
            queue.push_back((node, path));
        }
    }

    while let Some((pos, path)) = queue.pop_front() {
        if visited.contains(&pos) { continue }

        let reachable = grid.reachable_squares(&pos);
        for node in reachable.into_iter() {
            let mut new_path = path.clone();
            new_path.push(node.clone());

            if grid.get(&node).unwrap() == grid.ending {
                return Some(new_path);
            }
            queue.push_back((node, new_path));
        }
        visited.insert(pos);
    }
    None
}

fn part1(input: &str) -> Result<usize> {
    let grid: Grid = input.parse()?;
    let sp = shortest_path(&grid, true).context("no shortest path")?;
    let steps = sp.len();
    Ok(steps)
}

fn part2(input: &str) -> Result<usize> {
    let grid: Grid = input.parse()?;
    let sp = shortest_path(&grid, false).context("no shortest path")?;
    let steps = sp.len();
    Ok(steps)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let shortest_steps1 = part1(input)?;
    println!("[*] Shortest steps 1: {}", shortest_steps1);

    let shortest_steps2 = part2(input)?;
    println!("[*] Shortest steps 2: {}", shortest_steps2);

    Ok(())
}
