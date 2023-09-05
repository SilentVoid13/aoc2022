use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque, BTreeSet},
    rc::Rc,
    str::FromStr,
    time::Instant,
};

use anyhow::{Context, Error, Result};

use itertools::Itertools;

#[derive(Debug)]
pub struct Tunnels {
    pub valves: HashMap<String, Rc<RefCell<Valve>>>,
}

pub struct Valve {
    pub name: String,
    pub flow: usize,
    pub neighbours: Vec<Rc<RefCell<Valve>>>,
    pub path_costs: HashMap<String, usize>,
}

impl std::fmt::Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let neighbours_names = self
            .neighbours
            .iter()
            .map(|n| n.borrow().name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        f.write_fmt(format_args!("Valve {}: ({})", self.name, neighbours_names))
    }
}

impl Valve {
    // BFS shortest path between 2 valves
    pub fn shortest_path_cost(from: Rc<RefCell<Valve>>, to: Rc<RefCell<Valve>>) -> usize {
        let to = to.borrow();
        let mut queue = VecDeque::new();
        queue.push_front((0, from));

        while let Some((cost, valve)) = queue.pop_front() {
            if valve.borrow().name == to.name {
                // +1 for opening the valve
                return cost + 1;
            }
            for neighbour in valve.borrow().neighbours.iter() {
                let new_path = (cost + 1, neighbour.clone());
                queue.push_back(new_path);
            }
        }
        0
    }
}

impl FromStr for Tunnels {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut tunnels = Tunnels {
            valves: HashMap::new(),
        };

        for line in s.lines().filter(|l| !l.trim().is_empty()) {
            let name = line.split(" ").nth(1).context("no valve name")?;
            let (_, flow) = line.split_once("=").context("no flow rate")?;
            let flow = flow
                .split_once(";")
                .context("no flow rate")?
                .0
                .parse::<usize>()
                .context("invalid flow rate")?;
            let neighbours_names: Vec<String> =
                line.split_once("to valve").context("no neighbours")?.1[1..]
                    .split(",")
                    .map(|n| n.trim().to_string())
                    .collect();

            if let Some(v) = tunnels.valves.get(name) {
                v.borrow_mut().flow = flow;
            } else {
                let v = Valve {
                    name: name.to_string(),
                    flow,
                    neighbours: vec![],
                    path_costs: HashMap::new(),
                };
                tunnels
                    .valves
                    .insert(name.to_string(), Rc::new(RefCell::new(v)));
            }

            for neighbour_name in neighbours_names.iter() {
                if !tunnels.valves.contains_key(neighbour_name) {
                    let v = Valve {
                        name: neighbour_name.to_string(),
                        flow: 0,
                        neighbours: vec![],
                        path_costs: HashMap::new(),
                    };
                    tunnels
                        .valves
                        .insert(neighbour_name.to_string(), Rc::new(RefCell::new(v)));
                }
                let valve = tunnels.valves.get(name).context("invalid valve name")?;
                let neighbour = tunnels.valves.get(neighbour_name).unwrap();
                valve.borrow_mut().neighbours.push(neighbour.clone());
            }
        }

        for v1 in tunnels.valves.values() {
            for v2 in tunnels.valves.values() {
                if v1.borrow().name == v2.borrow().name {
                    continue;
                }
                let path_cost = Valve::shortest_path_cost(v1.clone(), v2.clone());
                v1.borrow_mut()
                    .path_costs
                    .insert(v2.borrow().name.clone(), path_cost);
            }
        }

        Ok(tunnels)
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub turn: usize,
    pub max_turn: usize,
    pub pressure: usize,
    pub position: Rc<RefCell<Valve>>,
    pub opened_valves: BTreeSet<String>,
}

impl State {
    pub fn new(
        turn: usize,
        max_turn: usize,
        pressure: usize,
        position: Rc<RefCell<Valve>>,
        opened_valves: BTreeSet<String>,
    ) -> State {
        State {
            turn,
            max_turn,
            pressure,
            position,
            opened_valves,
        }
    }

