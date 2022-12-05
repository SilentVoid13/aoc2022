use anyhow::{Context, Error, Result};
use std::{cell::RefCell, str::FromStr};

#[derive(Debug)]
struct Stacks(Vec<Stack>);

#[derive(Debug)]
struct Stack {
    pub crates: Vec<u8>,
}

#[derive(Debug)]
struct Instruction {
    pub n: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Stacks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = vec![];

        for l in s.lines().rev().skip(1) {
            if stacks.is_empty() {
                let nstacks = (l.len() + 1) / 4;
                for _ in 0..nstacks {
                    stacks.push(Stack { crates: vec![] });
                }
            }
            for (i, chunk) in l.as_bytes().chunks(4).enumerate() {
                let val = chunk.get(1).copied().context("no val")?;
                if val == ' ' as u8 {
                    continue;
                }
                let stack = stacks.get_mut(i).context("no stack")?;
                stack.crates.push(val);
            }
        }
        Ok(Stacks(stacks))
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s
            .split("move ")
            .skip(1)
            .flat_map(|l| l.split(" from "))
            .flat_map(|l| l.split(" to "))
            .collect();
        let n = splits.get(0).context("no n")?.parse()?;
        let from = splits.get(1).context("no from")?.parse::<usize>()? - 1usize;
        let to = splits.get(2).context("no to")?.parse::<usize>()? - 1usize;

        Ok(Instruction { n, from, to })
    }
}

impl Stacks {
    pub fn execute_instructions_9000(&mut self, instructions: &Vec<Instruction>) -> Result<()> {
        for instruction in instructions.iter() {
            for _ in 0..instruction.n {
                let from = self.0.get_mut(instruction.from).context("invalid from")?;
                let val = from.crates.pop().context("no more crates")?;
                let to = self.0.get_mut(instruction.to).context("invalid from")?;
                to.crates.push(val);
            }
        }

        Ok(())
    }

    pub fn execute_instructions_9001(&mut self, instructions: &Vec<Instruction>) -> Result<()> {
        for instruction in instructions.iter() {
            let from = self.0.get_mut(instruction.from).context("invalid from")?;
            let mut values: Vec<u8> = from.crates.iter().rev().copied().take(instruction.n).rev().collect();
            from.crates.truncate(from.crates.len() - values.len());
            let to = self.0.get_mut(instruction.to).context("invalid from")?;
            to.crates.append(&mut values);
        }

        Ok(())
    }
}

fn get_top_crates(stacks: &Stacks) -> Vec<u8> {
    stacks
        .0
        .iter()
        .filter(|s| !s.crates.is_empty())
        .map(|s| s.crates.last().copied().unwrap())
        .collect()
}

fn part1(input: &str) -> Result<String> {
    let (stacks, inst) = input.split_once("\n\n").context("invalid input")?;
    let mut stacks: Stacks = stacks.parse()?;
    let instructions: Vec<Instruction> = inst.lines().flat_map(|l| l.parse()).collect();
    stacks.execute_instructions_9000(&instructions)?;
    let top_crates = get_top_crates(&stacks);
    Ok(String::from_utf8(top_crates)?)
}

fn part2(input: &str) -> Result<String> {
    let (stacks, inst) = input.split_once("\n\n").context("invalid input")?;
    let mut stacks: Stacks = stacks.parse()?;
    let instructions: Vec<Instruction> = inst.lines().flat_map(|l| l.parse()).collect();
    stacks.execute_instructions_9001(&instructions)?;
    let top_crates = get_top_crates(&stacks);
    Ok(String::from_utf8(top_crates)?)
}

fn main() -> Result<()> {
    let input = include_str!("../input");
    let top_crates_9000 = part1(input)?;
    let top_crates_9001 = part2(input)?;
    println!("[*] Top crates 9000: {}", top_crates_9000);
    println!("[*] Top crates 9001: {}", top_crates_9001);

    Ok(())
}
