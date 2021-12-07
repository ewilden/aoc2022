use cached::proc_macro::cached;

#[derive(Copy, Clone)]
struct LanternFish {
  timer: usize,
}

#[aoc_generator(day6)]
fn parse(inp: &str) -> Vec<LanternFish> {
  inp.split(',').map(|c| c.parse::<usize>().unwrap()).map(|timer| LanternFish {timer}).collect()
}

#[cached]
fn num_fish_descended(curr_timer: usize, days_left: usize) -> usize {
  if curr_timer >= days_left {
    1
  } else {
    num_fish_descended(9, days_left - curr_timer) + num_fish_descended(7, days_left - curr_timer)
  }
}

fn step(state: &mut Vec<LanternFish>) {
  let mut new_fish_count = 0;
  for fish in state.iter_mut() {
    if fish.timer == 0 {
      new_fish_count += 1;
      fish.timer = 6;
    } else {
      fish.timer -= 1;
    }
  }
  state.append(&mut ((1..=new_fish_count).map(|_| LanternFish { timer: 8}).collect()))
}

#[aoc(day6, part1)]
fn part1(initial_state: &Vec<LanternFish>) -> usize {
  let mut state = initial_state.clone();
  for _ in 1..=80 {
    step(&mut state);
  }
  state.len()
}

#[aoc(day6, part1, recursive)]
fn part1_recursive_relation(initial_state: &Vec<LanternFish>) -> usize {
  initial_state.iter().map(|LanternFish { timer }| num_fish_descended(*timer, 80)).sum()
}

#[aoc(day6, part2)]
fn part2(initial_state: &Vec<LanternFish>) -> usize {
  initial_state.iter().map(|LanternFish { timer }| num_fish_descended(*timer, 256)).sum()
}