#[aoc_generator(day5)]
pub fn parse(inp: &str) -> Vec<((i32, i32), (i32, i32))> {
  inp.lines().map(|line| {
    let v = line.split(" -> ").map(|coord| {
      let v = coord.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
      (v[0], v[1])
    }).collect::<Vec<(i32, i32)>>();
    (v[0], v[1])
  }).collect()
}

use std::collections::HashMap;
use std::ops::RangeInclusive;

fn ambidextrous_range(a: i32, b: i32) -> (RangeInclusive<i32>, bool) {
  if a < b {
    (a..=b, false)
  } else {
    (b..=a, true)
  }
}

pub fn diagram(inp: Vec<((i32, i32), (i32, i32))>) -> HashMap<(i32, i32), i32> {
  let mut counts = HashMap::new();
  for (a, b) in inp.iter() {
    if a.0 == b.0 || a.1 == b.1 {
      for x in ambidextrous_range(a.0, b.0).0 {
        for y in ambidextrous_range(a.1, b.1).0 {
          *counts.entry((x,y)).or_insert(0) += 1;
        }
      }
    } else {
      let (rx, flippedx) = ambidextrous_range(a.0, b.0);
      let (ry, flippedy) = ambidextrous_range(a.1, b.1);
      let rx: Vec<i32> = {
        if flippedx { rx.rev().collect() } else { rx.collect() }
      };
      let ry: Vec<i32> = {
        if flippedy { ry.rev().collect() } else { ry.collect() }
      };
      for (x, y) in rx.into_iter().zip(ry) {
        *counts.entry((x,y)).or_insert(0) += 1;
      }
    }
  }
  counts
}

#[aoc(day5, part1)]
pub fn part1(inp: &Vec<((i32, i32), (i32, i32))>) -> usize {
  let counts = diagram(inp.clone().into_iter().filter(|(a,b)| a.0 == b.0 || a.1 == b.1).collect());
  counts.values().filter(|x| **x >= 2).count()
}

#[aoc(day5, part2)]
pub fn part2(inp: &Vec<((i32, i32), (i32, i32))>) -> usize {
  let counts = diagram(inp.clone());
  counts.values().filter(|x| **x >= 2).count()
}