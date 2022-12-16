use std::collections::{HashMap, HashSet};

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Vec<Vec<((usize, usize), i32)>> {
    let without_indices: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();
    without_indices
        .into_iter()
        .enumerate()
        .map(|(r, row)| {
            row.into_iter()
                .enumerate()
                .map(|(c, num)| ((r, c), num))
                .collect()
        })
        .collect()
}

fn rotate<T: Clone + Copy>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut out = Vec::from_iter(std::iter::repeat(Vec::new()).take(input[0].len()));
    for col in 0..input[0].len() {
        for row in 0..input.len() {
            out[col].push(input[input.len() - row - 1][col]);
        }
    }
    out
}

fn visible_from_left(input: &Vec<Vec<((usize, usize), i32)>>) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    input.iter().for_each(|row| {
        out.push(row[0].0);
        out.extend(
            row.iter()
                .scan(0, |acc, elem| {
                    let (coords, height) = *elem;
                    if height > *acc {
                        *acc = height;
                        Some(Some(coords))
                    } else {
                        Some(None)
                    }
                })
                .flatten(),
        );
    });
    out
}

fn viewing_distance_to_left(
    input: &Vec<Vec<((usize, usize), i32)>>,
) -> HashMap<(usize, usize), usize> {
    let mut out = HashMap::new();
    for r in 0..input.len() {
        let mut height_to_last_col: Vec<Option<usize>> =
            Vec::from_iter(std::iter::repeat(None).take(10));
        for c in 0..input[0].len() {
            let height_here: usize = input[r][c].1.try_into().unwrap();
            assert!(height_here <= 9);
            let last_col_at_least_as_tall =
                height_to_last_col[height_here..].iter().flatten().max();
            if let Some(&col) = last_col_at_least_as_tall {
                out.insert(input[r][c].0, c - col);
            } else {
                out.insert(input[r][c].0, c);
            }
            height_to_last_col[height_here] = Some(c);
        }
    }
    out
}

pub fn part1_impl(input: &Vec<Vec<((usize, usize), i32)>>) -> usize {
    // println!("{input:?}");
    let mut results = HashSet::new();
    let mut inp: Vec<Vec<((usize, usize), i32)>> = input.clone();
    assert!(inp != rotate(rotate(inp.clone())));
    assert!(rotate(inp.clone()) != rotate(rotate(rotate(inp.clone()))));
    assert!(inp == rotate(rotate(rotate(rotate(inp.clone())))));
    for _ in 1..=4 {
        results.extend(visible_from_left(&inp));
        inp = rotate(inp);
    }
    // println!("{results:?}");
    results.len()
}

#[aoc(day8, part1)]
pub fn part1(input: &Vec<Vec<((usize, usize), i32)>>) -> usize {
    let sample = part1_impl(&parse(
        "30373
25512
65332
33549
35390",
    ));
    println!("sample result: {sample}");
    part1_impl(input)
}

pub fn part2_impl(input: &Vec<Vec<((usize, usize), i32)>>) -> usize {
    let input = input.clone();
    let a = viewing_distance_to_left(&input);
    let input = rotate(input);
    let b = viewing_distance_to_left(&input);
    let input = rotate(input);
    let c = viewing_distance_to_left(&input);
    let input = rotate(input);
    let d = viewing_distance_to_left(&input);
    input
        .iter()
        .flat_map(|row| row.iter().map(|(coords, _height)| coords))
        .map(|coords| {
            let result = a[coords] * b[coords] * c[coords] * d[coords];
            // println!("result for {coords:?} = {result}");
            result
        })
        .max()
        .unwrap()
}

#[aoc(day8, part2)]
pub fn part2(input: &Vec<Vec<((usize, usize), i32)>>) -> usize {
    let sample = part2_impl(&parse(
        "30373
25512
65332
33549
35390",
    ));
    println!("sample result: {sample}");
    part2_impl(input)
    // todo!()
}
