use std::collections::HashMap;

use itertools::Itertools;

#[aoc_generator(day14)]
pub fn parse(input: &str) -> Vec<Vec<(i32, i32)>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| {
                    let (a, b) = pair.split_once(",").unwrap();
                    (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
                })
                .collect()
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(input: &Vec<Vec<(i32, i32)>>) -> i32 {
    let mut grid: HashMap<(i32, i32), ()> = HashMap::new();
    let mut bottom = 0;
    for path in input {
        for (a, b) in path.iter().tuple_windows() {
            for x in a.0.min(b.0)..=a.0.max(b.0) {
                for y in a.1.min(b.1)..=a.1.max(b.1) {
                    grid.insert((x, y), ());
                    if y > bottom {
                        bottom = y;
                    }
                }
            }
        }
    }
    let bottom = bottom;
    let mut count = 0;
    loop {
        let mut sand_loc = (500, 0);
        let exited = loop {
            if sand_loc.1 > bottom {
                break true;
            }

            if !grid.contains_key(&(sand_loc.0, sand_loc.1 + 1)) {
                sand_loc.1 += 1;
                continue;
            }

            if !grid.contains_key(&(sand_loc.0 - 1, sand_loc.1 + 1)) {
                sand_loc.0 -= 1;
                sand_loc.1 += 1;
                continue;
            }

            if !grid.contains_key(&(sand_loc.0 + 1, sand_loc.1 + 1)) {
                sand_loc.0 += 1;
                sand_loc.1 += 1;
                continue;
            }

            break false;
        };

        if exited {
            break;
        } else {
            count += 1;
            grid.insert(sand_loc, ());
        }
    }

    count
}

#[aoc(day14, part2)]
pub fn part2(input: &Vec<Vec<(i32, i32)>>) -> i32 {
    let mut grid: HashMap<(i32, i32), ()> = HashMap::new();
    let mut bottom = 0;
    for path in input {
        for (a, b) in path.iter().tuple_windows() {
            for x in a.0.min(b.0)..=a.0.max(b.0) {
                for y in a.1.min(b.1)..=a.1.max(b.1) {
                    grid.insert((x, y), ());
                    if y > bottom {
                        bottom = y;
                    }
                }
            }
        }
    }
    let bottom = bottom + 2;
    let mut count = 0;
    loop {
        let mut sand_loc = (500, 0);
        loop {
            if sand_loc.1 == bottom - 1 {
                break;
            }

            if !grid.contains_key(&(sand_loc.0, sand_loc.1 + 1)) {
                sand_loc.1 += 1;
                continue;
            }

            if !grid.contains_key(&(sand_loc.0 - 1, sand_loc.1 + 1)) {
                sand_loc.0 -= 1;
                sand_loc.1 += 1;
                continue;
            }

            if !grid.contains_key(&(sand_loc.0 + 1, sand_loc.1 + 1)) {
                sand_loc.0 += 1;
                sand_loc.1 += 1;
                continue;
            }

            break;
        }

        count += 1;
        grid.insert(sand_loc, ());

        if grid.contains_key(&(500, 0)) {
            break;
        }
    }

    count
}
