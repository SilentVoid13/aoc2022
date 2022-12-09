use std::str::FromStr;
use std::collections::HashSet;

use anyhow::{anyhow, Context, Error, Result};

#[derive(Debug)]
struct Rope {
    knots: Vec<Pos>,
    visited_tail_pos: HashSet<Pos>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Pos {
    x: isize,
    y: isize
}

struct Move {
    direction: Direction,
    n: usize,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (m, n) = s.split_once(" ").context("invalid move")?;
        let n = n.parse()?;
        let direction = match m {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return Err(anyhow!("unknown move")),
        };
        Ok(Move {
            direction,
            n
        })
    }
}

pub fn euclidean_distance(p: &Pos, q: &Pos) -> f64 {
    let r = (q.x - p.x).pow(2) + (q.y - p.y).pow(2);
    f64::sqrt(r as f64)
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Pos { x, y }
    }
}

impl Rope {
    pub fn new(n: usize) -> Self {
        let mut knots = vec![];
        for _ in 0..n {
            knots.push(Pos::new(0, 0))
        }
        Rope { knots, visited_tail_pos: HashSet::new() }
    }

    pub fn mov(&mut self, moves: &[Move]) -> Result<()> {
        for mov in moves.iter() {
            for _ in 0..mov.n {
                let mut lead_knot = self.knots.first_mut().unwrap();
                match mov.direction {
                    Direction::Right => lead_knot.x += 1,
                    Direction::Left => lead_knot.x -= 1,
                    Direction::Up => lead_knot.y -= 1,
                    Direction::Down => lead_knot.y += 1,
                };

                let mut i = 0;
                for j in 1..self.knots.len() {
                    let lead_knot = &self.knots[i];
                    let knot = &self.knots[j];

                    let dist = euclidean_distance(&lead_knot, &knot);
                    if dist > 1.5 {
                        let mut best_dist = 1e6;
                        let mut best_pos = None;
                        for y in -1..=1 {
                            for x in -1..=1 {
                                let new_pos = Pos {
                                    x: knot.x + x,
                                    y: knot.y + y,
                                };
                                let new_dist = euclidean_distance(&lead_knot, &new_pos);
                                if new_dist < best_dist {
                                    best_pos = Some(new_pos);
                                    best_dist = new_dist;
                                }
                            }
                        }
                        let knot = self.knots.get_mut(j).unwrap();
                        *knot = best_pos.unwrap();
                    }
                    i += 1;
                }
                self.visited_tail_pos.insert(self.knots.last().unwrap().clone());
            }
        }
        Ok(())
    }
}

fn part1(input: &str) -> Result<usize> {
    let moves: Vec<Move> = input.lines().flat_map(|l| l.parse()).collect();
    let mut rope = Rope::new(2);
    rope.mov(&moves)?;
    Ok(rope.visited_tail_pos.len())
}

fn part2(input: &str) -> Result<usize> {
    let moves: Vec<Move> = input.lines().flat_map(|l| l.parse()).collect();
    let mut rope = Rope::new(10);
    rope.mov(&moves)?;
    Ok(rope.visited_tail_pos.len())
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let visited_pos1 = part1(input)?;
    let visited_pos2 = part2(input)?;

    println!("[*] Visited tail positions (1): {}", visited_pos1);
    println!("[*] Visited tail positions (2): {}", visited_pos2);

    Ok(())
}
