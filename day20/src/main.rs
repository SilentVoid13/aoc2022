use std::{time::Instant, collections::VecDeque};

use anyhow::{Context, Result};

#[derive(Debug)]
struct CircularList {
    elements: VecDeque<isize>,
}

impl CircularList {
    pub fn mix(&mut self, iterations: usize) {
        let mut indexes = VecDeque::from_iter(0..self.elements.len());

        for _ in 0..iterations {
            for i in 0..self.elements.len() {
                let cur_idx = indexes.iter().position(|&e| e == i).unwrap();

                indexes.rotate_left(cur_idx);
                self.elements.rotate_left(cur_idx);
                let elt = self.elements.pop_front().unwrap();
                indexes.pop_front();

                // .abs() is important to prevent modifying non-overlapping negative numbers
                let rotations = self.offset_idx(elt.abs(), 0);
                if elt.is_positive() {
                    self.elements.rotate_left(rotations);
                    indexes.rotate_left(rotations);
                } else {
                    self.elements.rotate_right(rotations);
                    indexes.rotate_right(rotations);
                }
                self.elements.push_front(elt);
                indexes.push_front(i);
            }
        }
    }

    pub fn offset_idx(&self, idx: isize, offset: isize) -> usize {
        (idx + offset).rem_euclid(self.elements.len() as isize) as usize
    }

    pub fn get_coords(&self) -> Option<(isize, isize, isize)> {
        let zero_idx = self.elements.iter().position(|&e| e == 0)?;
        let x = self.elements[self.offset_idx(zero_idx as isize, 1000)];
        let y = self.elements[self.offset_idx(zero_idx as isize, 2000)];
        let z = self.elements[self.offset_idx(zero_idx as isize, 3000)];
        Some((x, y, z))
    }
}

fn part1(input: &str) -> Result<isize> {
    let elements = input
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<VecDeque<isize>>();

    let mut list = CircularList { elements };
    list.mix(1);
    let coords = list.get_coords().context("failed to get coords")?;
    Ok(coords.0 + coords.1 + coords.2)
}

fn part2(input: &str) -> Result<isize> {
    let elements = input
        .lines()
        .map(|l| l.parse::<isize>().unwrap() * 811589153)
        .collect::<VecDeque<isize>>();

    let mut list = CircularList { elements };
    list.mix(10);
    let coords = list.get_coords().context("failed to get coords")?;
    Ok(coords.0 + coords.1 + coords.2)

}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let instant = Instant::now();
    let coords_sum = part1(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 1: {} ({:?})", coords_sum, time);

    let instant = Instant::now();
    let coords_sum = part2(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 2: {} ({:?})", coords_sum, time);

    Ok(())
}
