use std::{str::FromStr, cmp::Ordering};

use anyhow::{anyhow, Context, Error, Result};

#[derive(Debug, Clone)]
struct Packets {
    values: Vec<Value>,
    p_size: usize,
}

#[derive(Debug, Clone)]
enum Value {
    List(Vec<Value>),
    Integer(usize),
}

#[derive(Debug)]
enum Comp {
    True,
    Neutral,
    False,
}

impl FromStr for Packets {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.as_bytes().into_iter().peekable();

        let first = chars.next().context("no opening bracket")?;
        if *first != b'[' {
            return Err(anyhow!("invalid opening char"));
        }

        let mut p_size = 1;
        let mut n_size = 0;
        let mut values = vec![];

        while let Some(c) = chars.peek().copied() {
            if *c == b',' {
                p_size += 1;
                chars.next().unwrap();
                continue;
            }
            if *c == b']' {
                p_size += 1;
                break;
            }

            match c {
                b'[' => {
                    let sub: Packets = s[p_size..].parse()?;
                    p_size += sub.p_size;
                    chars.nth(sub.p_size - 1);
                    values.push(Value::List(sub.values))
                }
                b'0'..=b'9' => {
                    p_size += 1;
                    chars.next().unwrap();
                    match chars.peek() {
                        Some(b'0'..=b'9') => {
                            n_size += 1;
                        }
                        Some(_) => {
                            let i = &s[p_size - n_size - 1..p_size];
                            n_size = 0;
                            let v = i.parse()?;
                            values.push(Value::Integer(v));
                        }
                        _ => (),
                    }
                }
                _ => return Err(anyhow!("invalid value")),
            }
        }
        Ok(Packets { values, p_size })
    }
}

impl Comp {
    pub fn ord(&self) -> Ordering {
        match self {
            Comp::True => Ordering::Less,
            Comp::Neutral => Ordering::Equal,
            Comp::False => Ordering::Greater,
        }
    }
}

impl Value {
    pub fn compare(v1: &Value, v2: &Value) -> Comp {
        match (v1, v2) {
            (Value::Integer(i1), Value::Integer(i2)) => {
                if i1 < i2 {
                    return Comp::True;
                } else if i1 > i2 {
                    return Comp::False;
                }
            }
            (Value::List(l1), Value::List(l2)) => {
                for (e1, e2) in l1.iter().zip(l2.iter()) {
                    let c = Value::compare(e1, e2);
                    match c {
                        Comp::False | Comp::True => return c,
                        _ => (),
                    }
                }
                if l2.len() < l1.len() {
                    return Comp::False;
                } else if l1.len() < l2.len() {
                    return Comp::True;
                }
            }
            (Value::List(_), Value::Integer(_)) => {
                return Value::compare(v1, &Value::List(vec![v2.clone()]));
            }
            (Value::Integer(_), Value::List(_)) => {
                return Value::compare(&Value::List(vec![v1.clone()]), v2);
            }
        }
        return Comp::Neutral;
    }
}

impl Packets {
    pub fn compare(p1: &Packets, p2: &Packets) -> Comp {
        for (v1, v2) in p1.values.iter().zip(p2.values.iter()) {
            let c = Value::compare(v1, v2);
            match c {
                Comp::False | Comp::True => return c,
                _ => (),
            }
        }
        if p2.values.len() < p1.values.len() {
            return Comp::False;
        } else if p1.values.len() < p2.values.len() {
            return Comp::True;
        }
        Comp::Neutral
    }
}

fn part1(input: &str) -> Result<usize> {
    let packets: Vec<Packets> = input
        .split("\n\n")
        .flat_map(|l| l.split("\n"))
        .filter(|s| !s.is_empty())
        .map(|p| p.parse().unwrap())
        .collect();
    let mut sum = 0;
    for (i, chunk) in packets.chunks(2).enumerate() {
        let v = Packets::compare(&chunk[0], &chunk[1]);
        if let Comp::True = v {
            sum += i + 1;
        }
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let mut packets: Vec<Packets> = input
        .split("\n\n")
        .flat_map(|l| l.split("\n"))
        .filter(|s| !s.is_empty())
        .map(|p| p.parse().unwrap())
        .collect();
    let div1: Packets = "[[2]]".parse()?;
    let div2: Packets = "[[6]]".parse()?;
    packets.push(div1.clone());
    packets.push(div2.clone());

    packets.sort_by(|p1, p2| Packets::compare(p1, p2).ord());
    let mut i1 = 0;
    let mut i2 = 0;
    for (i, packet) in packets.iter().enumerate() {
        if let Comp::Neutral = Packets::compare(&div1, packet) {
            i1 = i + 1;
        }
        if let Comp::Neutral = Packets::compare(&div2, packet) {
            i2 = i + 1;
        }
    }
    Ok(i1 * i2)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let right_order = part1(input)?;
    println!("[*] Right order packets: {}", right_order);

    let decoder_key = part2(input)?;
    println!("[*] Decoder key: {}", decoder_key);

    Ok(())
}
