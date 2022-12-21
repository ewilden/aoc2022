use std::{
    collections::{BTreeSet, HashMap},
    rc::Rc,
};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Equ,
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Vec<(String, Result<Operand, (String, Op, String)>)> {
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            let (root, exp) = line.split_once(": ").unwrap();
            (
                root.to_owned(),
                (exp.parse::<i64>().map(Operand::Num).map_err(|_| {
                    let (a, rest) = exp.split_once(" ").unwrap();
                    let op = rest.chars().next().unwrap();
                    let (_, b) = rest.split_once(" ").unwrap();
                    (
                        a.to_owned(),
                        {
                            match op {
                                '+' => Op::Add,
                                '-' => Op::Sub,
                                '*' => Op::Mul,
                                '/' => Op::Div,
                                _ => panic!("{}", op),
                            }
                        },
                        b.to_owned(),
                    )
                })),
            )
        })
        .collect()
}

#[derive(Clone)]
pub enum Operand {
    Num(i64),
    Humn(Rc<dyn Fn(i64) -> i64>),
}

impl Operand {
    fn num(&self) -> Option<i64> {
        match self {
            Self::Num(n) => Some(*n),
            _ => None,
        }
    }
}

#[aoc(day21, part1)]
fn part1(input: &[(String, Result<Operand, (String, Op, String)>)]) -> i64 {
    let _input = &parse(
        "root: pppw + sjmn
  dbpl: 5
  cczh: sllz + lgvd
  zczc: 2
  ptdq: humn - dvpt
  dvpt: 3
  lfqf: 4
  humn: 5
  ljgn: 2
  sjmn: drzm * dbpl
  sllz: 4
  pppw: cczh / lfqf
  lgvd: ljgn * ptdq
  drzm: hmdt - zczc
  hmdt: 32",
    );
    part1_impl(input)
}

fn part1_impl(input: &[(String, Result<Operand, (String, Op, String)>)]) -> i64 {
    let mut map = HashMap::new();
    let mut rev_refs = HashMap::new();
    let mut resolved = HashMap::new();
    for (monkey, job) in input {
        map.insert(monkey.clone(), job.clone());
        match job {
            Ok(v) => {
                resolved.insert(monkey.clone(), v.clone());
            }
            Err((a, _op, b)) => {
                rev_refs
                    .entry(a.clone())
                    .or_insert(BTreeSet::new())
                    .insert(monkey);
                rev_refs
                    .entry(b.clone())
                    .or_insert(BTreeSet::new())
                    .insert(monkey);
            }
        }
    }
    let mut newly_resolved = resolved.keys().cloned().collect_vec();
    while !newly_resolved.is_empty() {
        for monkey in std::mem::replace(&mut newly_resolved, Vec::new()) {
            for &rev_ref in rev_refs.get(&monkey).unwrap_or(&BTreeSet::new()) {
                if resolved.contains_key(rev_ref) {
                    continue;
                }
                let (a, op, b) = &map[rev_ref].as_ref().err().unwrap();
                if !resolved.contains_key(a) || !resolved.contains_key(b) {
                    continue;
                }
                let a = resolved[a].num().unwrap();
                let b = resolved[b].num().unwrap();
                let result = match *op {
                    Op::Add => a + b,
                    Op::Sub => a - b,
                    Op::Mul => a * b,
                    Op::Div => a / b,
                    Op::Equ => unreachable!(),
                };
                resolved.insert(rev_ref.to_string(), Operand::Num(result));
                newly_resolved.push(rev_ref.to_string());
            }
        }
    }
    resolved["root"].num().unwrap()
}

#[aoc(day21, part2)]
fn part2(input: &[(String, Result<Operand, (String, Op, String)>)]) -> i64 {
    let mut input = input.to_vec();
    for (monkey, job) in input.iter_mut() {
        if monkey == "root" {
            job.as_mut().err().unwrap().1 = Op::Equ;
        }
    }
    let input = &input;
    let mut map = HashMap::new();
    let mut rev_refs = HashMap::new();
    let mut resolved = HashMap::new();
    for (monkey, job) in input {
        map.insert(monkey.clone(), job.clone());
        match job {
            Ok(v) => {
                if monkey != "humn" {
                    resolved.insert(monkey.clone(), v.clone());
                } else {
                    resolved.insert(monkey.clone(), Operand::Humn(Rc::new(|n| n)));
                }
            }
            Err((a, _op, b)) => {
                rev_refs
                    .entry(a.clone())
                    .or_insert(BTreeSet::new())
                    .insert(monkey);
                rev_refs
                    .entry(b.clone())
                    .or_insert(BTreeSet::new())
                    .insert(monkey);
            }
        }
    }
    let mut newly_resolved = resolved.keys().cloned().collect_vec();
    while !newly_resolved.is_empty() {
        for monkey in std::mem::replace(&mut newly_resolved, Vec::new()) {
            for &rev_ref in rev_refs.get(&monkey).unwrap_or(&BTreeSet::new()) {
                if resolved.contains_key(rev_ref) {
                    continue;
                }
                let (a, op, b) = &map[rev_ref].as_ref().err().unwrap();
                if !resolved.contains_key(a) || !resolved.contains_key(b) {
                    continue;
                }
                let a = resolved[a].clone();
                let b = resolved[b].clone();
                newly_resolved.push(rev_ref.to_string());
                if let (Some(a), Some(b)) = (a.num(), b.num()) {
                    let result = match *op {
                        Op::Add => Some(a + b),
                        Op::Sub => Some(a - b),
                        Op::Mul => Some(a * b),
                        Op::Div => Some(a / b),
                        Op::Equ => unreachable!(),
                    };
                    if let Some(result) = result {
                        resolved.insert(rev_ref.to_string(), Operand::Num(result));
                    }
                }
                if let Operand::Humn(f) = a.clone() {
                    let f = f.clone();
                    let b = b.clone();
                    resolved.insert(
                        rev_ref.to_string(),
                        Operand::Humn(match *op {
                            Op::Add => Rc::new(move |n| f(n - b.num().unwrap())),
                            Op::Sub => Rc::new(move |n| f(n + b.num().unwrap())),
                            Op::Mul => Rc::new(move |n| f(n / b.num().unwrap())),
                            Op::Div => Rc::new(move |n| f(n * b.num().unwrap())),
                            Op::Equ => {
                                println!("hit equ");
                                continue;
                            }
                        }),
                    );
                }
                if let Operand::Humn(f) = b {
                    let f = f.clone();
                    let a = a.clone();
                    resolved.insert(
                        rev_ref.to_string(),
                        Operand::Humn(match *op {
                            Op::Add => Rc::new(move |n| f(n - a.num().unwrap())),
                            Op::Sub => Rc::new(move |n| f(-n + a.num().unwrap())),
                            Op::Mul => Rc::new(move |n| f(n / a.num().unwrap())),
                            Op::Div => Rc::new(move |n| f(a.num().unwrap() / n)),
                            Op::Equ => {
                                println!("hit equ");
                                continue;
                            }
                        }),
                    );
                }
            }
        }
    }
    let (a, _, b) = map["root"].as_ref().err().unwrap();
    let (f, x) = match (&resolved[a], &resolved[b]) {
        (Operand::Num(_), Operand::Num(_)) => panic!(),
        (Operand::Num(x), Operand::Humn(f)) | (Operand::Humn(f), Operand::Num(x)) => {
            (f.clone(), *x)
        }
        (Operand::Humn(_), Operand::Humn(_)) => panic!(),
    };
    f(x)
}
