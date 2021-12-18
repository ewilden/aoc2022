use std::collections::HashMap;
use std::collections::VecDeque;
use itertools::Itertools;
type CaveMap<T> = HashMap<(i64, i64), T>;

#[aoc_generator(day11)]
fn parse(inp: &str) -> CaveMap<Option<i64>> {
  let mut hm = HashMap::new();
  for (r, line) in inp.lines().enumerate() {
    for (c, height) in line.chars().enumerate() {
      hm.insert((r as i64, c as i64), Some((height as i64) - ('0' as i64)));
    }
  }
  hm
}

struct Simul {
  map: CaveMap<Option<i64>>,
  num_flashes: i64,
}

fn do_step(simul: &mut Simul) -> bool {
  let mut to_visit: VecDeque<(i64, i64)> = VecDeque::new();
  let Simul {map, num_flashes} = simul;

  // First, energy level of each octopus increases by 1.
  for (loc, level) in map.iter_mut() {
    *level = Some(level.unwrap() + 1);
    if level.unwrap() > 9 {
      to_visit.push_back(*loc);
    }
  }

  loop {
    if to_visit.is_empty() {
      break;
    }
    let loc@(r,c) = to_visit.pop_front().unwrap();
    if map.get(&loc) == Some(&None) {
      continue;
    }
    *num_flashes += 1;
    map.insert(loc, None);
    for loc_@(r_,c_) in ((r-1)..=(r+1)).cartesian_product((c-1)..=(c+1)) {
      if r_ == r && c_ == c {
        continue
      }
      match map.get_mut(&loc_) {
        None => {},
        Some(level) => match level.as_mut() {
          None => {},
          Some(lvl) => {
            *lvl += 1;
            if *lvl > 9 {
              to_visit.push_back(loc_);
            }
          }
        },
      };
    }
  }
  // Now set all the Nones to zero.
  let mut num_none = 0;
  for (_, level) in map.iter_mut() {
    if *level == None {
      *level = Some(0);
      num_none += 1;
    }
  }
  num_none == map.len()
}

#[aoc(day11, part1)]
fn part1(inp: &CaveMap<Option<i64>>) -> i64 {
  let mut simul: Simul = Simul { map: inp.clone(), num_flashes: 0};
  for _ in (1..=100) {
    do_step(&mut simul);
  }
  simul.num_flashes
}

#[aoc(day11, part2)]
fn part2(inp: &CaveMap<Option<i64>>) -> i64 {
  let mut simul: Simul = Simul { map: inp.clone(), num_flashes: 0};
  let mut step = 1;
  loop {
    if do_step(&mut simul) {
      return step;
    }
    step += 1;
  }
}
