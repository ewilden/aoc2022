use std::iter::Iterator as _;

use std::ops::RangeInclusive;

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(",").unwrap();
            fn parse_range(s: &str) -> RangeInclusive<i32> {
                let (lo, hi) = s.split_once("-").unwrap();
                (lo.parse().unwrap())..=(hi.parse().unwrap())
            }
            (parse_range(l), parse_range(r))
        })
        .collect()
}

fn contains(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.start() <= b.start() && a.end() >= b.end()
}

fn overlaps(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.start() >= b.start() && a.start() <= b.end() || a.end() >= b.start() && a.end() <= b.end()
}

#[aoc(day4, part1)]
fn pt1(inp: &Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>) -> usize {
    inp.iter()
        .filter(|(l, r)| contains(l, r) || contains(r, l))
        .count()
}

#[aoc(day4, part2)]
fn pt2(inp: &Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>) -> usize {
    inp.iter()
        .filter(|(l, r)| overlaps(l, r) || overlaps(r, l))
        .count()
}
