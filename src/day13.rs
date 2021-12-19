
#[derive(Clone, Copy)]
enum Dim {
  X,
  Y
}

struct Input {
  coords: Vec<(i32, i32)>,
  folds: Vec<(Dim, i32)>,
}

use std::collections::HashSet;
type Grid = HashSet<(i32, i32)>;

#[aoc_generator(day13)]
fn parse(inp: &str) -> Input {
  let mut splitted = inp.split("\n\n");
  let (raw_coords, raw_folds) = (splitted.next().unwrap(), splitted.next().unwrap());
  let coords = raw_coords.lines().map(|line| {
    let mut splitted = line.split(',');
    (splitted.next().unwrap().parse().unwrap(), splitted.next().unwrap().parse().unwrap())
  }).collect();
  let folds = raw_folds.lines().map(|line| {
    let mut without_junk = line["fold along ".len()..].split('=');
    let raw_dir = without_junk.next().unwrap();
    let raw_num = without_junk.next().unwrap();
    let dir = match raw_dir {
      "x" => Dim::X,
      "y" => Dim::Y,
      _ => panic!("{}", raw_dir),
    };
    let num = raw_num.parse::<i32>().unwrap();
    (dir, num)
  }).collect();
  Input { coords, folds }
}

fn build_grid(Input { coords, ..}: &Input) -> Grid {
  coords.iter().cloned().collect()
}

fn transpose_grid(grid: &mut Grid) {
  let new_grid = grid.drain().map(|(a,b)| (b,a)).collect::<Grid>();
  grid.extend(new_grid);
}

fn do_fold(grid: &mut Grid, fold: (Dim, i32)) {
  let (n, should_transpose) = match fold {
    (Dim::X, n) => (n, false),
    (Dim::Y, n) => {
      transpose_grid(grid);
      (n, true)
    }
  };

  let new_grid = grid.drain().map(|loc@(x,y)| {
    if x < n {
      loc
    } else {
      let diff = x - n;
      (n - diff, y)
    }
  }).collect::<Grid>();

  grid.extend(new_grid);

  if should_transpose {
    transpose_grid(grid);
  }
}

fn pretty_print_grid(grid: &Grid) {
  let max_x = grid.iter().map(|(x,_)| x).max().unwrap();
  let max_y = grid.iter().map(|(_,y)| y).max().unwrap();
  for y in 0..=*max_y {
    for x in 0..=*max_x {
      if grid.contains(&(x,y)) {
        print!("#")
      } else {
        print!(".")
      }
    }
    println!()
  }
}

#[aoc(day13, part1)]
fn part1(inp: &Input) -> usize {
  let mut grid = build_grid(inp);
  do_fold(&mut grid, inp.folds[0]);
  grid.len()
}

#[aoc(day13, part2)]
fn part2(inp: &Input) -> usize {
  let mut grid = build_grid(inp);
  for fold in &inp.folds {
    do_fold(&mut grid, *fold);
  }
  pretty_print_grid(&grid);
  grid.len()
}
