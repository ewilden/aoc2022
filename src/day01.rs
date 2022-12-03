use itertools::Itertools as _;
use std::iter::Iterator as _;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .group_by(|line| *line == "")
        .into_iter()
        .filter_map(|(k, group)| {
            (!k).then(|| group.map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>())
        })
        .collect()
}

#[aoc(day1, part1)]
fn pt1(inp: &Vec<Vec<i32>>) -> i32 {
    inp.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
fn pt2(inp: &Vec<Vec<i32>>) -> i32 {
    inp.iter()
        .map(|elf| elf.iter().sum::<i32>())
        .sorted()
        .rev()
        .take(3)
        .sum::<i32>()
}
