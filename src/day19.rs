use std::{
    collections::{BTreeSet, HashMap},
    ops::{Add, AddAssign, Mul, Neg},
};

use itertools::Itertools;

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

    fn options(self, blueprint: Blueprint) -> impl Iterator<Item = RobotType> {
        [
            RobotType::Ore,
            RobotType::Clay,
            RobotType::Obsidian,
            RobotType::Geode,
        ]
        .into_iter()
        .filter(move |&t| self.spend(blueprint, t.into()).is_some())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl From<RobotType> for Robots {
    fn from(t: RobotType) -> Self {
        match t {
            RobotType::Ore => Robots {
                ore_robots: 1,
                ..Default::default()
            },
            RobotType::Clay => Robots {
                clay_robots: 1,
                ..Default::default()
            },
            RobotType::Obsidian => Robots {
                obsidian_robots: 1,
                ..Default::default()
            },
            RobotType::Geode => Robots {
                geode_robots: 1,
                ..Default::default()
            },
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct State {
    minutes_left: i32,
    resources: Resources,
    robots: Robots,
    not_allowed_next: BTreeSet<RobotType>,
}

struct Solver19 {
    blueprint: Blueprint,
    cache: HashMap<State, i32>,
}

impl Solver19 {
    fn solve(&mut self, state: State) -> i32 {
        if !self.cache.contains_key(&state) {
            let result = self.solve_impl(state.clone());
            if state.minutes_left < 6 && state.minutes_left > 0 {
                self.cache.insert(state, result);
            }
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
            not_allowed_next,
        } = state.clone();
        if minutes_left == 0 {
            return 0;
        }

        let robot_options = resources
            .options(self.blueprint)
            .filter(|&ty| match ty {
                RobotType::Ore => {
                    Ore(robots.ore_robots)
                        < self
                            .blueprint
                            .clay_robot_cost
                            .max(self.blueprint.obsidian_robot_cost.0)
                            .max(self.blueprint.geode_robot_cost.0)
                }
                RobotType::Clay => Clay(robots.clay_robots) < self.blueprint.obsidian_robot_cost.1,
                RobotType::Obsidian => {
                    Obsidian(robots.obsidian_robots) < self.blueprint.geode_robot_cost.1
                }
                RobotType::Geode => true,
            })
            .collect_vec();
        let (mined_resources, new_geodes) = robots.mine();

        let mut best = 0;
        for option in std::iter::once(None).chain(robot_options.iter().map(Some)) {
            let option = option.copied();
            if let Some(option) = option {
                if not_allowed_next.contains(&option) {
                    continue;
                }
            }

            best = best.max(
                self.solve(State {
                    minutes_left: minutes_left - 1,
                    resources: resources
                        .spend(self.blueprint, option.map(Robots::from).unwrap_or_default())
                        .unwrap()
                        + mined_resources,
                    robots: robots + option.map(Robots::from).unwrap_or_default(),
                    not_allowed_next: {
                        if let Some(_option) = option {
                            BTreeSet::new()
                        } else {
                            robot_options.iter().copied().collect()
                        }
                    },
                }),
            )
        }

        best + new_geodes
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &[Blueprint]) -> i32 {
    // let input = &parse("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    // Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
    input
        .iter()
        .enumerate()
        .map(|(i, &blueprint)| {
            let mut solver = Solver19 {
                blueprint,
                cache: HashMap::new(),
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
                not_allowed_next: BTreeSet::new(),
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

#[aoc(day19, part2)]
pub fn part2(input: &[Blueprint]) -> i32 {
    // let input = &parse("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    // Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
    input
        .iter()
        .take(3)
        .map(|&blueprint| {
            let mut solver = Solver19 {
                blueprint,
                cache: HashMap::new(),
            };
            let sol = solver.solve(State {
                minutes_left: 32,
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
                not_allowed_next: BTreeSet::new(),
            });

            println!(
                "solved blueprint with score {sol} and state size {}",
                solver.cache.len()
            );
            sol
        })
        .product()
}
