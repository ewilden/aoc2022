use std::collections::HashMap;

use petgraph::{algo::dijkstra, Directed, Graph};

#[derive(Clone, Copy)]
pub enum Spot {
    Start,
    End,
    Other(i32),
}

impl Spot {
    pub fn elevation(&self) -> i32 {
        match self {
            Spot::Start => 0,
            Spot::End => ('z' as i32) - ('a' as i32),
            Spot::Other(n) => *n,
        }
    }

    pub fn can_go_to(&self, other: Spot) -> bool {
        self.elevation() + 1 >= other.elevation()
    }
}

#[aoc_generator(day12)]
pub fn parse(input: &str) -> Vec<Vec<Spot>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Spot::Start,
                    'E' => Spot::End,
                    c => Spot::Other(c as i32 - 'a' as i32),
                })
                .collect()
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &Vec<Vec<Spot>>) -> i32 {
    let mut nodes = HashMap::new();
    let mut graph: Graph<Spot, (), Directed> = Graph::new();
    let mut start_loc = None;
    let mut end_loc = None;
    for r in 0..input.len() {
        for c in 0..input[0].len() {
            let me_spot = input[r][c];
            match me_spot {
                Spot::Start => {
                    start_loc = Some((r, c));
                }
                Spot::End => {
                    end_loc = Some((r, c));
                }
                _ => (),
            };
            let me = *nodes.entry((r, c)).or_insert(graph.add_node(me_spot));
            for (other_r, other_c) in [(r, c + 1), (r + 1, c)]
                .into_iter()
                .filter(|(other_r, other_c)| *other_r < input.len() && *other_c < input[0].len())
            {
                let you_spot = input[other_r][other_c];
                let you = *nodes
                    .entry((other_r, other_c))
                    .or_insert(graph.add_node(you_spot));
                if me_spot.can_go_to(you_spot) {
                    graph.add_edge(me, you, ());
                }
                if you_spot.can_go_to(input[r][c]) {
                    graph.add_edge(you, me, ());
                }
            }
        }
    }
    let start_loc = start_loc.unwrap();
    let end_loc = end_loc.unwrap();
    let graph = graph;
    let nodes = nodes;
    let dijkstra_outcome = dijkstra(&graph, nodes[&start_loc], Some(nodes[&end_loc]), |_| 1i32);
    dijkstra_outcome[&nodes[&end_loc]]
}

#[aoc(day12, part2)]
pub fn part2(input: &Vec<Vec<Spot>>) -> i32 {
    let mut nodes = HashMap::new();
    let mut graph: Graph<Spot, (), Directed> = Graph::new();
    let mut end_loc = None;
    for r in 0..input.len() {
        for c in 0..input[0].len() {
            let me_spot = input[r][c];
            match me_spot {
                Spot::Start => {}
                Spot::End => {
                    end_loc = Some((r, c));
                }
                _ => (),
            };
            let me = *nodes.entry((r, c)).or_insert(graph.add_node(me_spot));
            for (other_r, other_c) in [(r, c + 1), (r + 1, c)]
                .into_iter()
                .filter(|(other_r, other_c)| *other_r < input.len() && *other_c < input[0].len())
            {
                let you_spot = input[other_r][other_c];
                let you = *nodes
                    .entry((other_r, other_c))
                    .or_insert(graph.add_node(you_spot));
                if me_spot.can_go_to(you_spot) {
                    graph.add_edge(me, you, ());
                }
                if you_spot.can_go_to(input[r][c]) {
                    graph.add_edge(you, me, ());
                }
            }
        }
    }
    let end_loc = end_loc.unwrap();
    graph.reverse();
    let graph = graph;
    let nodes = nodes;
    let dijkstra_outcome = dijkstra(&graph, nodes[&end_loc], None, |_| 1i32);
    nodes
        .iter()
        .filter_map(|((r, c), node_index)| match input[*r][*c] {
            Spot::Other(0) => dijkstra_outcome.get(node_index).copied(),
            _ => None,
        })
        .min()
        .unwrap()
}
