use std::time::Instant;

use anyhow::{Context, Error, Result};

fn part1(input: &str) -> Result<usize> {
    Ok(0)
}

fn part2(input: &str) -> Result<usize> {
    Ok(0)
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

