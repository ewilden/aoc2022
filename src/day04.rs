use std::collections::VecDeque;

#[derive(Clone)]
struct Board<T> ([[T; 5]; 5]);

impl<T> From<[[T; 5]; 5]> for Board<T> {
  fn from(arr: [[T; 5]; 5]) -> Self {
    Board(arr)
  }
}

impl<T> Board<T> {
  fn map<F, S>(self, mut func: F) -> Board<S>
      where F: FnMut(T) -> S {
    Board(self.0.into_iter().map(|row| {
      row.into_iter().map(|t| func(t)).collect::<Vec<S>>().try_into().map_err(|_| "map failed").unwrap()
    }).collect::<Vec<[S; 5]>>().try_into().map_err(|_| "map failed").unwrap())
  }
}

#[derive(Clone)]
struct Input<T> {
  nums: VecDeque<i32>,
  boards: Vec<Board<T>>,
}

#[aoc_generator(day4)]
fn parse(inp: &str) -> Input<i32> {
  let mut inp_iter = inp.split("\n\n");
  let raw_nums = inp_iter.next().unwrap();
  let nums: VecDeque<i32> = raw_nums.split(',').map(|s| s.parse().unwrap()).collect();
  let boards: Vec<Board<i32>> = inp_iter.map(|raw_board| {
    Board(raw_board.split('\n')
      .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<i32>>().try_into().unwrap())
      .collect::<Vec<[i32; 5]>>().try_into().unwrap())
  }).collect();
  Input { nums, boards}
}

fn choose_winner(boards: &Vec<Board<Option<i32>>>) -> Option<(usize, &Board<Option<i32>>)> {
  for (i, board) in boards.iter().enumerate() {
    for row in board.0.iter() {
      if row.iter().all(|o| o.is_none()) {
        return Some((i, board))
      }
    }
    for col_ind in 0..=4 {
      if board.0.iter().map(|row| row[col_ind]).all(|o| o.is_none()) {
        return Some((i, board))
      }
    }
  }
  None
}

#[aoc(day4, part1)]
fn part1(inp: &Input<i32>) -> i32 {
  let Input { nums: orig_nums, boards: orig_boards } = inp.clone();
  let mut state = Input { 
    nums: orig_nums.clone(), 
    boards: orig_boards.into_iter().map(|b| b.map(Some)).collect(),
  };
  let mut last_num = None;
  let winner = loop {
    if let Some(board) = choose_winner(&state.boards) {
      break board
    }
    let num = state.nums.pop_front().unwrap();
    last_num = Some(num);
    for Board(board) in state.boards.iter_mut() {
      for row in board.iter_mut() {
        for item in row.iter_mut() {
          if *item == Some(num) {
            *item = None
          }
        }
      }
    }
  };
  winner.1.clone().map(|o| o.unwrap_or(0)).0.iter().map(|row| row.iter().sum::<i32>()).sum::<i32>()
  * last_num.unwrap()
}

#[aoc(day4, part2)]
fn part2(inp: &Input<i32>) -> i32 {
  let Input { nums: orig_nums, boards: orig_boards } = inp.clone();
  let mut state = Input { 
    nums: orig_nums.clone(), 
    boards: orig_boards.into_iter().map(|b| b.map(Some)).collect(),
  };
  let mut last_num = None;
  let mut do_remove = true;
  loop {
    if state.boards.len() == 1 {
      do_remove = false;
    }
    if do_remove {
      while let Some((i, board)) = choose_winner(&state.boards) {
        state.boards.swap_remove(i);
      }
    } else if let Some((_, winner)) = choose_winner(&state.boards) {
      return last_num.unwrap() * winner.clone().map(|o| o.unwrap_or(0)).0.iter().map(|row| row.iter().sum::<i32>()).sum::<i32>()
    }
    let num = state.nums.pop_front().unwrap();
    last_num = Some(num);
    for Board(board) in state.boards.iter_mut() {
      for row in board.iter_mut() {
        for item in row.iter_mut() {
          if *item == Some(num) {
            *item = None
          }
        }
      }
    }
  };
}