    pub fn apply(&self, mv: &Move) -> Self {
        let mut next_state = self.clone();
        next_state.opened_valves.insert(mv.pos.borrow().name.clone());
        next_state.pressure += mv.reward;
        next_state.position = mv.pos.clone();
        next_state.turn = self.turn + mv.path_cost;
        next_state
    }

    pub fn possible_moves(&self, tunnels: &Tunnels) -> Vec<Move> {
        let mut moves = vec![];
        for v in tunnels.valves.values() {
            let valve = v.borrow();
            if valve.name == self.position.borrow().name {
                continue;
            }
            if self.opened_valves.contains(&valve.name) {
                continue;
            }
            // It is useless to open a 0-flow valve
            if valve.flow == 0 {
                continue;
            }

            let path_cost = self.position.borrow().path_costs[&valve.name];
            // Ignore invalid paths
            if path_cost + self.turn > self.max_turn {
                continue;
            }

            let reward = valve.flow * (self.max_turn - (self.turn + path_cost));

            let mv = Move {
                path_cost,
                reward,
                pos: v.clone(),
            };
            moves.push(mv);
        }
        moves
    }

    pub fn find_best_moves(&self, tunnels: &Tunnels) -> (Self, Vec<Move>) {
        let mut best_moves = vec![];
        let mut best_state = self.clone();
        let mut best_pressure = 0;

        let mut moves = self.possible_moves(tunnels);
        // Consider best moves first
        moves.sort_by_key(|m| m.reward);
        moves.reverse();

        for mv in moves {
            let next = self.apply(&mv);
            let (next, mut next_moves) = next.find_best_moves(tunnels);
            next_moves.push(mv);
            if next.pressure > best_pressure {
                best_pressure = next.pressure;
                best_moves = next_moves;
                best_state = next;
            }
        }
        (best_state, best_moves)
    }

    pub fn find_best_moves2(&self, tunnels: &Tunnels, best: &mut Best) -> Self {
        let mut best_state = self.clone();

        best.entry(self.opened_valves.clone())
            .and_modify(|v| {
                if self.pressure as u64 > *v {
                    *v = self.pressure as u64
                }
            })
            .or_insert(self.pressure.try_into().unwrap());

        for mv in self.possible_moves(tunnels) {
            let next = self.apply(&mv).find_best_moves2(tunnels, best);
            if next.pressure > best_state.pressure {
                best_state = next;
            }
        }
        best_state
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    pub pos: Rc<RefCell<Valve>>,
    pub path_cost: usize,
    pub reward: usize,
}

type Best = HashMap<BTreeSet<String>, u64>;

fn part1(input: &str) -> Result<usize> {
    let tunnels: Tunnels = input.parse()?;
    let max_turn = 30;
    let start_pos = &tunnels.valves["AA"];
    let start_state = State::new(0, max_turn, 0, start_pos.clone(), BTreeSet::new());
    let best = start_state.find_best_moves(&tunnels);
    Ok(best.0.pressure)
}

fn part2(input: &str) -> Result<usize> {
    // For part 2, we first run as if we were alone and we save the best combinations of opened valves
    // We then simply pick the two best disjoint sets of opened valves

    let tunnels: Tunnels = input.parse()?;
    let max_turn = 26;
    let start_pos = &tunnels.valves["AA"];
    let start_state = State::new(0, max_turn, 0, start_pos.clone(), BTreeSet::new());
    let mut best = Best::default();
    start_state.find_best_moves2(&tunnels, &mut best);
    let best_pressure = best
        .iter()
        .tuple_combinations()
        .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
        .map(|(human, elephant)| human.1 + elephant.1)
        .max()
        .unwrap();
    Ok(best_pressure as usize)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let instant = Instant::now();
    let max_pressure = part1(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 1: {} ({:?})", max_pressure, time);

    let instant = Instant::now();
    let max_pressure = part2(input)?;
    let time = Instant::now() - instant;
    println!("[*] part 2: {} ({:?})", max_pressure, time);

    Ok(())
}
