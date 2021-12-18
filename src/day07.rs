#[aoc_generator(day7)]
fn parse(inp: &str) -> Vec<i32> {
  inp.split(',').map(|c| c.parse::<i32>().unwrap()).collect()
}

#[aoc(day7, part1)]
fn part1(inp: &Vec<i32>) -> i32 {
  let eval = |candidate: i32| { inp.iter().map(|n| (n - candidate).abs()).sum() };
  let lo = inp.iter().min().unwrap();
  let hi = inp.iter().max().unwrap();
  (*lo..=*hi).map(eval).min().unwrap()
}

fn cost(diff: i32) -> i32 {
  /*
  0 -> 0
  1 -> 1
  2 -> 1 + 2
  3 -> 1 + 2 + 3
  4 -> 1 + 2 + 3 + 4
  5 -> 1 + 2 + 3 + 4 + 5
  */
  let n = diff.abs();
  n * (n + 1) / 2
}

#[aoc(day7, part2)]
fn part2(inp: &Vec<i32>) -> i32 {
  let eval = |candidate: i32| { inp.iter().map(|n| {
    let diff = n - candidate;
    cost(diff)
  }).sum() };
  let lo = inp.iter().min().unwrap();
  let hi = inp.iter().max().unwrap();
  (*lo..=*hi).map(eval).min().unwrap()
}