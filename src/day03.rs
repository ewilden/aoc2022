use std::collections::HashSet;
use std::iter::Iterator as _;

use itertools::Itertools;

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn priority(common: char) -> i32 {
    i32::from(if common.is_uppercase() {
        (common as i32) - ('A' as i32) + 27
    } else {
        (common as i32) - ('a' as i32) + 1
    })
}

#[aoc(day3, part1)]
fn pt1(inp: &Vec<String>) -> i32 {
    inp.iter()
        .map(|s| {
            let (a, b) = s.split_at(s.len() / 2);
            let a = a.chars().collect::<HashSet<_>>();
            let b = b.chars().collect::<HashSet<_>>();
            let common = *a.intersection(&b).next().unwrap();
            priority(common)
        })
        .sum()
}

#[aoc(day3, part2)]
fn pt2(inp: &Vec<String>) -> i32 {
    inp.iter()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let common = group
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).map(|x| *x).collect())
                .unwrap()
                .into_iter()
                .next()
                .unwrap();
            priority(common)
        })
        .sum()
}
