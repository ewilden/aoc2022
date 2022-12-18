use std::collections::{BTreeSet, HashMap};

use petgraph::{
    algo::floyd_warshall,
    stable_graph::{DefaultIx, NodeIndex},
    Directed, Graph,
};

#[aoc_generator(day16)]
pub fn parse(input: &str) -> Vec<(String, i32, Vec<String>)> {
    input
        .lines()
        .map(|line| {
            let (valve_desc, tunnels) = line.split_once("; ").unwrap();
            let (valve, flow_rate) = valve_desc.split_once(" has flow rate=").unwrap();
            let valve = valve.strip_prefix("Valve ").unwrap().to_owned();
            let flow_rate = flow_rate.parse::<i32>().unwrap();
            let tunnels = tunnels
                .strip_prefix("tunnels lead to valves ")
                .unwrap_or_else(|| tunnels.strip_prefix("tunnel leads to valve ").unwrap());
            let tunnels = tunnels
                .split(", ")
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();
            (valve, flow_rate, tunnels)
        })
        .collect()
}

pub struct State {
    pub graph: Graph<String, i32, Directed>,
    pub name_to_node: HashMap<String, NodeIndex<DefaultIx>>,
    pub name_to_flow_rate: HashMap<String, i32>,
    pub from_to_dists: HashMap<(NodeIndex<DefaultIx>, NodeIndex<DefaultIx>), i32>,
}

const USE_CACHE: bool = false;

impl State {
    fn utility_upper_bound(
        &self,
        minutes_remaining: i32,
        node: &str,
        open_nodes: &BTreeSet<String>,
    ) -> i32 {
        if minutes_remaining <= 1 {
            return 0;
        }

        let util_from_me = if open_nodes.contains(node) {
            0
        } else {
            (minutes_remaining - 1) * self.name_to_flow_rate.get(node).copied().unwrap_or(0)
        };

        let mut sum = util_from_me;
        for (other, flow_rate) in &self.name_to_flow_rate {
            if open_nodes.contains(other) {
                continue;
            }
            sum += self
                .from_to_dists
                .get(&(self.name_to_node[node], self.name_to_node[other]))
                .map(|&dist| (minutes_remaining - dist - 1).max(0) * flow_rate)
                .unwrap_or(0);
        }
        sum
    }

    fn actual_utility_cached(
        &self,
        minutes_remaining: i32,
        node: &str,
        open_nodes: &BTreeSet<String>,
        came_from: Option<&str>,
        cache: &mut HashMap<(i32, String, BTreeSet<String>), i32>,
    ) -> i32 {
        let key = (minutes_remaining, node.to_owned(), open_nodes.clone());
        if !cache.contains_key(&key) {
            let value = self.actual_utility(minutes_remaining, node, open_nodes, came_from, cache);
            if USE_CACHE {
                cache.insert(key.clone(), value);
            }
            return value;
        } else {
            println!("cache hit for {minutes_remaining} + {node}");
        }
        cache[&key]
    }

    fn actual_utility(
        &self,
        minutes_remaining: i32,
        node: &str,
        open_nodes: &BTreeSet<String>,
        came_from: Option<&str>,
        cache: &mut HashMap<(i32, String, BTreeSet<String>), i32>,
    ) -> i32 {
        if minutes_remaining <= 1 {
            return 0;
        }

        let upper_bounds_without_opening_this_one = self
            .graph
            .neighbors(self.name_to_node[node])
            .map(|other_node_index| {
                let other_name = self.graph.node_weight(other_node_index).unwrap();
                (
                    self.utility_upper_bound(minutes_remaining - 1, other_name, &open_nodes),
                    false,
                    other_name,
                )
            })
            .collect::<Vec<_>>();

        let open_nodes_including_this_one = {
            let mut open_nodes = open_nodes.clone();
            open_nodes.insert(node.to_owned());
            open_nodes
        };

        let utility_from_opening_this_one =
            (minutes_remaining - 1) * self.name_to_flow_rate.get(node).copied().unwrap_or(0);

        let upper_bounds_while_opening_this_one = {
            if open_nodes.contains(node) || utility_from_opening_this_one == 0 {
                Vec::new()
            } else {
                self.graph
                    .neighbors(self.name_to_node[node])
                    .map(|other_node_index| {
                        let other_name = self.graph.node_weight(other_node_index).unwrap();
                        (
                            utility_from_opening_this_one
                                + self.utility_upper_bound(
                                    minutes_remaining - 2,
                                    other_name,
                                    &open_nodes_including_this_one,
                                ),
                            true,
                            other_name,
                        )
                    })
                    .collect::<Vec<_>>()
            }
        };

        let mut upper_bounds = upper_bounds_without_opening_this_one;
        upper_bounds.extend(upper_bounds_while_opening_this_one.into_iter());
        upper_bounds.sort_unstable_by_key(|&(bound, _, _)| -bound);
        let upper_bounds = upper_bounds;

        let mut best = 0;

        for (upper_bound, open_this_one, other_name) in upper_bounds {
            if upper_bound <= best {
                continue;
            }

            if !open_this_one && came_from == Some(other_name) {
                continue;
            }

            let potential = {
                let mut minutes_remaining = minutes_remaining;
                let mut score = 0;
                let mut open_nodes = open_nodes;

                if open_this_one {
                    minutes_remaining -= 1;
                    score += minutes_remaining * self.name_to_flow_rate[node];
                    open_nodes = &open_nodes_including_this_one;
                }

                score
                    + self.actual_utility(
                        minutes_remaining - 1,
                        other_name,
                        open_nodes,
                        Some(node),
                        cache,
                    )
            };
            if potential > best {
                // println!("new best = {potential}");
                best = potential;
            }
        }
        best
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &[(String, i32, Vec<String>)]) -> i32 {
    let mut graph: Graph<String, i32, Directed> = Graph::new();
    let mut name_to_node: HashMap<String, NodeIndex<DefaultIx>> = HashMap::new();
    let mut name_to_flow_rate: HashMap<String, i32> = HashMap::new();
    for (valve, flow_rate, tunnels) in input {
        if *flow_rate > 0 {
            name_to_flow_rate.insert(valve.to_owned(), *flow_rate);
        }
        let node = *name_to_node
            .entry(valve.to_owned())
            .or_insert_with(|| graph.add_node(valve.to_owned()));

        for tunnel in tunnels {
            let dest = *name_to_node
                .entry(tunnel.to_owned())
                .or_insert_with(|| graph.add_node(tunnel.to_owned()));
            graph.add_edge(node, dest, 1);
        }
    }
    let from_to_dists = floyd_warshall(&graph, |_| 1i32).unwrap();
    let state = State {
        graph,
        name_to_node,
        name_to_flow_rate,
        from_to_dists,
    };

    state.actual_utility_cached(30, "AA", &BTreeSet::new(), None, &mut HashMap::new())
}
