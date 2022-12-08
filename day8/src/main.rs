use anyhow::Result;

struct Grid<'a> {
    grid: &'a [&'a [u8]],
    height: usize,
    width: usize,
}

impl<'a> Grid<'_> {
    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.grid.get(y)?.get(x).copied()
    }

    pub fn is_at_edge(&self, x: usize, y: usize) -> bool {
        x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1
    }

    pub fn is_shorter(&self, val: u8, x: usize, y: usize) -> bool {
        self.get(x, y).unwrap() < val
    }

    pub fn visible(&self, x: usize, y: usize) -> bool {
        if self.is_at_edge(x, y) {
            return true;
        }
        let val = self.get(x, y).unwrap();

        let visible = (y + 1..self.height).all(|y| self.is_shorter(val, x, y))
            || (0..=y.saturating_sub(1))
                .rev()
                .all(|y| self.is_shorter(val, x, y))
            || (x + 1..self.width).all(|x| self.is_shorter(val, x, y))
            || (0..=x.saturating_sub(1))
                .rev()
                .all(|x| self.is_shorter(val, x, y));
        visible
    }

    fn compute_direction_score(
        &self,
        iter: impl Iterator<Item = usize>,
        val: u8,
        v1: usize,
        left: bool,
    ) -> usize {
        let mut score = 0;
        for v2 in iter {
            let valid = if left {
                self.is_shorter(val, v1, v2)
            } else {
                self.is_shorter(val, v2, v1)
            };
            score += 1;
            if !valid {
                break;
            }
        }
        score
    }

    pub fn scenic_score(&self, x: usize, y: usize) -> usize {
        if self.is_at_edge(x, y) {
            return 0;
        }
        let val = self.get(x, y).unwrap();
        let score = self.compute_direction_score(y + 1..self.height, val, x, true)
            * self.compute_direction_score((0..=y.saturating_sub(1)).rev(), val, x, true)
            * self.compute_direction_score(x + 1..self.width, val, y, false)
            * self.compute_direction_score((0..=x.saturating_sub(1)).rev(), val, y, false);
        score
    }
}

fn part1(input: &str) -> Result<usize> {
    let v: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let height = v.len();
    let width = v.first().unwrap().len();
    let grid = Grid {
        grid: &v,
        height,
        width,
    };
    let mut c = 0;
    for y in 0..height {
        for x in 0..width {
            if grid.visible(x, y) {
                c += 1;
            }
        }
    }
    Ok(c)
}

fn part2(input: &str) -> Result<usize> {
    let v: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let height = v.len();
    let width = v.first().unwrap().len();
    let grid = Grid {
        grid: &v,
        height,
        width,
    };
    let mut max_score = 0;
    for y in 0..height {
        for x in 0..width {
            let score = grid.scenic_score(x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }
    Ok(max_score)
}

fn main() -> Result<()> {
    let input = include_str!("../input");

    let visible_count = part1(input)?;
    let scenic_score = part2(input)?;

    println!("[*] Visible trees: {}", visible_count);
    println!("[*] Best scenic score: {}", scenic_score);

    Ok(())
}
