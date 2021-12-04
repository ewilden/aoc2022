// use bitvec::prelude::*;

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<Vec<i32>> {
  input.lines().map(|line| {
    let mut v = Vec::new();
    line.chars().for_each(|c| {
      match c {
        '0' => v.push(0),
        '1' => v.push(1),
        _ => panic!("not 0 or 1: {}, {}", c, line),
      }
    });
    v
  }).map(|v| v.try_into().unwrap()).collect()
}

fn to_decimal(arr: &Vec<i32>) -> i32 {
    arr.iter().fold(0, |sum, elem| {
      (sum << 1) + elem
    })
}

pub fn most_least_common(input: &Vec<Vec<i32>>, tiebreak: i32) -> (Vec<i32>, Vec<i32>) {
  let zeroes: Vec<i32> = input[0].iter().map(|_| 0).collect();
  let mut sum: Vec<i32> = zeroes.clone();
  for input_line in input.iter() {
    for (z, i) in sum.iter_mut().zip(input_line) {
      *z += i;
    }
  }
  let mut most_common = zeroes.clone();
  let mut least_common = zeroes.clone();
  for (ind, z) in sum.iter().enumerate() {
    let mut common = 0;
    if (*z * 2 + tiebreak) > input.len().try_into().unwrap() {
      common = 1;
    }
    most_common[ind] = common;
    least_common[ind] = 1 - common;
  }
  (most_common, least_common)
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<i32>>) -> i32 {
  let (gamma, epsilon) = most_least_common(input, 0);
  to_decimal(&gamma) * to_decimal(&epsilon)
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Vec<i32>>) -> i32 {
  let oxygen = {
    let mut candidates = input.clone();
    let mut bit_pos = 0;
    loop {
      if candidates.len() == 1 {
        break to_decimal(&candidates[0])
      }
      let (most, _) = most_least_common(&candidates, 1);
      candidates = candidates.into_iter().filter(|c| c[bit_pos] == most[bit_pos]).collect();
      bit_pos += 1;
    }
  };
  let co2 = {
    let mut candidates = input.clone();
    let mut bit_pos = 0;
    loop {
      if candidates.len() == 1 {
        break to_decimal(&candidates[0])
      }
      let (_, least) = most_least_common(&candidates, 1);
      candidates = candidates.into_iter().filter(|c| c[bit_pos] == least[bit_pos]).collect();
      bit_pos += 1;
    }
  };
  oxygen * co2
}