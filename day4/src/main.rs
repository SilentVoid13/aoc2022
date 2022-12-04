use std::{ops::Range, str::FromStr};

use anyhow::{Context, Error, Result};

struct ElfPair(ElfRange, ElfRange);

struct ElfRange(Range<usize>);

impl FromStr for ElfPair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r1, r2) = s.split_once(",").context("invalid pair")?;
        Ok(ElfPair(r1.parse()?, r2.parse()?))
    }
}

impl FromStr for ElfRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("-").context("invalid range")?;
        let range = Range {
            start: a.parse()?,
            end: b.parse()?,
        };
        Ok(ElfRange(range))
    }
}

impl ElfPair {
    pub fn fully_contained(&self) -> bool {
        return self.0 .0.start <= self.1 .0.start && self.0 .0.end >= self.1 .0.end
            || self.1 .0.start <= self.0 .0.start && self.1 .0.end >= self.0 .0.end;
    }

    pub fn overlap(&self) -> bool {
        self.0 .0.end >= self.1 .0.start && self.1 .0.end >= self.0 .0.start
    }
}

fn part1(input: &str) -> Result<usize> {
    let pairs: Vec<ElfPair> = input.lines().flat_map(|l| l.parse()).collect();
    let fc = pairs.iter().filter(|p| p.fully_contained()).count();
    Ok(fc)
}

fn part2(input: &str) -> Result<usize> {
    let pairs: Vec<ElfPair> = input.lines().flat_map(|l| l.parse()).collect();
    let fc = pairs.iter().filter(|p| p.overlap()).count();
    Ok(fc)
}

fn main() -> Result<()> {
    let input = include_str!("../input");
    let fc = part1(input)?;
    let ov = part2(input)?;

    println!("[*] Fully contained pairs: {}", fc);
    println!("[*] Overlapped pairs: {}", ov);
    Ok(())
}
