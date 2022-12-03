use anyhow::{anyhow, Error, Result};
use std::{collections::HashSet, str::FromStr};

struct Rucksack {
    compartment1: HashSet<char>,
    compartment2: HashSet<char>,
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let middle = s.len() / 2;
        let (c1, c2) = s.split_at(middle);
        let h1 = HashSet::from_iter(c1.chars());
        let h2 = HashSet::from_iter(c2.chars());
        Ok(Rucksack {
            compartment1: h1,
            compartment2: h2,
        })
    }
}

impl Rucksack {
    pub fn all_items(&self) -> HashSet<char> {
        self.compartment1.union(&self.compartment2).copied().collect()
    }

    pub fn compartments_common_items(&self) -> HashSet<char> {
        self.compartment1
            .intersection(&self.compartment2)
            .copied()
            .collect()
    }

    pub fn item_priority(c: char) -> Result<usize> {
        let p = match c {
            'a'..='z' => c as usize - 'a' as usize + 1,
            'A'..='Z' => c as usize - 'A' as usize + 27,
            _ => return Err(anyhow!("invalid item value")),
        };
        Ok(p)
    }
}

fn part1(input: &str) -> Result<usize> {
    let rucksacks: Vec<Rucksack> = input.lines().flat_map(|l| l.parse()).collect();
    let mut priority_sum = 0;
    for rucksack in rucksacks.iter() {
        for item in rucksack.compartments_common_items().iter().copied() {
            priority_sum += Rucksack::item_priority(item)?;
        }
    }
    Ok(priority_sum)
}

fn part2(input: &str) -> Result<usize> {
    let rucksacks: Vec<Rucksack> = input.lines().flat_map(|l| l.parse()).collect();
    let mut priority_sum = 0;
    for group_rucksacks in rucksacks.chunks(3) {
        let first_items = group_rucksacks[0].all_items();
        let common_items = group_rucksacks.iter().skip(1).fold(first_items, |acc, x| {
            acc.intersection(&x.all_items()).copied().collect()
        });
        for item in common_items.iter().copied() {
            priority_sum += Rucksack::item_priority(item)?;
        }
    }
    Ok(priority_sum)
}

fn main() -> Result<()> {
    let input = include_str!("../input");
    let priority_sum = part1(input)?;
    let priority_sum_grp = part2(input)?;

    println!("[*] Priority sum: {}", priority_sum);
    println!("[*] Priority sum by group: {}", priority_sum_grp);

    Ok(())
}
