use std::{
    collections::{BTreeSet, HashMap},
    ops::{Add, AddAssign, Mul, Neg},
};

use itertools::{Either, Itertools};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Ore(i32);

impl Mul<i32> for Ore {
    type Output = Ore;
    fn mul(self, rhs: i32) -> Self::Output {
        Ore(self.0 * rhs)
    }
}

impl Add for Ore {
    type Output = Ore;
    fn add(self, rhs: Self) -> Self::Output {
        Ore(self.0 + rhs.0)
    }
}

impl AddAssign for Ore {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Ore {
    type Output = Ore;
    fn neg(self) -> Self::Output {
        Ore(-self.0)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Clay(i32);

impl Mul<i32> for Clay {
    type Output = Clay;
    fn mul(self, rhs: i32) -> Self::Output {
        Clay(self.0 * rhs)
    }
}

impl Add for Clay {
    type Output = Clay;
    fn add(self, rhs: Self) -> Self::Output {
        Clay(self.0 + rhs.0)
    }
}

impl AddAssign for Clay {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Clay {
    type Output = Clay;
    fn neg(self) -> Self::Output {
        Clay(-self.0)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Obsidian(i32);

impl Mul<i32> for Obsidian {
    type Output = Obsidian;
    fn mul(self, rhs: i32) -> Self::Output {
        Obsidian(self.0 * rhs)
    }
}

impl Add for Obsidian {
    type Output = Obsidian;
    fn add(self, rhs: Self) -> Self::Output {
        Obsidian(self.0 + rhs.0)
    }
}

impl AddAssign for Obsidian {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Obsidian {
    type Output = Obsidian;
    fn neg(self) -> Self::Output {
        Obsidian(-self.0)
    }
}

#[derive(Clone, Copy)]
pub struct Blueprint {
    pub ore_robot_cost: Ore,
    pub clay_robot_cost: Ore,
    pub obsidian_robot_cost: (Ore, Clay),
    pub geode_robot_cost: (Ore, Obsidian),
}

#[aoc_generator(day19)]
pub fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once("Each ore robot costs ").unwrap();
            let (ore, line) = line.split_once(" ore. Each clay robot costs ").unwrap();
            let (clay, line) = line.split_once(" ore. Each obsidian robot costs ").unwrap();
            let (obsidian_ore, line) = line.split_once(" ore and ").unwrap();
            let (obsidian_clay, line) = line.split_once(" clay. Each geode robot costs ").unwrap();
            let (geode_ore, line) = line.split_once(" ore and ").unwrap();
            let (geode_obsidian, _) = line.split_once(" obsidian.").unwrap();

            Blueprint {
                ore_robot_cost: Ore(ore.parse().unwrap()),
                clay_robot_cost: Ore(clay.parse().unwrap()),
                obsidian_robot_cost: (
                    Ore(obsidian_ore.parse().unwrap()),
                    Clay(obsidian_clay.parse().unwrap()),
                ),
                geode_robot_cost: (
                    Ore(geode_ore.parse().unwrap()),
                    Obsidian(geode_obsidian.parse().unwrap()),
                ),
            }
        })
        .collect()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Robots {
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
}

impl Default for Robots {
    fn default() -> Self {
        Robots {
            ore_robots: 0,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

impl Add for Robots {
    type Output = Robots;
    fn add(self, rhs: Self) -> Self::Output {
        Robots {
            ore_robots: self.ore_robots + rhs.ore_robots,
            clay_robots: self.clay_robots + rhs.clay_robots,
            obsidian_robots: self.obsidian_robots + rhs.obsidian_robots,
            geode_robots: self.geode_robots + rhs.geode_robots,
        }
    }
}

impl Robots {
    fn mine(&self) -> (Resources, i32) {
        let &Robots {
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots,
        } = self;
        (
            Resources {
                ore: Ore(ore_robots),
                clay: Clay(clay_robots),
                obsidian: Obsidian(obsidian_robots),
            },
            geode_robots,
        )
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Resources {
    ore: Ore,
    clay: Clay,
    obsidian: Obsidian,
}

impl Add for Resources {
    type Output = Resources;
    fn add(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
        }
    }
}

impl Resources {
    fn spend(
        self,
        Blueprint {
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost: (obs_ore_cost, obs_clay_cost),
            geode_robot_cost: (geode_ore_cost, geode_obs_cost),
        }: Blueprint,
        Robots {
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots,
        }: Robots,
    ) -> Option<Resources> {
        let Resources {
            mut ore,
            mut clay,
            mut obsidian,
        } = self;
        ore += -(ore_robot_cost * ore_robots
            + clay_robot_cost * clay_robots
            + obs_ore_cost * obsidian_robots
            + geode_ore_cost * geode_robots);
        clay += -obs_clay_cost * obsidian_robots;
        obsidian += -geode_obs_cost * geode_robots;
        [ore.0, clay.0, obsidian.0]
            .into_iter()
            .all(|x| x >= 0)
            .then_some(Resources {
                ore,
                clay,
                obsidian,
            })
    }

    fn options(
        self,
        blueprint: Blueprint,
        ore: impl Iterator<Item = i32>,
        clay: impl Iterator<Item = i32>,
        obsidian: impl Iterator<Item = i32>,
        geode: impl Iterator<Item = i32>,
    ) -> impl Iterator<Item = Robots> {
        let max_ore = (0..)
            .take_while(|&ore_robots| {
                self.spend(
                    blueprint,
                    Robots {
                        ore_robots,
                        ..Default::default()
                    },
                )
                .is_some()
            })
            .last()
            .unwrap();
        let max_clay = (0..)
            .take_while(|&clay_robots| {
                self.spend(
                    blueprint,
                    Robots {
                        clay_robots,
                        ..Default::default()
                    },
                )
                .is_some()
            })
            .last()
            .unwrap();
        let max_obsidian = (0..)
            .take_while(|&obsidian_robots| {
                self.spend(
                    blueprint,
                    Robots {
                        obsidian_robots,
                        ..Default::default()
                    },
                )
                .is_some()
            })
            .last()
            .unwrap();
        let max_geode = (0..)
            .take_while(|&geode_robots| {
                self.spend(
                    blueprint,
                    Robots {
                        geode_robots,
                        ..Default::default()
                    },
                )
                .is_some()
            })
            .last()
            .unwrap();
        // println!("{max_ore}, {max_clay}, {max_obsidian}, {max_geode}");
        (ore.take_while(move |x| x <= &max_ore))
            .cartesian_product(clay.take_while(|&x| x <= max_clay).collect_vec())
            .cartesian_product(obsidian.take_while(|&x| x <= max_obsidian).collect_vec())
            .cartesian_product(geode.take_while(|&x| x <= max_geode).collect_vec())
            .filter_map(
                move |(((ore_robots, clay_robots), obsidian_robots), geode_robots)| {
                    let robots = Robots {
                        ore_robots,
                        clay_robots,
                        obsidian_robots,
                        geode_robots,
                    };
                    let _resources = self.spend(blueprint, robots)?;
                    // [
                    //     Robots {
                    //         ore_robots: 1,
                    //         ..Default::default()
                    //     },
                    //     Robots {
                    //         clay_robots: 1,
                    //         ..Default::default()
                    //     },
                    //     Robots {
                    //         obsidian_robots: 1,
                    //         ..Default::default()
                    //     },
                    //     Robots {
                    //         geode_robots: 1,
                    //         ..Default::default()
                    //     },
                    // ]
                    // .into_iter()
                    // .all(move |robots| resources.spend(blueprint, robots).is_none())
                    // .then_some(robots)
                    Some(robots)
                },
            )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct State {
    minutes_left: i32,
    resources: Resources,
    robots: Robots,
    geodes: i32,
    previously_made: BTreeSet<RobotType>,
}

struct Solver19 {
    blueprint: Blueprint,
    cache: HashMap<State, i32>,
}

impl Solver19 {
    fn solve(&mut self, state: State) -> i32 {
        if !self.cache.contains_key(&state) {
            let result = self.solve_impl(state.clone());
            self.cache.insert(state, result);
            return result;
        }
        self.cache[&state]
    }

    fn solve_impl(&mut self, state: State) -> i32 {
        let State {
            minutes_left,
            robots,
            resources,
            geodes,
            previously_made,
        } = state.clone();
        if minutes_left == 0 {
            return geodes;
        }

        let robot_options = resources.options(
            self.blueprint,
            {
                if !previously_made.is_disjoint(&BTreeSet::from_iter([
                    RobotType::Clay,
                    RobotType::Obsidian,
                    RobotType::Geode,
                ])) {
                    Either::Left(0..=0)
                } else {
                    Either::Right(0..)
                }
            },
            {
                if !previously_made.is_disjoint(&BTreeSet::from_iter([
                    RobotType::Obsidian,
                    RobotType::Geode,
                ])) {
                    Either::Left(0..=0)
                } else {
                    Either::Right(0..)
                }
            },
            {
                if !previously_made.is_disjoint(&BTreeSet::from_iter([RobotType::Geode])) {
                    Either::Left(0..=0)
                } else {
                    Either::Right(0..)
                }
            },
            0..,
        );
        let (mined_resources, new_geodes) = robots.mine();

        if new_geodes > 0 {
            println!("made {new_geodes} geodes!!!!!!!!");
        }

        if minutes_left == 1 {
            return new_geodes + geodes;
        }

        let mut best = geodes + new_geodes;
        for option in robot_options {
            let previously_made = {
                let mut previously_made = previously_made.clone();
                if option.ore_robots > 0 {
                    previously_made.insert(RobotType::Ore);
                }
                if option.clay_robots > 0 {
                    previously_made.insert(RobotType::Clay);
                }
                if option.obsidian_robots > 0 {
                    previously_made.insert(RobotType::Obsidian);
                }
                if option.geode_robots > 0 {
                    previously_made.insert(RobotType::Geode);
                }
                previously_made
            };
            best = best.max(self.solve(State {
                minutes_left: minutes_left - 1,
                resources: resources.spend(self.blueprint, option).unwrap() + mined_resources,
                robots: robots + option,
                geodes: geodes + new_geodes,
                previously_made,
            }))
        }

        best
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &[Blueprint]) -> i32 {
    let input = &parse("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
    input
        .iter()
        .enumerate()
        .map(|(i, &blueprint)| {
            let mut solver = Solver19 {
                blueprint,
                cache: HashMap::new(),
            };
            let sol = solver.solve(State {
                minutes_left: 10,
                resources: Resources {
                    ore: Ore(0),
                    clay: Clay(0),
                    obsidian: Obsidian(0),
                },
                robots: Robots {
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                },
                geodes: 0,
                previously_made: BTreeSet::new(),
            });
            let i = i + 1;
            println!(
                "solved blueprint {i} with score {sol} and state size {}",
                solver.cache.len()
            );
            i32::try_from(i).unwrap() * sol
        })
        .sum()
}
