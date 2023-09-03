use std::{collections::HashSet, str::FromStr, time::Instant};

use anyhow::{anyhow, Context, Error, Result};
use itertools::Itertools;

#[derive(Debug)]
pub struct Grid {
    pub sensors: Vec<Sensor>,
    pub beacons: Vec<GridCoord>,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct GridCoord {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, PartialEq)]
pub struct Sensor {
    pub coord: GridCoord,
    pub closest_beacon: GridCoord,
    pub closest_beacon_dist: isize,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut sensors = vec![];
        let mut beacons = vec![];
        for line in s.lines() {
            let (sensor_str, beacon_str) = line.split_once(":").context("invalid line")?;
            let beacon = beacon_str.parse()?;
            let sensor_coord = sensor_str.parse()?;
            let sensor = Sensor {
                coord: sensor_coord,
                closest_beacon: beacon,
                closest_beacon_dist: sensor_coord.manhattan(&beacon),
            };
            sensors.push(sensor);
            beacons.push(beacon);
        }

        Ok(Grid { sensors, beacons })
    }
}

impl FromStr for GridCoord {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (_, x_str) = s.split_once("x=").context("invalid x coord")?;
        let (x_str, _) = x_str.split_once(",").context("invalid x coord")?;
        let x = x_str.parse()?;
        let (_, y_str) = s.split_once("y=").context("invalid y coord")?;
        let y = y_str.parse()?;

        Ok(GridCoord { x, y })
    }
}

impl GridCoord {
    pub fn manhattan(&self, coord: &Self) -> isize {
        (self.x - coord.x).abs() + (self.y - coord.y).abs()
    }
}

impl Grid {}

impl Sensor {
    pub fn compute_intersections(&self, other: &Self) -> Vec<GridCoord> {
        let mut intersections = vec![];

        let self_points = [
            (
                1,
                self.coord.y - (self.coord.x - (self.closest_beacon_dist + 1)),
            ),
            (
                1,
                self.coord.y - (self.coord.x + (self.closest_beacon_dist + 1)),
            ),
            (
                -1,
                self.coord.y + (self.coord.x - (self.closest_beacon_dist + 1)),
            ),
            (
                -1,
                self.coord.y + (self.coord.x + (self.closest_beacon_dist + 1)),
            ),
        ];

        let other_points = [
            (
                1,
                other.coord.y - (other.coord.x - (other.closest_beacon_dist + 1)),
            ),
            (
                1,
                other.coord.y - (other.coord.x + (other.closest_beacon_dist + 1)),
            ),
            (
                -1,
                other.coord.y + (other.coord.x - (other.closest_beacon_dist + 1)),
            ),
            (
                -1,
                other.coord.y + (other.coord.x + (other.closest_beacon_dist + 1)),
            ),
        ];

        for (p1, p2) in self_points.iter().cartesian_product(other_points.iter()) {
            let (a1, b1) = p1;
            let (a2, b2) = p2;

            // Parallel lines
            if a1 == a2 {
                continue;
            }

            let x = (b2 - b1) / (a1 - a2);
            let y = a1 * (x as isize) + b1;
            intersections.push(GridCoord { x, y });
        }

        intersections
    }
}

fn part1(input: &str) -> Result<usize> {
    let grid: Grid = input.parse()?;
    let mut count = 0;
    let count_y = 2000000;

    let mut visited_x: HashSet<isize> = HashSet::new();
    for sensor in grid.sensors.iter() {
        let mut queue = vec![(sensor.coord.x, true), (sensor.coord.x + 1, false)];
        while let Some((coord_x, left)) = queue.pop() {
            let coord = GridCoord {
                x: coord_x,
                y: count_y,
            };
            // Free square
            if coord.manhattan(&sensor.coord) > sensor.closest_beacon_dist {
                continue;
            }

            if left {
                queue.push((coord_x - 1, left));
            } else {
                queue.push((coord_x + 1, left));
            }

            // Special case: beacons are not considered taken squares
            if coord == sensor.closest_beacon {
                continue;
            }

            if !visited_x.contains(&coord_x) {
                visited_x.insert(coord_x);
                count += 1;
            }
        }
    }

    Ok(count)
}

fn part2(input: &str) -> Result<isize> {
    let grid: Grid = input.parse()?;
    let min_val = 0;
    let max_val = 4000000;

    // We know that only one square is possible for the distress beacon on the whole map
    // At least 2 diamond areas are necessary to isolate a single point
    // Thus our beacon is a point on the line of a diamond + 1
    // Each line of a diamond has a simple ax+b equation
    // We can compute the intersections of all the lines of the diamond areas + 1 to find the
    // beacon

    let mut points: HashSet<GridCoord> = HashSet::new();
    for sensor in grid.sensors.iter() {
        for sensor2 in grid.sensors.iter().filter(|&s| !(s == sensor)) {
            let intersections = sensor.compute_intersections(&sensor2);
            for p in intersections
                .into_iter()
                .filter(|p| p.x >= min_val && p.y >= min_val && p.x <= max_val && p.y <= max_val)
            {
                points.insert(p);
            }
        }
    }

    for p in points {
        let mut free = true;
        for sensor in grid.sensors.iter() {
            if p.manhattan(&sensor.coord) <= sensor.closest_beacon_dist {
                free = false;
                break;
            }
        }
        if free {
            return Ok(p.x * 4000000 + p.y);
        }
    }

    Err(anyhow!("not found"))
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let start = Instant::now();
    let count = part1(input)?;
    let time = start.elapsed();
    println!("[*] part 1: {} ({:?})", count, time);

    let start = Instant::now();
    let freq = part2(input)?;
    let time = start.elapsed();
    println!("[*] part 2: {} ({:?})", freq, time);

    Ok(())
}
