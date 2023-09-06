use std::{str::FromStr, time::Instant, collections::HashSet};

use anyhow::{Context, Error, Result};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Cube {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl FromStr for Cube {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut splits = s.split(",").into_iter();
        let x = splits.next().context("no x")?.parse::<isize>()?;
        let y = splits.next().context("no y")?.parse::<isize>()?;
        let z = splits.next().context("no z")?.parse::<isize>()?;
        Ok(Cube { x, y, z })
    }
}

impl Cube {
    // Manhattan distance to determine if we're close
    pub fn is_adjacent(&self, other: &Self) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        let dz = (self.z - other.z).abs();

        dx == 1 && dy == 0 && dz == 0
            || dx == 0 && dy == 1 && dz == 0
            || dx == 0 && dy == 0 && dz == 1
    }
}

fn part1(input: &str) -> Result<usize> {
    let cubes: Vec<Cube> = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Cube>>>()?;

    let mut total_free_sides = 0;
    for cube in cubes.iter() {
        let mut free_sides = 6;
        for cube2 in cubes.iter().filter(|&c| c != cube) {
            if cube.is_adjacent(&cube2) {
                free_sides -= 1;
            }
        }
        total_free_sides += free_sides;
    }

    Ok(total_free_sides)
}

fn part2(input: &str) -> Result<usize> {
    let cubes: Vec<Cube> = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Cube>>>()?;


    let mut cur_cubes = HashSet::new();
    for c in cubes.iter() {
        cur_cubes.insert(c);
    }

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    let mut min_x = 0;
    let mut min_y = 0;
    let mut min_z = 0;
    for cube in cubes.iter() {
        max_x = cube.x.max(max_x);
        max_y = cube.y.max(max_x);
        max_z = cube.z.max(max_x);
        min_x = cube.x.min(min_x);
        min_y = cube.y.min(min_x);
        min_z = cube.z.min(min_x);
    }

    let mut bounding_cube = HashSet::new();
    for x in min_x-1..=max_x+1 {
        for y in min_y-1..=max_y+1 {
            for z in min_z-1..=max_z+1 {
                let c = Cube { x, y, z };
                bounding_cube.insert(c);
            }
        }
    }

    let mut queue = vec![Cube { x: min_x-1, y: min_x-1, z: min_x-1 }];
    while let Some(cube) = queue.pop() {
        let coords = [
            (1,0,0),
            (0,1,0),
            (0,0,1),
            (-1,0,0),
            (0,-1,0),
            (0,0,-1),
        ];
        for coord in coords.iter() {
            let c = Cube { x: cube.x + coord.0, y: cube.y + coord.1, z: cube.z + coord.2 };
            if c == cube { continue; }
            if bounding_cube.contains(&c) {
                if !cur_cubes.contains(&c) {
                    bounding_cube.remove(&c);
                    queue.push(c);
                }
            }
        }
    }

    let mut total_free_sides = 0;
    for cube in bounding_cube.iter() {
        let mut free_sides = 6;
        for cube2 in bounding_cube.iter().filter(|&c| c != cube) {
            if cube.is_adjacent(&cube2) {
                free_sides -= 1;
            }
        }
        total_free_sides += free_sides;
    }

    Ok(total_free_sides)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let instant = Instant::now();
    let free_sides = part1(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 1: {} ({:?})", free_sides, time);

    let instant = Instant::now();
    let free_sides = part2(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 2: {} ({:?})", free_sides, time);

    Ok(())
}
