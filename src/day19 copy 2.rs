use std::ops::{Add, AddAssign, Mul, Neg};

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
        clay += -(obs_clay_cost * obsidian_robots);
        obsidian += -(geode_obs_cost * geode_robots);
        [ore.0, clay.0, obsidian.0]
            .into_iter()
            .all(|x| x >= 0)
            .then_some(Resources {
                ore,
                clay,
                obsidian,
            })
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
            let i = i + 1;
            let mut best = 0;
            let mut best_strategy = ((0, 0), 0);
            for robot_counts in (0..=15).cartesian_product(0..=15).cartesian_product(0..=15) {
                let (
                    (mut ore_robots_to_make, mut clay_robots_to_make),
                    mut obsidian_robots_to_make,
                ) = robot_counts;

                let mut robots = Robots {
                    ore_robots: 1,
                    ..Default::default()
                };

                let mut resources = Resources {
                    ore: Ore(0),
                    clay: Clay(0),
                    obsidian: Obsidian(0),
                };

                let mut geodes = 0;

                for _minute in 1..=24 {
                    let (new_resources, new_geodes) = robots.mine();
                    geodes += new_geodes;

                    let mut did_build = false;
                    while !did_build && ore_robots_to_make > 0 {
                        if let Some(resources_after_spending) = resources.spend(
                            blueprint,
                            Robots {
                                ore_robots: 1,
                                clay_robots: 0,
                                obsidian_robots: 0,
                                geode_robots: 0,
                            },
                        ) {
                            resources = resources_after_spending;
                            ore_robots_to_make -= 1;
                            robots.ore_robots += 1;
                            did_build = true;
                        } else {
                            break;
                        }
                    }

                    if ore_robots_to_make == 0 {
                        while !did_build && clay_robots_to_make > 0 {
                            if let Some(resources_after_spending) = resources.spend(
                                blueprint,
                                Robots {
                                    ore_robots: 0,
                                    clay_robots: 1,
                                    obsidian_robots: 0,
                                    geode_robots: 0,
                                },
                            ) {
                                resources = resources_after_spending;
                                clay_robots_to_make -= 1;
                                robots.clay_robots += 1;
                                did_build = true;
                            } else {
                                break;
                            }
                        }

                        if clay_robots_to_make == 0 {
                            while !did_build && obsidian_robots_to_make > 0 {
                                if let Some(resources_after_spending) = resources.spend(
                                    blueprint,
                                    Robots {
                                        ore_robots: 0,
                                        clay_robots: 0,
                                        obsidian_robots: 1,
                                        geode_robots: 0,
                                    },
                                ) {
                                    resources = resources_after_spending;
                                    obsidian_robots_to_make -= 1;
                                    robots.obsidian_robots += 1;
                                    did_build = true;
                                } else {
                                    break;
                                }
                            }

                            if obsidian_robots_to_make == 0 {
                                while !did_build {
                                    if let Some(resources_after_spending) = resources.spend(
                                        blueprint,
                                        Robots {
                                            ore_robots: 0,
                                            clay_robots: 0,
                                            obsidian_robots: 0,
                                            geode_robots: 1,
                                        },
                                    ) {
                                        resources = resources_after_spending;
                                        robots.geode_robots += 1;
                                        did_build = true;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    resources = resources + new_resources;
                }
                if geodes > best {
                    best = geodes;
                    best_strategy = robot_counts;
                }
            }

            println!("blueprint {i} can score {best} using {best_strategy:?}");
            i32::try_from(i).unwrap() * best
        })
        .sum()
}
