use std::collections::VecDeque;
use itertools::Itertools;

#[aoc_generator(day10)]
fn parse(inp: &str) -> Vec<String> {
  inp.lines().map(|s| s.to_owned()).collect()
}

fn first_illegal_character(line: &String) -> Option<char> {
  build_stack(line).err()
}

fn build_stack(line: &String) -> Result<VecDeque<char>, char> {
  let mut stack = VecDeque::new();
  fn try_consume(stack: &mut VecDeque<char>, c: char) -> bool {
    match stack.pop_front() {
      Some(c_) => c == c_,
      None => false,
    }
  }
  for c in line.chars() {
    match c {
      '(' | '[' | '{' | '<' => stack.push_front(c),
      ')' => if !try_consume(&mut stack, '(') { return Err(c) } else {}
      ']' => if !try_consume(&mut stack, '[') { return Err(c) } else {}
      '}' => if !try_consume(&mut stack, '{') { return Err(c) } else {}
      '>' => if !try_consume(&mut stack, '<') { return Err(c) } else {}
      _ => panic!()
    }
  }
  Ok(stack)
}

#[aoc(day10, part1)]
fn part1(inp: &Vec<String>) -> i64 {
  inp.iter().filter_map(first_illegal_character).map(|c| match c {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
    _ => panic!(),
  }).sum()
}

#[aoc(day10, part2)]
fn part2(inp: &Vec<String>) -> i64 {
  let stack_scores: Vec<i64> = inp.iter().map(build_stack).filter_map(|res| res.ok()).map(|stack| {
    let mut score = 0;
    for c in stack {
      let char_score = match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!()
      };
      score = score * 5 + char_score;
    }
    score
  }).sorted().collect();
  stack_scores[stack_scores.len() / 2]
}