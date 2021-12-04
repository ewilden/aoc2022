#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<i32> {
  input.lines()
    .map(|line| line.parse::<i32>().unwrap())
    .collect()
}

#[aoc(day1, part1)]
fn pt1(inp: &Vec<i32>) -> i32 {
  inp.iter().fold((None, 0), |(prev, n), curr| {
    (Some(curr), match prev {
      None => n,
      Some(p) => if curr > p { n + 1 } else { n }
    })
  }).1
}

#[aoc(day1, part2)]
fn pt2(inp: &Vec<i32>) -> i32 {
  use std::collections::VecDeque;
  let mut deq: VecDeque<i32> = VecDeque::new();
  let mut num_increases = 0;
  let mut curr_sum = None;
  for i in inp.iter() {
    deq.push_back(*i);
    if deq.len() == 4 {
      deq.pop_front();
    }
    if deq.len() == 3 {
      let new_sum = deq.iter().sum::<i32>();
      if let Some(curr) = curr_sum{ 
        if new_sum > curr {
          num_increases += 1;
        }
      }
      curr_sum = Some(new_sum);
    }
  }
  num_increases
}