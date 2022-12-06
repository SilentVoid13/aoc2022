use anyhow::{anyhow, Context, Error, Result};
use std::collections::HashSet;

fn find_marker(input: &str, size: usize) -> Result<usize> {
    for (i, win) in input.as_bytes().windows(size).enumerate() {
        let mut set = HashSet::new();
        let unique = win.into_iter().all(|v| set.insert(v));
        if unique {
            return Ok(i + win.len());
        }
    }
    Err(anyhow!("no packet found"))
}

fn part1(input: &str) -> Result<usize> {
    find_marker(input, 4)
}

fn part2(input: &str) -> Result<usize> {
    find_marker(input, 14)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let first_marker = part1(input)?;
    let first_msg = part2(input)?;
    println!("[*] First marker index: {}", first_marker);
    println!("[*] First message index: {}", first_msg);

    Ok(())
}
