use bitvec::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools as _;

type bitindex = usize;

fn digit_to_bitmask(digit: i32) -> BitVec {
    match digit {
        0 => bitvec![1, 1, 1, 0, 1, 1, 1],
        1 => bitvec![0, 0, 1, 0, 0, 1, 0],
        2 => bitvec![1, 0, 1, 1, 1, 0, 1],
        3 => bitvec![1, 0, 1, 1, 0, 1, 1],
        4 => bitvec![0, 1, 1, 1, 0, 1, 0],
        5 => bitvec![1, 1, 0, 1, 0, 1, 1],
        6 => bitvec![1, 1, 0, 1, 1, 1, 1],
        7 => bitvec![1, 0, 1, 0, 0, 1, 0],
        8 => bitvec![1, 1, 1, 1, 1, 1, 1],
        9 => bitvec![1, 1, 1, 1, 0, 1, 1],
        _ => panic!(),
    }
}

fn zeroes() -> BitVec { bitvec![0, 0, 0, 0, 0, 0, 0] }
fn ones() -> BitVec { bitvec![1, 1, 1, 1, 1, 1, 1] }
fn singleton(ind: usize) -> BitVec {
    let mut z = zeroes();
    z.set(ind, true);
    z
}

fn bitmask_to_digit(bm: &BitVec) -> Option<i32> {
    for d in 0..=9 {
        if digit_to_bitmask(d) == *bm {
            return Some(d as i32)
        }
        if digit_to_bitmask(d).len() != bm.len() {
            panic!()
        }
    }
    None
}

fn num_segments(digit: i32) -> usize {
    digit_to_bitmask(digit)
        .as_bitslice()
        .count_ones()
        .try_into()
        .unwrap()
}

fn unique_segment_count(seg_count: usize) -> Option<i32> {
    match seg_count {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        0 | 1 | 5 | 6 => None,
        _ => panic!(),
    }
}

#[derive(Clone)]
struct InputLine {
    signals: Vec<Vec<char>>,
    output: (Vec<char>, Vec<char>, Vec<char>, Vec<char>),
}

impl InputLine {
    fn all_entries(&self) -> Vec<Vec<char>> {
        self.signals.clone()
            .into_iter()
            .chain(from_quad(self.output.clone()))
            .collect()
    }
}

fn from_quad<T>(quad: (Vec<T>, Vec<T>, Vec<T>, Vec<T>)) -> Vec<Vec<T>> {
    let (a, b, c, d) = quad;
    vec![a, b, c, d]
}

