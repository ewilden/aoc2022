use std::collections::VecDeque;
use std::iter::Iterator as _;

use regex::Regex;

use itertools::Itertools;

#[aoc_generator(day5)]
pub fn parse(input: &str) -> (Vec<VecDeque<String>>, Vec<(usize, usize, usize)>) {
    let starting_stacks = {
        let mut first_part = input
            .lines()
            .take_while(|line| *line != "")
            .collect::<Vec<_>>()
            .into_iter()
            .rev();
        let num_cols = first_part.next().unwrap().split_whitespace().count();
        println!("num_cols: {num_cols}");
        let mut stacks: Vec<VecDeque<String>> =
            Vec::from_iter(std::iter::repeat(VecDeque::new()).take(num_cols));
        for line in first_part {
            for (i, _) in line.match_indices("[") {
                stacks[i / 4].push_front(line[(i + 1)..(i + 2)].to_owned())
            }
        }
        stacks
    };
    let instruction_pattern = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    let instructions = {
        input
            .lines()
            .skip_while(|line| *line != "")
            .skip(1)
            .map(|line| {
                let caps = instruction_pattern.captures(line).unwrap();
                (
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                )
            })
            .collect()
    };
    (starting_stacks, instructions)
}

#[aoc(day5, part1)]
fn pt1(
    (starting_stacks, instructions): &(Vec<VecDeque<String>>, Vec<(usize, usize, usize)>),
) -> String {
    let mut stacks = starting_stacks.clone();
    for (count, from, to) in instructions {
        for _ in 1..=*count {
            let to_put = stacks[from - 1].pop_front().unwrap();
            stacks[to - 1].push_front(to_put);
        }
    }
    stacks.into_iter().map(|stack| stack[0].clone()).join("")
}

#[aoc(day5, part2)]
fn pt2(
    (starting_stacks, instructions): &(Vec<VecDeque<String>>, Vec<(usize, usize, usize)>),
) -> String {
    let mut stacks = starting_stacks.clone();
    for (count, from, to) in instructions {
        let to_put = stacks[from - 1].drain(0..*count).collect::<Vec<_>>();
        for item in to_put.into_iter().rev() {
            stacks[to - 1].push_front(item);
        }
    }
    stacks.into_iter().map(|stack| stack[0].clone()).join("")
}
