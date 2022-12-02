use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

enum Shape {
    Rock,
    Paper,
    Scissor,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Shape {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shape = match s {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissor,
            _ => return Err(anyhow!("invalid shape")),
        };
        Ok(shape)
    }
}

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outcome = match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => return Err(anyhow!("invalid outcome")),
        };
        Ok(outcome)
    }
}

impl Shape {
    pub fn score(shape: &Shape) -> usize {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }
}

impl Outcome {
    pub fn score(play1: &Shape, play2: &Outcome) -> usize {
        match (play1, play2) {
            (Shape::Rock, Outcome::Lose) => Shape::score(&Shape::Scissor),
            (Shape::Rock, Outcome::Win) => Shape::score(&Shape::Paper),
            (Shape::Scissor, Outcome::Lose) => Shape::score(&Shape::Paper),
            (Shape::Scissor, Outcome::Win) => Shape::score(&Shape::Rock),
            (Shape::Paper, Outcome::Lose) => Shape::score(&Shape::Rock),
            (Shape::Paper, Outcome::Win) => Shape::score(&Shape::Scissor),
            (_, Outcome::Draw) => Shape::score(play1),
        }
    }
}

fn duel_score1(play1: &Shape, play2: &Shape) -> usize {
    match (play1, play2) {
        (Shape::Rock, Shape::Scissor) => 0,
        (Shape::Rock, Shape::Paper) => 6,
        (Shape::Scissor, Shape::Rock) => 6,
        (Shape::Scissor, Shape::Paper) => 0,
        (Shape::Paper, Shape::Scissor) => 6,
        (Shape::Paper, Shape::Rock) => 0,
        _ => 3,
    }
}

fn duel_score2(play: &Outcome) -> usize {
    match play {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut final_score = 0;
    for line in input.lines() {
        let (a, b) = line.split_once(" ").context("invalid duel")?;
        let play1 = a.parse::<Shape>()?;
        let play2 = b.parse::<Shape>()?;
        final_score += Shape::score(&play2) + duel_score1(&play1, &play2);
    }
    Ok(final_score)
}

fn part2(input: &str) -> Result<usize> {
    let mut final_score = 0;
    for line in input.lines() {
        let (a, b) = line.split_once(" ").context("invalid duel")?;
        let play1 = a.parse::<Shape>()?;
        let play2 = b.parse::<Outcome>()?;
        final_score += Outcome::score(&play1, &play2) + duel_score2(&play2);
    }
    Ok(final_score)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let final_score1 = part1(input)?;
    let final_score2 = part2(input)?;

    println!("[*] Final score 1:  {}", final_score1);
    println!("[*] Final score 2:  {}", final_score2);

    Ok(())
}
