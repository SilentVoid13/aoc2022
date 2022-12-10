use std::{str::FromStr, fmt::Display};

use anyhow::{anyhow, Context, Error, Result};

struct Cpu {
    register: isize,
    total_cycles: isize,
    crt: Crt,
}

#[derive(Debug)]
struct Instruction {
    value: InstructionType,
    cycles: isize,
}

#[derive(Debug)]
enum InstructionType {
    AddX(isize),
    Nop,
}

struct Crt {
    pixels: [[char; 40]; 6],
    cur_i: usize,
    cur_row: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(" ");
        let n = splits.next().context("no name")?;
        let i = match n {
            "addx" => {
                let v = splits.next().context("no add value")?.parse()?;
                let t = InstructionType::AddX(v);
                Instruction::new(t, 2)
            }
            "noop" => {
                let t = InstructionType::Nop;
                Instruction::new(t, 1)
            }
            _ => return Err(anyhow!("invalid instruction name")),
        };
        Ok(i)
    }
}

impl Instruction {
    pub fn new(value: InstructionType, cycles: isize) -> Self {
        Instruction { value, cycles }
    }
}

impl Crt {
    pub fn new() -> Self {
        Crt { pixels: [['.'; 40]; 6], cur_i: 0, cur_row: 0 }
    }

    pub fn draw_pixel(&mut self, sprite_pos: isize, cycles: isize) {
        for _ in 0..cycles {
            let range = sprite_pos - self.cur_i as isize;
            if range >= -1 && range <= 1 {
                self.pixels[self.cur_row][self.cur_i] = '#';
            }
            self.cur_i += 1;
            if self.cur_i % 40 == 0 {
                self.cur_row += 1;
                self.cur_i = 0;
            }
        }
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.pixels.iter() {
            write!(f, "{}\n", String::from_iter(row.iter()))?;
        }
        Ok(())
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register: 1,
            total_cycles: 0,
            crt: Crt::new(),
        }
    }

    pub fn execute_instructions(
        &mut self,
        instructions: &[Instruction],
        sig_idxs: &[isize],
    ) -> Vec<isize> {
        let mut res = vec![];
        let mut sig_idxs = sig_idxs.into_iter();
        let mut sig_idx = sig_idxs.next();

        for instruction in instructions.iter() {
            self.crt.draw_pixel(self.register, instruction.cycles);
            let new_cycle = self.total_cycles + instruction.cycles;
            if let Some(si) = sig_idx {
                if *si <= new_cycle {
                    let sig_strength = self.register * si;
                    res.push(sig_strength);
                    sig_idx = sig_idxs.next();
                }
            }
            self.total_cycles = new_cycle;
            if let InstructionType::AddX(n) = instruction.value {
                self.register += n;
            }
        }

        res
    }
}

fn part1(input: &str) -> Result<isize> {
    let instructions: Vec<Instruction> = input.lines().flat_map(|l| l.parse()).collect();
    let sig_idxs = [20, 60, 100, 140, 180, 220];
    let mut cpu = Cpu::new();
    let sig = cpu.execute_instructions(&instructions, &sig_idxs);
    Ok(sig.iter().sum())
}

fn part2(input: &str) -> Result<Crt> {
    let instructions: Vec<Instruction> = input.lines().flat_map(|l| l.parse()).collect();
    let sig_idxs = [20, 60, 100, 140, 180, 220];
    let mut cpu = Cpu::new();
    let _ = cpu.execute_instructions(&instructions, &sig_idxs);
    Ok(cpu.crt)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let sig_sum = part1(input)?;
    let crt = part2(input)?;

    println!("[*] Signals sum: {}", sig_sum);
    println!("[*] CRT Output:\n\n{}", crt);

    Ok(())
}
