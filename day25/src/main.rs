use std::time::Instant;

use anyhow::{Context, Error, Result};

fn snafu_decode(l: &str) -> i64 {
    let mut total = 0;
    for (i, c) in l.chars().rev().enumerate() {
        let v = match c {
            '=' => -2 * 5_i64.pow(i as u32),
            '-' => -1 * 5_i64.pow(i as u32),
            '0' => 0,
            '1' => 5_i64.pow(i as u32),
            '2' => 2 * 5_i64.pow(i as u32),
            _ => unreachable!()
        };
        total += v;
    }
    total
}

fn snafu_encode(v: i64) -> String {
    let mut s = vec![];
    let mut res = v;
    while res != 0 {
        let rem = res % 5;
        res = (res as f64 / 5.0).round() as i64;
        let c = match rem {
            4 => '-',
            3 => '=',
            2 => '2',
            1 => '1',
            0 => '0',
            _ => unreachable!()
        };
        s.push(c);
    }
    s.reverse();
    String::from_iter(s.iter())
}

fn part1(input: &str) -> Result<String> {
    let sum: i64 = input.lines().map(|l| snafu_decode(l)).sum();
    let v = snafu_encode(sum);
    Ok(v)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let instant = Instant::now();
    let res = part1(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 1: {} ({:?})", res, time);

    Ok(())
}
