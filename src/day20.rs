use std::{collections::BTreeMap, ops::Bound};

use itertools::Itertools;
use num::{BigInt, BigRational};

#[aoc_generator(day20)]
pub fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

fn bigrat(i: impl Into<BigInt>) -> BigRational {
    BigRational::from_integer(i.into())
}

#[aoc(day20, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let _input = &parse(
        "1
2
-3
3
-2
0
4",
    );
    let mut weight_to_num = input
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let i = BigRational::from_integer(BigInt::from(i));
            (i, *n)
        })
        .collect::<BTreeMap<_, _>>();
    for (index, n) in input.iter().copied().enumerate() {
        let index = bigrat(index);
        let mut steps =
            usize::try_from(n.rem_euclid(i64::try_from(input.len() - 1).unwrap())).unwrap();
        if steps == 0 {
            continue;
        }
        assert!(weight_to_num.remove(&index) == Some(n));
        let mut traversal = weight_to_num.range((Bound::Excluded(index), Bound::Unbounded));
        let result = {
            loop {
                if steps == 1 {
                    break traversal.next().ok_or(steps);
                }
                assert!(steps > 1);
                match traversal.next() {
                    Some(_) => (),
                    None => break Err(steps),
                }
                steps -= 1;
            }
        };
        match result {
            Ok(lower_bound) => {
                if let Some(upper_bound) = traversal.next() {
                    assert!(matches!(
                        weight_to_num.insert((lower_bound.0 + upper_bound.0) / bigrat(2), n),
                        None
                    ));
                } else {
                    let new_k = lower_bound.0.clone() + bigrat(1);
                    assert!(matches!(weight_to_num.insert(new_k, n), None));
                }
            }
            Err(steps_remaining) => {
                assert!(steps_remaining >= 1);
                let lower_bound = weight_to_num.iter().nth(steps_remaining - 1).unwrap();
                let upper_bound = weight_to_num.iter().nth(steps_remaining).unwrap();
                assert!(matches!(
                    weight_to_num.insert((lower_bound.0 + upper_bound.0) / bigrat(2), n),
                    None
                ));
            }
        }
    }
    let zero_index = weight_to_num
        .iter()
        .enumerate()
        .find_map(|(ind, (_, &x))| (x == 0).then(|| ind))
        .unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|index| {
            weight_to_num
                .values()
                .nth((index + zero_index) % input.len())
                .unwrap()
        })
        .sum()
}

#[aoc(day20, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let _input = &parse(
        "1
2
-3
3
-2
0
4",
    );
    let input = &input.iter().copied().map(|x| x * 811589153).collect_vec();
    let mut weight_to_num = input
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let i = bigrat(i);
            (i, *n)
        })
        .collect::<BTreeMap<_, _>>();
    let mut weights = Vec::from_iter(input.iter().enumerate().map(|(i, _)| bigrat(i)));

    for _ in 1..=10 {
        for (index, n) in input.iter().copied().enumerate() {
            let weight = weights[index].clone();
            let mut steps =
                usize::try_from(n.rem_euclid(i64::try_from(input.len() - 1).unwrap())).unwrap();
            if steps == 0 {
                continue;
            }
            assert!(weight_to_num.remove(&weight) == Some(n));
            let mut traversal = weight_to_num.range((Bound::Excluded(weight), Bound::Unbounded));
            let result = {
                loop {
                    if steps == 1 {
                        break traversal.next().ok_or(steps);
                    }
                    assert!(steps > 1);
                    match traversal.next() {
                        Some(_) => (),
                        None => break Err(steps),
                    }
                    steps -= 1;
                }
            };
            match result {
                Ok(lower_bound) => {
                    if let Some(upper_bound) = traversal.next() {
                        let new_weight = (lower_bound.0 + upper_bound.0) / bigrat(2);
                        assert!(matches!(weight_to_num.insert(new_weight.clone(), n), None));
                        weights[index] = new_weight;
                    } else {
                        let new_weight = lower_bound.0.clone() + bigrat(1);
                        assert!(matches!(weight_to_num.insert(new_weight.clone(), n), None));
                        weights[index] = new_weight.clone();
                    }
                }
                Err(steps_remaining) => {
                    assert!(steps_remaining >= 1);
                    let lower_bound = weight_to_num.iter().nth(steps_remaining - 1).unwrap();
                    let upper_bound = weight_to_num.iter().nth(steps_remaining).unwrap();
                    let new_weight = (lower_bound.0 + upper_bound.0) / bigrat(2);
                    assert!(matches!(weight_to_num.insert(new_weight.clone(), n), None));
                    weights[index] = new_weight.clone();
                }
            }
        }
    }
    let zero_index = weight_to_num
        .iter()
        .enumerate()
        .find_map(|(ind, (_, &x))| (x == 0).then(|| ind))
        .unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|index| {
            weight_to_num
                .values()
                .nth((index + zero_index) % input.len())
                .unwrap()
        })
        .sum()
}
