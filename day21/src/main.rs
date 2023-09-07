use std::cell::RefCell;
use std::{collections::HashMap, str::FromStr, time::Instant};

use anyhow::{anyhow, Context, Error, Result};

type MonkeyId = String;

#[derive(Debug)]
struct Riddle {
    pub monkeys: HashMap<MonkeyId, RefCell<Monkey>>,
}

#[derive(Debug, Clone)]
struct Monkey {
    pub id: MonkeyId,
    pub job: Job,
}

#[derive(Debug, Clone)]
struct Job {
    pub res: Option<f64>,
    pub operation: Option<Operation>,
}

#[derive(Debug, Clone)]
struct Operation {
    pub typ: OpType,
    pub m1: MonkeyId,
    pub m2: MonkeyId,
}

#[derive(Debug, Clone)]
enum OpType {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (id, _) = s.split_once(":").context("invalid name")?;
        let (_, job) = s.split_once(" ").context("invalid job")?;

        let job = if let Ok(v) = job.parse::<f64>() {
            Job {
                res: Some(v),
                operation: None,
            }
        } else {
            let mut splits = job.split(" ");
            let m1 = splits.next().unwrap();
            let op = splits.next().unwrap();
            let m2 = splits.next().unwrap();
            let m1 = m1.to_string();
            let m2 = m2.to_string();
            let typ = match op {
                "+" => OpType::Add,
                "-" => OpType::Sub,
                "*" => OpType::Mul,
                "/" => OpType::Div,
                _ => return Err(anyhow!("invalid job")),
            };
            Job {
                res: None,
                operation: Some(Operation { typ, m1, m2 }),
            }
        };

        Ok(Monkey {
            id: id.to_string(),
            job,
        })
    }
}

impl Riddle {
    pub fn pass(&mut self) -> bool {
        let mut moved = false;
        for monkey in self.monkeys.values().filter(|m| m.borrow().job.res.is_none()) {
            let mut monkey = monkey.borrow_mut();
            if let Some(op) = &monkey.job.operation {
                if let OpType::Eq = op.typ {
                    continue;
                }
                let res1 = self.monkeys[&op.m1].borrow().job.res;
                let res2 = self.monkeys[&op.m2].borrow().job.res;
                match (res1, res2) {
                    (Some(r1), Some(r2)) => {
                        match op.typ {
                            OpType::Mul => {
                                monkey.job.res = Some(r1 * r2)
                            },
                            OpType::Div => monkey.job.res = Some(r1 / r2),
                            OpType::Add => monkey.job.res = Some(r1 + r2),
                            OpType::Sub => monkey.job.res = Some(r1 - r2),
                            _ => unreachable!(),
                        };
                        moved = true;
                    },
                    _ => {},
                };
            }
        }
        moved
    }

    pub fn dfs_solve_x(&self, m: RefCell<Monkey>, exp_value: f64) -> f64 {
        let monkey = m.borrow_mut();
        if monkey.id == "humn" {
            return exp_value;
        }
        let op = monkey.job.operation.as_ref().unwrap();

        let m1 = &self.monkeys[&op.m1];
        let m2 = &self.monkeys[&op.m2];
        let monkey1 = m1.borrow();
        let monkey2 = m2.borrow();
        let (v, child_m, left) = match (monkey1.job.res, monkey2.job.res) {
            (None, Some(v)) => (v, m1, true),
            (Some(v), None) => (v, m2, false),
            _ => unreachable!("two unknown childs")
        };

        let new_v = match op.typ {
            OpType::Eq => {
                v
            }
            OpType::Add => {
                exp_value - v
            },
            OpType::Sub => {
                if left {
                    exp_value + v
                } else {
                    v - exp_value
                }
            },
            OpType::Mul => {
                exp_value / v
            },
            OpType::Div => {
                if left {
                    exp_value * v
                } else {
                    v / exp_value
                }
            },
        };
        return self.dfs_solve_x(child_m.clone(), new_v);
    }
}

fn part1(input: &str) -> Result<f64> {
    let mut monkeys: HashMap<MonkeyId, RefCell<Monkey>> = HashMap::new();
    for l in input.lines() {
        let m: Monkey = l.parse()?;
        monkeys.insert(m.id.clone(), RefCell::new(m));
    }
    let mut riddle = Riddle { monkeys };
    while riddle.pass() {}
    let res = riddle.monkeys["root"].borrow().job.res.unwrap();
    Ok(res)
}

fn part2(input: &str) -> Result<f64> {
    let mut monkeys: HashMap<MonkeyId, RefCell<Monkey>> = HashMap::new();
    for l in input.lines() {
        let m: Monkey = l.parse()?;
        monkeys.insert(m.id.clone(), RefCell::new(m));
    }
    monkeys["root"].borrow_mut().job.operation.as_mut().unwrap().typ = OpType::Eq;
    monkeys["humn"].borrow_mut().job.operation = None;
    monkeys["humn"].borrow_mut().job.res = None;

    let mut riddle = Riddle { monkeys };
    while riddle.pass() {}
    let x = riddle.dfs_solve_x(riddle.monkeys["root"].clone(), 0.0);
    Ok(x)

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