#[aoc_generator(day8)]
fn parse(inp: &str) -> Vec<InputLine> {
    inp.lines()
        .map(|line| {
            let splitted = line.split(" | ").collect::<Vec<_>>();
            let raw_fst = splitted[0];
            let raw_snd = splitted[1];
            let v: Vec<Vec<char>> = raw_snd.split(" ").map(|s| s.chars().collect()).collect();
            InputLine {
                signals: raw_fst.split(" ").map(|s| s.chars().collect()).collect(),
                output: (v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(inp: &Vec<InputLine>) -> usize {
    inp.iter()
        .map(|input| {
            let uniq_digit_count: usize = from_quad(input.output.clone())
                .iter()
                .map(|v| v.len())
                .map(|n| {
                    let may_uniq_digit: Option<i32> = unique_segment_count(n);
                    may_uniq_digit.iter().len()
                })
                .sum();
            uniq_digit_count
        })
        .sum()
}

use single::Single;

fn solve_line(InputLine { signals, output }: InputLine) -> i32 {
    let dig1 = signals.iter().filter(|cs| cs.len() == 2).single().unwrap();
    let dig4 = signals.iter().filter(|cs| cs.len() == 4).single().unwrap();
    let dig7 = signals.iter().filter(|cs| cs.len() == 3).single().unwrap();
    let dig8 = signals.iter().filter(|cs| cs.len() == 7).single().unwrap();

    let digs0and9 = signals.iter().filter(|cs| cs.len() == 6 && dig1.iter().all(|c| cs.contains(c)));
    let dig9 = digs0and9.clone().filter(|cs| dig4.iter().all(|c| cs.contains(c))).single().unwrap();
    let dig0 = digs0and9.filter(|cs| cs != &dig9).single().unwrap();

    let bit3 = dig8.iter().filter(|c| !dig0.contains(c)).single().unwrap();
    let bit4 = dig8.iter().filter(|c| !dig9.contains(c)).single().unwrap();

    let dig2 = signals.iter().filter(|cs| cs.len() == 5 && cs.contains(bit3) && cs.contains(bit4)).single().unwrap();
    fn symm_diff(a: &Vec<char>, b: &Vec<char>) -> usize {
        let mut diff = 0;
        for c in a {
            if !b.contains(c) {
                diff += 1;
            }
        }
        for c in b {
            if !a.contains(c) {
                diff += 1;
            }
        }
        diff
    }
    let dig3 = signals.iter().filter(|cs| cs.len() == 5 && symm_diff(cs, dig2) == 2).single().unwrap();
    let dig5 = signals.iter().filter(|cs| cs.len() == 5 && symm_diff(cs, dig2) == 4).single().unwrap();

    let dig6 = signals.iter().filter(|cs| {
        cs != &dig0 
            && cs != &dig1
            && cs != &dig2
            && cs != &dig3
            && cs != &dig4
            && cs != &dig5
            && cs != &dig7
            && cs != &dig8
            && cs != &dig9
    }).single().unwrap();

    fn sortt<'a>(chars: &'a Vec<char>) -> Vec<char> {
        chars.clone().into_iter().sorted().collect()
    }
    
    let translate_digit = |chars| {
        if sortt(chars) == sortt(dig0) { 0 } else
        if sortt(chars) == sortt(dig1) { 1 } else
        if sortt(chars) == sortt(dig2) { 2 } else
        if sortt(chars) == sortt(dig3) { 3 } else
        if sortt(chars) == sortt(dig4) { 4 } else
        if sortt(chars) == sortt(dig5) { 5 } else
        if sortt(chars) == sortt(dig6) { 6 } else
        if sortt(chars) == sortt(dig7) { 7 } else
        if sortt(chars) == sortt(dig8) { 8 } else
        if sortt(chars) == sortt(dig9) { 9 } else { panic!("{:?} {:?}", chars, [dig0, dig1, dig2, dig3, dig4, dig5, dig6, dig7, dig8, dig9]) }
    };

    1000 * translate_digit(&output.0)
    + 100 * translate_digit(&output.1)
    + 10 * translate_digit(&output.2)
    + 1 * translate_digit(&output.3)
}

#[aoc(day8, part2)]
fn part2(inp: &Vec<InputLine>) -> i32 {
    inp.iter().cloned().map(solve_line).sum()
}

//  0000
// 1    2
// 1    2
//  3333
// 4    5
// 4    5
//  6666

// simpler rules:
// 2 -> 3 segments ==> new one is 0th bit (1 -> 7)

// we know which two segments are in 1. then 0 and 9 are the only 6-segment digits with both of those segments.

// we also know which four segments are in 4. 9 contains those four, and 0 is the other one. 

// now that we know which group is 9, and which group is 0, we can determine the middle, 3rd bit (8 - 0)
// and the bottom left, 4th bit (8 - 9). 

// there are three digits with five segments (2, 3, 5). 2 is the only one with the 3rd and 4th bit.
// then since 2 xor 3 has 2 set bits and 2 xor 5 has 4 set bits, we know those groupigns as well.

// === summary so far ===
// We have these bits: 0th, 3rd, 4th
// We identified these groupings: 
// - 1, 4, 7, 8 (unique number of segments)
// - 0, 9
// - 2, 3, 5

// The only remaining segment is 6!

