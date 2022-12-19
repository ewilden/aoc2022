use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

#[derive(Clone, Copy)]
pub enum Jet {
    Left,
    Right,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn add(
        self,
        Point {
            x: other_x,
            y: other_y,
        }: Point,
    ) -> Point {
        let Point { x, y } = self;
        Point {
            x: x + other_x,
            y: y + other_y,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct Rock {
    pub offset: Point,
    pub body_pattern: HashSet<Point>,
}

impl Rock {
    pub fn body(&self) -> HashSet<Point> {
        self.body_pattern
            .iter()
            .map(|point| (*point).add(self.offset))
            .collect()
    }

    pub fn pretty(&self) {
        let points = self.body_pattern.clone();
        let max_y = points.iter().map(|point| point.y).max().unwrap();
        let max_x = points.iter().map(|point| point.x).max().unwrap();
        for y in (0..=max_y).rev() {
            for x in 0..=max_x {
                if points.contains(&((x, y).into())) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }

    pub fn displaced(self, jet: Jet) -> Rock {
        let Rock {
            offset,
            body_pattern,
        } = self.clone();
        let next = match jet {
            Jet::Left => Rock {
                body_pattern,
                offset: offset.add(Point { x: -1, y: 0 }),
            },
            Jet::Right => Rock {
                body_pattern,
                offset: offset.add(Point { x: 1, y: 0 }),
            },
        };
        if next
            .body()
            .into_iter()
            .any(|point| point.x < 0 || point.x > 6)
        {
            self
        } else {
            next
        }
    }

    pub fn dropped(self) -> Rock {
        let Rock {
            offset,
            body_pattern,
        } = self;
        Rock {
            body_pattern,
            offset: offset.add(Point { x: 0, y: -1 }),
        }
    }
}

#[derive(Debug)]
pub struct Tower {
    pub occupied: HashSet<Point>,
    pub rockmod: i64,
    pub height: i64,
}

impl Tower {
    pub fn new() -> Tower {
        Tower {
            occupied: HashSet::new(),
            rockmod: 0,
            height: 0,
        }
    }

    pub fn jet(&self, rock: Rock, jet: Jet) -> Rock {
        let next = rock.clone().displaced(jet);
        if next.body().is_disjoint(&self.occupied) {
            next
        } else {
            rock
        }
    }

    pub fn drop(&self, rock: Rock) -> Result<Rock, Rock> {
        let next = rock.clone().dropped();
        if !next.body().is_disjoint(&self.occupied)
            || next.body().into_iter().any(|point| point.y < 0)
        {
            Err(rock)
        } else {
            Ok(next)
        }
    }

    pub fn place(&mut self, rock: Rock) -> Option<i64> {
        assert!(rock.body().is_disjoint(&self.occupied));
        assert!(matches!(self.drop(rock.clone()), Err(_)));
        self.occupied.extend(rock.body());
        self.height = self
            .height
            .max(rock.body().into_iter().map(|point| point.y).max().unwrap() + 1);

        let ys_to_check = rock
            .body()
            .into_iter()
            .map(|point| point.y)
            .collect::<HashSet<_>>();
        for y in ys_to_check.into_iter().sorted_by_key(|y| -y) {
            if (0i64..7i64).all(|x| self.occupied.contains(&(x, y).into())) {
                return Some(y);
            }
        }
        None
    }

    pub fn spawn(&mut self) -> Rock {
        let Rock {
            offset: _,
            body_pattern,
        } = self.next_rock();
        Rock {
            offset: (2, self.height + 3).into(),
            body_pattern,
        }
    }

    pub fn next_rock(&mut self) -> Rock {
        let curr_rockmod = self.rockmod;
        self.rockmod = (self.rockmod + 1) % 5;
        match curr_rockmod {
            0 => Rock {
                offset: Point::default(),
                body_pattern: HashSet::from_iter(
                    [(0, 0), (1, 0), (2, 0), (3, 0)]
                        .into_iter()
                        .map(Point::from),
                ),
            },
            1 => Rock {
                offset: Point::default(),
                body_pattern: HashSet::from_iter(
                    [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]
                        .into_iter()
                        .map(Point::from),
                ),
            },
            2 => Rock {
                offset: Point::default(),
                body_pattern: HashSet::from_iter(
                    [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]
                        .into_iter()
                        .map(Point::from),
                ),
            },
            3 => Rock {
                offset: Point::default(),
                body_pattern: HashSet::from_iter(
                    [(0, 0), (0, 1), (0, 2), (0, 3)]
                        .into_iter()
                        .map(Point::from),
                ),
            },
            4 => Rock {
                offset: Point::default(),
                body_pattern: HashSet::from_iter(
                    [(0, 0), (0, 1), (1, 0), (1, 1)]
                        .into_iter()
                        .map(Point::from),
                ),
            },
            n => panic!("{n}"),
        }
    }
}

#[aoc_generator(day17)]
pub fn parse(input: &str) -> Vec<Jet> {
    input
        .chars()
        .map(|c| match c {
            '>' => Jet::Right,
            '<' => Jet::Left,
            _ => panic!("{c}"),
        })
        .collect()
}

#[aoc(day17, part1)]
pub fn part1(input: &[Jet]) -> i64 {
    // {
    //     let mut tower = Tower::new();
    //     for _ in 1..=5 {
    //         tower.spawn().pretty();
    //         println!()
    //     }
    // }

    println!(
        "{}",
        part1_impl(&parse(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"))
    );
    part1_impl(input)
    // 0
}

pub fn part1_impl(input: &[Jet]) -> i64 {
    let mut num_rocks_placed = 0;
    let mut tower = Tower::new();
    let mut jet_i = 0;
    while num_rocks_placed < 2022 {
        let mut rock = tower.spawn();
        loop {
            // println!("rock: {rock:?}");
            // println!("tower: {tower:?}");
            rock = tower.jet(rock, input[jet_i]);
            jet_i = (jet_i + 1) % input.len();
            rock = match tower.drop(rock) {
                Ok(rock) => rock,
                Err(rock) => {
                    tower.place(rock);
                    num_rocks_placed += 1;
                    // println!("{num_rocks_placed} rocks placed");
                    break;
                }
            }
        }
    }
    tower.height
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CacheKey {
    pub footprint: BTreeSet<Point>,
    pub jet_i: usize,
    pub rockmod: i64,
}

#[derive(Clone, Copy, Debug)]
pub struct CacheVal {
    pub full_y: i64,
    pub num_rocks_placed: i64,
}

const MAX_ROCKS: i64 = 1000000000000;

#[aoc(day17, part2)]
pub fn part2(input: &[Jet]) -> i64 {
    let mut num_rocks_placed: i64 = 0;
    let mut tower = Tower::new();
    let mut jet_i = 0;
    let mut cache: HashMap<CacheKey, CacheVal> = HashMap::new();
    let mut artificial_height = 0i64;
    while num_rocks_placed < MAX_ROCKS {
        let mut rock = tower.spawn();
        let newly_full_y = loop {
            rock = tower.jet(rock, input[jet_i]);
            jet_i = (jet_i + 1) % input.len();
            rock = match tower.drop(rock) {
                Ok(rock) => rock,
                Err(rock) => {
                    let newly_full_y = tower.place(rock);
                    num_rocks_placed += 1;
                    break newly_full_y;
                }
            }
        };
        if let Some(newly_full_y) = newly_full_y {
            let mut footprint: BTreeSet<Point> = BTreeSet::new();
            let height = tower.height;
            let rockmod = tower.rockmod;
            for x in 0..7 {
                for y in newly_full_y..height {
                    if tower.occupied.contains(&(x, y).into()) {
                        footprint.insert((x, y - height).into());
                    }
                }
            }
            let cache_key = CacheKey {
                footprint,
                jet_i,
                rockmod,
            };
            if let Some(old_cache_val) = cache.get(&cache_key).cloned() {
                println!("cache hit: {}", cache.len());
                let CacheVal {
                    full_y: old_full_y,
                    num_rocks_placed: old_num_rocks_placed,
                } = old_cache_val;
                let full_y_delta = newly_full_y - old_full_y;
                let rocks_placed_delta = num_rocks_placed - old_num_rocks_placed;
                let num_cycles_to_do = (MAX_ROCKS - num_rocks_placed) / rocks_placed_delta;
                artificial_height += num_cycles_to_do * full_y_delta;
                num_rocks_placed += num_cycles_to_do * rocks_placed_delta;
                cache.clear();
            } else {
                let new_cache_val = CacheVal {
                    full_y: newly_full_y,
                    num_rocks_placed: num_rocks_placed,
                };
                cache.insert(cache_key, new_cache_val);
            }
        }
    }
    tower.height + artificial_height
}
