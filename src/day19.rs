use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Mul, Neg},
};

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
            let line = line.trim();
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

    fn options(self, blueprint: Blueprint) -> impl Iterator<Item = Robots> {
        std::iter::once(Robots::default()).chain(
            [
                Robots {
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                },
                Robots {
                    ore_robots: 0,
                    clay_robots: 1,
                    obsidian_robots: 0,
                    geode_robots: 0,
                },
                Robots {
                    ore_robots: 0,
                    clay_robots: 0,
                    obsidian_robots: 1,
                    geode_robots: 0,
                },
                Robots {
                    ore_robots: 0,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 1,
                },
            ]
            .into_iter()
            .filter(move |&robots| self.spend(blueprint, robots).is_some()),
        )
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct State {
    minutes_left: i32,
    resources: Resources,
    robots: Robots,
    first_minute: Robots,
    last_minute: Robots,
}

struct Solver19 {
    blueprint: Blueprint,
    cache: HashMap<State, i32>,
}

impl Solver19 {
    fn solve(&mut self, state: State) -> i32 {
        if !self.cache.contains_key(&state) {
            let result = self.solve_impl(state.clone());
            // self.cache.insert(state, result);
            return result;
        }
        self.cache[&state]
        // self.solve_impl(state)
    }

    fn solve_impl(&mut self, state: State) -> i32 {
        let State {
            minutes_left,
            robots,
            resources,
            first_minute,
            last_minute,
        } = state.clone();
        if minutes_left == 0 {
            return 0;
        }

        let robot_options = resources.options(self.blueprint);
        let (mined_resources, new_geodes) = robots.mine();

        let mut best = 0;
        for option in robot_options {
            if option.ore_robots > 0
                && !(last_minute.ore_robots..=first_minute.ore_robots).contains(&minutes_left)
            {
                continue;
            }
            if option.clay_robots > 0
                && !(last_minute.clay_robots..=first_minute.clay_robots).contains(&minutes_left)
            {
                continue;
            }
            if option.obsidian_robots > 0
                && !(last_minute.obsidian_robots..=first_minute.obsidian_robots)
                    .contains(&minutes_left)
            {
                continue;
            }
            if option.geode_robots > 0
                && !(last_minute.geode_robots..=first_minute.geode_robots).contains(&minutes_left)
            {
                continue;
            }
            best = best.max(self.solve(State {
                minutes_left: minutes_left - 1,
                resources: resources.spend(self.blueprint, option).unwrap() + mined_resources,
                robots: robots + option,
                first_minute,
                last_minute,
            }))
        }

        best + new_geodes
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &[Blueprint]) -> i32 {
    //     let input = &parse("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    // Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
    input
        .iter()
        .enumerate()
        .map(|(i, &blueprint)| {
            let mut solver = Solver19 {
                blueprint,
                cache: HashMap::new(),
            };
            let mut best = 0;
            for first_ore in 1..=24 {
                for first_clay in 3..24 {
                    for first_obsidian in 2..first_clay {
                        for first_geode in 1..first_obsidian {
                            for last_ore in 1..=first_ore {
                                for last_clay in 3..=first_clay {
                                    for last_obsidian in 2..=first_obsidian {
                                        for last_geode in 1..=first_geode {
                                            let first_minute = Robots {
                                                ore_robots: first_ore,
                                                clay_robots: first_clay,
                                                obsidian_robots: first_obsidian,
                                                geode_robots: first_geode,
                                            };
                                            let last_minute = Robots {
                                                ore_robots: last_ore,
                                                clay_robots: last_clay,
                                                obsidian_robots: last_obsidian,
                                                geode_robots: last_geode,
                                            };
                                            let sol = solver.solve(State {
                                                minutes_left: 24,
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
                                                first_minute,
                                                last_minute,
                                            });
                                            best = best.max(sol);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let i = i + 1;
            println!(
                "solved blueprint {i} with score {best} and state size {}",
                solver.cache.len()
            );
            i32::try_from(i).unwrap() * best
        })
        .sum()
}