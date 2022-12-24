use std::{str::{FromStr, Split}, borrow::BorrowMut, collections::VecDeque};

use anyhow::{Context, Error, Result};

struct Grid {
    tiles: Vec<Tile>,
    source: GridCoord,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Debug)]
struct GridCoord {
    x: usize,
    y: usize,
}

#[derive(Clone)]
enum Tile {
    Air,
    Rock,
    SandSource,
    Sand,
}

enum MoveStatus {
    Moved,
    NoMove,
    Finished,
}

impl FromStr for GridCoord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(" -> ").next().unwrap();
        let (x, y) = s.split_once(",").context("invalid coord")?;
        let x = x.parse()?;
        let y = y.parse()?;
        Ok(GridCoord { x, y })
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut max_width = 500;
        let mut min_width = 500;
        let mut height = 0;
        let mut paths = vec![];

        for line in s.lines() {
            let splits: Vec<&str> = line.split(" -> ").collect();
            for coords in splits.windows(2) {
                let coord1: GridCoord = coords[0].parse()?;
                let coord2: GridCoord = coords[1].parse()?;
                max_width = max_width.max(coord1.x).max(coord2.x);
                min_width = min_width.min(coord1.x).min(coord2.x);
                height = height.max(coord1.y).max(coord2.y);

                paths.push((coord1, coord2));
            }
        }
        height += 1;
        max_width += 1;

        let width = max_width - min_width;
        let mut tiles = vec![Tile::Air; width * height];

        let source_x = 500 - min_width;
        let source = GridCoord { x: source_x, y: 0 };
        tiles[source.x] = Tile::SandSource;

        for path in paths.iter() {
            let x1 = path.0.x - min_width;
            let x2 = path.1.x - min_width;
            let y1 = path.0.y;
            let y2 = path.1.y;

            for x in x1.min(x2)..=x1.max(x2) {
                tiles[width * path.0.y + x] = Tile::Rock;
            }
            for y in y1.min(y2)..=y1.max(y2) {
                tiles[width * y + x1] = Tile::Rock;
            }
        }

        Ok(Grid {
            tiles,
            source,
            width,
            height,
        })
    }
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Grid {
    pub fn in_bounds(&self, c: GridCoord) -> bool {
        c.x < self.width && c.y < self.height
    }

    pub fn tile(&self, c: GridCoord) -> Option<&Tile> {
        if !self.in_bounds(c) {
            return None;
        }
        Some(&self.tiles[self.width * c.y + c.x])
    }

    pub fn tile_mut(&mut self, c: GridCoord) -> Option<&mut Tile> {
        if !self.in_bounds(c) {
            return None;
        }
        Some(&mut self.tiles[self.width * c.y + c.x])
    }

    pub fn step(&mut self) -> bool {
        let mut c = self.source;
        let new_sand = self.tile_mut(c).unwrap();
        if let Tile::Sand = new_sand {
            return false;
        }
        *new_sand = Tile::Sand;

        let mut rest = false;
        while !rest {
            let coords = [(c.x, c.y + 1), (c.x - 1, c.y + 1), (c.x + 1, c.y + 1)];
            for coord in coords {
                let new_c = coord.into();
                let tile = self.tile(new_c);
                match tile {
                    Some(Tile::Air) => {
                        let old_tile = self.tile_mut(c).unwrap();
                        *old_tile = Tile::Air;

                        let new_tile = self.tile_mut(new_c).unwrap();
                        *new_tile = Tile::Sand;

                        c = new_c;
                        rest = false;
                        break;
                    }
                    Some(_) => rest = true,
                    None => {
                        let old_tile = self.tile_mut(c).unwrap();
                        *old_tile = Tile::Air;
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn toggle_floor(&mut self) {
        let sup = self.width * 2;
        let new_height = self.height + 2;
        let new_width = self.width + sup * 2;

        let mut tiles = vec![Tile::Air; new_width * new_height];

        for y in 0..self.height {
            for x in 0..self.width {
                let c = (x, y).into();
                tiles[y * new_width + (x + sup)] = self.tile(c).unwrap().clone();
            }
        }
        for x in 0..new_width {
            tiles[new_width * (new_height - 1) + x] = Tile::Rock;
        }

        self.width = new_width;
        self.height = new_height;
        self.source.x += sup;
        self.tiles = tiles;
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}x{} grid:", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.tile((x, y).into()).unwrap();
                let c = match c {
                    Tile::SandSource => '+',
                    Tile::Air => '.',
                    Tile::Rock => '#',
                    Tile::Sand => 'o',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut grid: Grid = input.parse()?;
    let mut i = 0;
    while grid.step() {
        i += 1;
    }
    //println!("{:?}", grid);
    Ok(i)
}

fn part2(input: &str) -> Result<usize> {
    let mut grid: Grid = input.parse()?;
    grid.toggle_floor();
    let mut i = 0;
    while grid.step() {
        i += 1;
    }
    //println!("{:?}", grid);
    Ok(i)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let sand_units = part1(input)?;
    println!("[*] Sand units: {}", sand_units);

    let sand_units = part2(input)?;
    println!("[*] Sand units 2: {}", sand_units);

    Ok(())
}
