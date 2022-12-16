use std::collections::{BTreeMap, BTreeSet, HashSet};

use itertools::Itertools;

#[aoc_generator(day15)]
pub fn parse(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    input
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Sensor at ").unwrap();
            let mut points = line.split(": closest beacon is at ").map(|s| {
                let s = s.strip_prefix("x=").unwrap();
                let (x, y) = s.split_once(", y=").unwrap();
                (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
            });
            (points.next().unwrap(), points.next().unwrap())
        })
        .collect()
}

#[derive(Clone, Copy)]
pub enum Point {
    Sensor,
    Beacon,
}

fn manhattan(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

const Y: i64 = 2000000;

#[aoc(day15, part1)]
pub fn part1(input: &[((i64, i64), (i64, i64))]) -> usize {
    println!(
        "{:?}",
        part1_impl(
            &parse(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
            ),
            10
        )
    );
    part1_impl(input, Y)
}

pub fn part1_impl(input: &[((i64, i64), (i64, i64))], y: i64) -> usize {
    let mut beaconset: BTreeSet<i64> = BTreeSet::new();
    for &(_sensor, beacon) in input {
        if beacon.1 == y {
            beaconset.insert(beacon.0);
        }
    }
    let mut intervals: Vec<(i64, i64)> = Vec::new();
    for &(sensor, beacon) in input {
        let radius = manhattan(sensor, beacon);
        let height = (sensor.1 - y).abs();
        if height > radius {
            continue;
        }
        let center = sensor.0;
        let margin = radius - height;
        let new_range = (center - margin, center + margin);
        intervals.push(new_range);
        intervals.sort();
        intervals = intervals
            .into_iter()
            .coalesce(|a @ (l0, r0), b @ (l1, r1)| {
                if l1 <= r0 {
                    Ok((l0.min(l1), r0.max(r1)))
                } else {
                    Err((a, b))
                }
            })
            .collect();
    }

    intervals
        .into_iter()
        .map(|(l, r)| {
            usize::try_from(r - l + 1).unwrap()
                - beaconset.iter().filter(|&&x| x >= l && x <= r).count()
        })
        .sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &[((i64, i64), (i64, i64))]) -> i64 {
    println!(
        "{}",
        part2_impl(
            &parse(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
            ),
            20,
            20
        )
    );
    part2_impl(input, 4000000, 4000000)
}

fn part2_impl(input: &[((i64, i64), (i64, i64))], limit_x: i64, limit_y: i64) -> i64 {
    // lp constraints:
    // 0 <= x <= limit_x
    // 0 <= y <= limit_y
    // for (sensor, beacon) in input:
    //   manhattan(sensor, beacon) < manhattan(sensor, (x, y))
    // ~ sensor_dist < |sensor_x - x| + |sensor_y - y|
    // ~
    let candidates: HashSet<(i64, i64)> = input
        .iter()
        .flat_map(|&(sensor, beacon)| {
            let radius = manhattan(sensor, beacon) + 1;
            let mut vec = Vec::new();
            {
                for offset in 0..=radius {
                    let x = sensor.0 + offset;
                    let y = sensor.1 + radius - offset;
                    vec.push((x, y));
                }

                for offset in 0..=radius {
                    let x = sensor.0 + (-offset);
                    let y = sensor.1 + -(radius - offset);
                    vec.push((x, y));
                }

                for offset in 0..=radius {
                    let x = sensor.0 + offset;
                    let y = sensor.1 + -(radius - offset);
                    vec.push((x, y));
                }

                for offset in 0..=radius {
                    let x = sensor.0 + (-offset);
                    let y = sensor.1 + radius - offset;
                    vec.push((x, y));
                }
            }
            vec
        })
        .collect();

    for &candidate in candidates
        .iter()
        .filter(|&&(x, y)| x >= 0 && x <= limit_x && y >= 0 && y <= limit_y)
    {
        if input
            .iter()
            .all(|&(sensor, beacon)| manhattan(sensor, candidate) > manhattan(sensor, beacon))
        {
            return candidate.0 * 4000000 + candidate.1;
        }
    }

    unreachable!()
}
