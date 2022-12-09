use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

#[derive(Clone, Copy)]
pub enum Dir {
    U,
    R,
    D,
    L,
}

impl Dir {
    fn move_dir(&self, (x, y): &mut (i32, i32)) {
        use Dir::*;
        match self {
            U => *y += 1,
            R => *x += 1,
            D => *y -= 1,
            L => *x -= 1,
        }
    }
}

impl FromStr for Dir {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Dir::*;
        match s {
            "R" => Ok(R),
            "U" => Ok(U),
            "D" => Ok(D),
            "L" => Ok(L),
            _ => Err(format!("couldn't parse dir from {s}")),
        }
    }
}

#[aoc_generator(day9)]
pub fn parse(input: &str) -> Vec<(Dir, usize)> {
    input
        .lines()
        .map(|line| {
            let (dir, num) = line.split_once(" ").unwrap();
            let dir: Dir = dir.parse().unwrap();
            let num = num.parse().unwrap();
            (dir, num)
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<(Dir, usize)>) -> usize {
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    let mut h: (i32, i32) = (0, 0);
    let mut t: (i32, i32) = (0, 0);
    positions.insert(t);
    for (dir, dist) in input {
        for _ in 0..*dist {
            dir.move_dir(&mut h);
            if (h.0 - t.0).abs() > 1 {
                dir.move_dir(&mut t);
                t.1 = h.1;
            }
            if (h.1 - t.1).abs() > 1 {
                dir.move_dir(&mut t);
                t.0 = h.0;
            }
            positions.insert(t);
        }
    }
    positions.len()
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<(Dir, usize)>) -> usize {
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    let mut knots: Vec<(i32, i32)> = Vec::from_iter(std::iter::repeat((0, 0)).take(10));
    positions.insert(knots[9]);
    for (dir, dist) in input {
        for _ in 0..*dist {
            dir.move_dir(&mut knots[0]);
            for i in 0..(knots.len() - 1) {
                let h = knots[i];
                let mut t = knots[i + 1];
                if (h.0 - t.0).abs() > 1 {
                    (if h.0 > t.0 { Dir::R } else { Dir::L }).move_dir(&mut t);
                    if h.1 != t.1 {
                        t.1 += (h.1 - t.1) / (h.1 - t.1).abs();
                    }
                }
                if (h.1 - t.1).abs() > 1 {
                    (if h.1 > t.1 { Dir::U } else { Dir::D }).move_dir(&mut t);
                    if h.0 != t.0 {
                        t.0 += (h.0 - t.0) / (h.0 - t.0).abs();
                    }
                }
                knots[i] = h;
                knots[i + 1] = t;
            }

            positions.insert(knots[9]);
        }
    }
    positions.len()
}
