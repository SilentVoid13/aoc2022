use std::convert::{TryFrom, TryInto};

use anyhow::{anyhow, Context, Error, Result};

#[derive(Debug, PartialEq)]
enum CommandType {
    Ls,
    Cd,
}

#[derive(Debug)]
struct Command<'a> {
    r#type: CommandType,
    arguments: Vec<&'a str>,
    output: &'a str,
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (prompt, output) = s.split_once("\n").context("invalid cmd")?;
        let prompt = prompt.trim_start();
        let (cmd_name, arguments) = match prompt.split_once(" ") {
            Some((a, b)) => (a, b.split(" ").collect::<Vec<&str>>()),
            None => (prompt, vec![]),
        };
        let cmd_type = match cmd_name {
            "ls" => CommandType::Ls,
            "cd" => CommandType::Cd,
            _ => return Err(anyhow!("invalid cmd type")),
        };
        Ok(Command {
            r#type: cmd_type,
            arguments,
            output,
        })
    }
}

fn parse_filesystem<'a>(commands: &[Command<'a>]) -> Result<Vec<(&'a str, usize)>> {
    let mut final_fs: Vec<(&str, usize)> = vec![];
    let mut tmp_fs: Vec<(&str, usize)> = vec![];

    for command in commands.iter() {
        match command.r#type {
            CommandType::Cd => {
                let loc = command.arguments.first().unwrap();
                if loc == &".." {
                    let (name, sum) = tmp_fs.pop().unwrap();
                    final_fs.push((name, sum));
                    tmp_fs.last_mut().unwrap().1 += sum;
                } else {
                    tmp_fs.push((loc, 0));
                }
            }
            CommandType::Ls => {
                for line in command.output.split("\n").filter(|l| !l.is_empty()) {
                    let (val, _) = line.split_once(" ").context("invalid output")?;

                    if val != "dir" {
                        let size = val.parse::<usize>()?;
                        tmp_fs.last_mut().unwrap().1 += size;
                    }
                }
            }
        }
    }
    for _ in 0..tmp_fs.len() {
        let (name, sum) = tmp_fs.pop().unwrap();
        final_fs.push((name, sum));
        if !tmp_fs.is_empty() {
            tmp_fs.last_mut().unwrap().1 += sum;
        }
    }

    Ok(final_fs)
}

fn part1(input: &str) -> Result<usize> {
    let commands: Vec<Command> = input
        .split("$")
        .skip(1)
        .flat_map(|s| s.try_into())
        .collect();
    let threshold = 100_000;
    let fs = parse_filesystem(&commands)?;
    let sum = fs.iter().map(|p| p.1).filter(|s| s <= &threshold).sum();
    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let commands: Vec<Command> = input
        .split("$")
        .skip(1)
        .flat_map(|s| s.try_into())
        .collect();
    let fs = parse_filesystem(&commands)?;

    let total_space = 70_000_000;
    let update_space = 30_000_000;
    let taken_space = fs.last().unwrap().1;
    let remaining_space = total_space - taken_space;
    let required_space = update_space - remaining_space;
    let deleted_space = fs.iter().map(|p| p.1).filter(|s| s >= &required_space).min().unwrap();

    Ok(deleted_space)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let sum_size = part1(input)?;
    let deleted_size = part2(input)?;

    println!("[*] Sum size of directories: {}", sum_size);
    println!("[*] Deleted size of directory: {}", deleted_size);

    Ok(())
}
