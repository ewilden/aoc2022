use petgraph::{algo, prelude::*};
use std::collections::HashMap;

#[aoc_generator(day15)]
fn parse(inp: &str) -> HashMap<(i32, i32), i32> {
  let mut m = HashMap::new();
  for (r, line) in inp.lines().enumerate() {
    for (c, ch) in line.chars().enumerate() {
      m.insert((r as i32, c as i32), (ch as i32 - '0' as i32));
    }
  }
  m
}

struct GraphData {
  graph: Graph<i32, ()>,
  nodes: HashMap<(i32, i32), NodeIndex>,
}

impl GraphData {
  fn add_edge(&mut self, a: ((i32, i32), i32), b: ((i32, i32), i32)) {
    let (ka, wa) = a;
    let (kb, wb) = b;
    let ia = {
      if let Some(ia) = self.nodes.get(&ka) {
        *ia
      } else {
        let ia = self.graph.add_node(wa);
        self.nodes.insert(ka, ia);
        ia
      }
    };
    let ib = {
      if let Some(ib) = self.nodes.get(&kb) {
        *ib
      } else {
        let ib = self.graph.add_node(wb);
        self.nodes.insert(kb, ib);
        ib
      }
    };
    self.graph.add_edge(ia, ib, ());
    self.graph.add_edge(ib, ia, ());
  }
}

use itertools::Itertools as _;

fn build_graph(inp: &HashMap<(i32, i32), i32>) -> GraphData {
  let mut g = GraphData { graph: Graph::new(), nodes: HashMap::new() };
  for ka in inp.clone().keys() {
    for kb in ((ka.0 - 1)..=(ka.0 + 1)).cartesian_product((ka.1 - 1)..=(ka.1 + 1)) {
      if (ka.0 - kb.0).abs() + (ka.1 - kb.1).abs() == 1 && inp.contains_key(ka) && inp.contains_key(&kb) && ka <= &kb {
        g.add_edge((*ka, inp[ka]), (kb, inp[&kb]));
      }
    }
  }
  g
}

#[aoc(day15, part1)]
fn part1(inp: &HashMap<(i32, i32), i32>) -> i32 {
  let g = build_graph(inp);
  let maxkey = g.nodes.keys().max().unwrap();
  let node_to_dist = algo::dijkstra(&g.graph, g.nodes[&(0, 0)], Some(g.nodes[maxkey]), |edge| {
    let target_node = edge.target();
    g.graph[target_node]
  });
  node_to_dist[&g.nodes[maxkey]]
}

#[aoc(day15, part2)]
fn part2(inp: &HashMap<(i32, i32), i32>) -> i32 {
  let expanded_inp: HashMap<(i32, i32), i32> = {
    let orig_num_rows = inp.keys().map(|(r,_)| r).max().unwrap() + 1;
    let orig_num_cols = inp.keys().map(|(_,c)| c).max().unwrap() + 1;
    (0..(orig_num_rows * 5)).cartesian_product(0..(orig_num_cols * 5)).map(|(r,c)| {
      let orig_risk = inp[&(r % orig_num_rows, c % orig_num_cols)];
      let num_tiles_down = r / orig_num_rows;
      let num_tiles_across = c / orig_num_cols;
      let new_risk = ((orig_risk - 1) + num_tiles_across + num_tiles_down) % 9 + 1;
      ((r, c), new_risk)
    }).collect()
  };

  let g = build_graph(&expanded_inp);
  let maxkey = g.nodes.keys().max().unwrap();
  let node_to_dist = algo::dijkstra(&g.graph, g.nodes[&(0, 0)], Some(g.nodes[maxkey]), |edge| {
    let target_node = edge.target();
    g.graph[target_node]
  });
  node_to_dist[&g.nodes[maxkey]]
}