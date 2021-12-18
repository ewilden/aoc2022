use std::collections::HashMap;
use std::collections::VecDeque;
use itertools::Itertools;
type CaveMap<T> = HashMap<(i32, i32), T>;

#[aoc_generator(day9)]
fn parse(inp: &str) -> CaveMap<i32> {
  let mut hm = HashMap::new();
  for (r, line) in inp.lines().enumerate() {
    for (c, height) in line.chars().enumerate() {
      hm.insert((r as i32, c as i32), (height as i32) - ('0' as i32));
    }
  }
  hm
}

#[aoc(day9, part1)]
fn part1(inp: &CaveMap<i32>) -> i32 {
  inp.iter().filter_map(|((r,c),height)| {
    for r_ in (r-1)..=(r+1) {
      for c_ in (c-1)..=(c+1) {
        if r_ == *r && c_ == *c {
          continue
        }
        if !(r_ == *r || c_ == *c) {
          continue
        }
        if let Some(height_) = inp.get(&(r_,c_)) {
          if height_ <= height {
            return None
          }
        }
      }
    }
    Some(1 + height)
  }).sum()
}

fn fill_all_basins(basins: &mut CaveMap<(i32, i32)>, heights: &CaveMap<i32>, basin_sizes: &mut CaveMap<usize>) {
  let mut locs_todo: VecDeque<(i32, i32)> = heights.keys().cloned().collect();
  loop {
    if locs_todo.is_empty() {
      return
    }
    let loc = locs_todo.pop_front().unwrap();

    if heights[&loc] == 9 {
      continue
    }

    if let Some(basin) = basins.get(&loc) {
      continue
    }

    let (r, c) = loc;
  
    let (lowest_neighbor, lowest_neighbor_height) = {
      ((r - 1)..=(r + 1)).cartesian_product((c - 1)..=(c + 1))
        .filter(|(r_, c_)| {
          if r_ == &r && c_ == &c {
            return false
          }
          if !(r_ == &r || c_ == &c) {
            return false
          }
          true
        })
        .filter_map(|loc_| heights.get(&loc_).map(|height| (loc_, height)))
        .min_by_key(|(l, height)| (*height, *l))
        .unwrap()
    };

    if *lowest_neighbor_height > heights[&loc] {
      basins.insert(loc, loc);
      basin_sizes.insert(loc, 1);
      continue
    }

    if let Some(lowest_neighbor_basin) = basins.get(&lowest_neighbor).copied() {
      basins.insert(loc, lowest_neighbor_basin);
      basin_sizes.entry(lowest_neighbor_basin).and_modify(|e| {*e += 1});
      continue
    } else {
      // need to return to this after we've handled neighbors
      locs_todo.push_back(loc);
      // better to do lowest neighbor next? optimization?
      locs_todo.push_front(lowest_neighbor);
    }
  }
}

fn fill_basin(basins: &mut CaveMap<(i32, i32)>, heights: &CaveMap<i32>, loc: &(i32, i32)) -> Option<(i32, i32)> {

  if heights[loc] == 9 {
    return None
  }

  if let Some(basin) = basins.get(loc) {
    return Some(*basin)
  }

  let (r, c) = loc;

  let (lowest_neighbor, lowest_neighbor_height) = {
    ((r - 1)..=(r + 1)).cartesian_product((c - 1)..=(c + 1))
      .filter(|(r_, c_)| {
        if r_ == r && c_ == c {
          return false
        }
        if !(r_ == r || c_ == c) {
          return false
        }
        true
      })
      .map(|loc_| (loc_, heights[&loc_]))
      .min_by_key(|(l, height)| (*height, *l))
      .unwrap()
  };

  let nbr_basin = fill_basin(basins, heights, &lowest_neighbor).unwrap();

  basins.insert(*loc, nbr_basin);
  
  Some(nbr_basin)
}

#[aoc(day9, part2)]
fn part2(heights: &CaveMap<i32>) -> usize {
  let mut basins = HashMap::new();
  let mut basin_sizes = HashMap::new();
  fill_all_basins(&mut basins, heights, &mut basin_sizes);
  basin_sizes.values().sorted_by_key(|s| -(**s as i32)).take(3).product()
}

// #[aoc(day9, part1, notreal)]
// fn part1test(_: &HeightMap) -> i32 {
//   part1(2199943210
//     3987894921
//     9856789892
//     8767896789
//     9899965678)
// }