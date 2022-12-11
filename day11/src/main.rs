use std::{str::FromStr, collections::VecDeque};

use anyhow::{anyhow, Context, Error, Result};
use ibig::{modular::ModuloRing, UBig};

pub struct Monkey {
    items: VecDeque<UBig>,
    visited: usize,
    operation: Operation,
    division: usize,
    throw1: usize,
    throw2: usize,
}

impl Monkey {
    pub fn new(
        items: VecDeque<UBig>,
        operation: Operation,
        test_div: usize,
        throw1: usize,
        throw2: usize,
    ) -> Self {
        Monkey {
            items,
            visited: 0,
            operation,
            division: test_div,
            throw1,
            throw2,
        }
    }
}

pub enum Operation {
    Add(UBig),
    Mult(UBig),
    Square,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut operation = s.trim_start().split(" ").skip(1);
        let operand = operation.next().context("invalid operand")?;
        let value = operation.next().context("invalid value")?.parse::<UBig>();
        let operation = match (operand, value) {
            ("+", Ok(v)) => Operation::Add(v),
            ("*", Ok(v)) => Operation::Mult(v),
            ("*", _) => Operation::Square,
            _ => return Err(anyhow!("invalid operation")),
        };
        Ok(operation)
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1);
        let (_, items) = lines
            .next()
            .context("no starting items")?
            .split_once(":")
            .context("invalid starting items")?;
        let items: VecDeque<UBig> = items
            .trim_start()
            .split(",")
            .flat_map(|i| i.trim_start().parse())
            .collect();

        let (_, operation) = lines
            .next()
            .context("no operation")?
            .split_once("=")
            .context("invalid operation")?;
        let operation = operation.parse()?;

        let test_div = lines
            .next()
            .context("no division test")?
            .split(" ")
            .last()
            .context("invalid division test")?
            .parse()?;

        let throw1 = lines
            .next()
            .context("no throw 1")?
            .split(" ")
            .last()
            .context("invalid throw 1")?
            .parse()?;

        let throw2 = lines
            .next()
            .context("no throw 2")?
            .split(" ")
            .last()
            .context("invalid throw 2")?
            .parse()?;

        Ok(Monkey::new(items, operation, test_div, throw1, throw2))
    }
}

impl Monkey {
    pub fn execute_operation(&self, ring: &ModuloRing, value: &UBig) -> UBig {
        let a = ring.from(value);
        let r = match &self.operation {
            Operation::Add(v) => a + ring.from(v),
            Operation::Mult(v) => a * ring.from(v),
            Operation::Square => a.clone() * a,
        };
        r.residue()
    }
}

pub fn play_keep_away<'a>(monkeys: &mut [Monkey], rounds: usize, divide: bool) {
    let len = monkeys.len();
    let common_modulo: usize = monkeys.iter().map(|m| m.division).product();
    let ring = ModuloRing::new(&UBig::from(common_modulo));

    for _ in 0..rounds {
        for i in 0..len {
            while let Some(item) = monkeys[i].items.pop_front() {
                let mut new_item = monkeys[i].execute_operation(&ring, &item);
                if divide {
                    new_item /= 3;
                }

                let new_mi = if &new_item % monkeys[i].division == 0 {
                    monkeys[i].throw1
                } else {
                    monkeys[i].throw2
                };
                monkeys[new_mi].items.push_back(new_item);
                monkeys[i].visited += 1;
            }
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let monkeys: Result<Vec<Monkey>> = input.split("\n\n").map(|l| l.parse()).collect();
    let mut monkeys = monkeys?;
    play_keep_away(monkeys.as_mut_slice(), 20, true);
    monkeys.sort_by(|a, b| b.visited.cmp(&a.visited));
    let monkey_business = monkeys[0].visited * monkeys[1].visited;
    Ok(monkey_business)
}

fn part2(input: &str) -> Result<usize> {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").flat_map(|l| l.parse()).collect();
    play_keep_away(monkeys.as_mut_slice(), 10_000, false);
    monkeys.sort_by(|a, b| b.visited.cmp(&a.visited));
    let monkey_business = monkeys[0].visited * monkeys[1].visited;
    Ok(monkey_business)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let monkey_business1 = part1(input)?;
    println!("[*] Monkey business level 1: {}", monkey_business1);

    let monkey_business2 = part2(input)?;
    println!("[*] Monkey business level 2: {}", monkey_business2);

    Ok(())
}
