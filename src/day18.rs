use std::collections::HashSet;

use petgraph::unionfind::UnionFind;

#[aoc_generator(day18)]
pub fn parse(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .map(|line| {
            let (x, line) = line.split_once(",").unwrap();
            let (y, z) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        })
        .collect()
}

fn add((a, b, c): (i32, i32, i32), (x, y, z): (i32, i32, i32)) -> (i32, i32, i32) {
    (a + x, b + y, c + z)
}

#[aoc(day18, part1)]
pub fn part1(input: &[(i32, i32, i32)]) -> usize {
    let mut grid: HashSet<(i32, i32, i32)> = HashSet::new();
    for &point in input {
        grid.insert(point);
    }
    grid.iter()
        .map(|&point| {
            [
                (1, 0, 0),
                (-1, 0, 0),
                (0, 1, 0),
                (0, -1, 0),
                (0, 0, 1),
                (0, 0, -1),
            ]
            .into_iter()
            .filter(|&delta| !grid.contains(&add(point, delta)))
            .count()
        })
        .sum()
}

fn unionize_point((x, y, z): (i32, i32, i32)) -> usize {
    usize::try_from(x + 10).unwrap()
        + usize::try_from(y + 10).unwrap() * 100
        + usize::try_from(z + 10).unwrap() * 10000
}

#[aoc(day18, part2)]
pub fn part2(input: &[(i32, i32, i32)]) -> usize {
    let mut grid: HashSet<(i32, i32, i32)> = HashSet::new();
    for &point in input {
        grid.insert(point);
    }
    let max_x = grid.iter().map(|point| point.0).max().unwrap();
    let max_y = grid.iter().map(|point| point.1).max().unwrap();
    let max_z = grid.iter().map(|point| point.2).max().unwrap();

    let mut union_find = UnionFind::new(unionize_point((max_x + 2, max_y + 2, max_z + 2)));
    for x in (-1)..=(max_x + 1) {
        for y in (-1)..=(max_y + 1) {
            for z in (-1)..=(max_z + 1) {
                let point = (x, y, z);
                if !grid.contains(&point) {
                    for delta in [
                        (1, 0, 0),
                        (-1, 0, 0),
                        (0, 1, 0),
                        (0, -1, 0),
                        (0, 0, 1),
                        (0, 0, -1),
                    ] {
                        let neighbor = add(point, delta);
                        if !grid.contains(&neighbor) {
                            union_find.union(unionize_point(point), unionize_point(neighbor));
                        }
                    }
                }
            }
        }
    }

    grid.iter()
        .map(|&point| {
            [
                (1, 0, 0),
                (-1, 0, 0),
                (0, 1, 0),
                (0, -1, 0),
                (0, 0, 1),
                (0, 0, -1),
            ]
            .into_iter()
            .filter(|&delta| {
                let neighbor = add(point, delta);
                union_find.equiv(unionize_point((-1, -1, -1)), unionize_point(neighbor))
            })
            .count()
        })
        .sum()
}
