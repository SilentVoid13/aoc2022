use anyhow::{Context, Result};

fn get_calories_sum(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|c| c.lines().map(|v| v.parse::<u32>().unwrap()).sum())
        .collect()
}

fn part1(input: &str) -> Result<u32> {
    let sum_calories = get_calories_sum(input);
    let v = sum_calories.iter().max().context("max err")?;
    Ok(*v)
}

fn part2(input: &str) -> Result<u32> {
    let mut sum_calories = get_calories_sum(input);
    sum_calories.sort_by(|x, y| y.cmp(x));
    let v = sum_calories.iter().take(3).sum();
    Ok(v)
}

fn main() -> Result<()> {
    let input = include_str!("../input");
    let max_cal_sum = part1(input)?;
    let max_3cal_sum = part2(input)?;
    println!("[*] Biggest calories sum: {}", max_cal_sum);
    println!("[*] Biggest top 3 calories sum: {}", max_3cal_sum);

    Ok(())
}
