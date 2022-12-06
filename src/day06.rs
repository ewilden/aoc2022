use std::collections::HashSet;

use itertools::Itertools;

#[aoc_generator(day6)]
pub fn parse(input: &str) -> String {
    input.to_owned()
}

#[aoc(day6, part1)]
fn pt1(input: &str) -> usize {
    input.chars().collect::<Vec<_>>()[..]
        .windows(4)
        .find_position(|chars| chars.iter().collect::<HashSet<_>>().len() == 4)
        .unwrap()
        .0
        + 4
}

#[aoc(day6, part2)]
fn pt2(input: &str) -> usize {
    input.chars().collect::<Vec<_>>()[..]
        .windows(14)
        .find_position(|chars| chars.iter().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        .0
        + 14
}
