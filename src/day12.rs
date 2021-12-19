use petgraph::{algo, prelude::*};
use multimap::MultiMap;
use counter::Counter;
use itertools::Itertools as _;

#[aoc_generator(day12)]
fn parse(inp: &str) -> Vec<(String, String)> {
  inp.lines().map(|s| {
    let mut ss = s.split('-');
    (ss.next().unwrap().to_string(), ss.next().unwrap().to_string())
  }).collect()
}

fn is_upper(s: &String) -> bool {
  s == &s.to_uppercase()
}

struct CaveGraph {
  graph: Graph<String, i32, Undirected>,
  name_to_idx: MultiMap<String, NodeIndex>,
  cave_to_duplicate: Option<String>,
}

impl CaveGraph {
  fn get_or_create_node(&mut self, name: &String) -> Vec<NodeIndex> {
    let Self { graph, name_to_idx, cave_to_duplicate } = self;
    match name_to_idx.get_vec(name).cloned() {
      Some(idx) => idx,
      None => {
        let mut idxes = vec![graph.add_node(name.clone())];
        if Some(name) == cave_to_duplicate.as_ref() {
          idxes.push(graph.add_node(name.clone()));
        }
        name_to_idx.insert_many(name.to_string(), idxes.clone());
        idxes
      }
    }
  }

  fn extend_edge(&mut self, l: &String, r: &String){
    let lidxes = self.get_or_create_node(l);
    let ridxes = self.get_or_create_node(r);
    self.graph.extend_with_edges(lidxes.into_iter().cartesian_product(ridxes.into_iter()));
  }
}

fn build_graph(inp: &Vec<(String, String)>, cave_to_duplicate: Option<String>) -> CaveGraph {
  let mut big_edges: MultiMap<String, String> = MultiMap::new();
  for (l,r) in inp {
    if is_upper(l) {
      big_edges.insert(l.clone(), r.clone());
      if is_upper(r) {
        panic!()
      }
    } else if is_upper(r) {
      big_edges.insert(r.clone(), l.clone());
    }
  }

  let graph: Graph<String, i32, Undirected> = Graph::new_undirected();
  let name_to_idx: MultiMap<String, NodeIndex> = MultiMap::new();
  let mut graph: CaveGraph = CaveGraph { graph, name_to_idx, cave_to_duplicate };
  for (l,r) in inp.iter().filter(|(l,r)| !is_upper(l) && !is_upper(r)).cloned() {
    graph.extend_edge(&l,&r);
  }
  
  for (big, smalls) in big_edges.into_iter() {
    for i in 0..smalls.len() {
      for j in i..smalls.len() {
        graph.extend_edge(&smalls[i], &smalls[j]);
      }
    }
  }
  graph
}

#[aoc(day12, part1)]
fn part1(inp: &Vec<(String, String)>) -> usize {
  let graph = build_graph(inp, None);
  let CaveGraph { graph, name_to_idx, cave_to_duplicate: _ } = graph;
  algo::all_simple_paths::<Vec<_>, _>(&graph, 
    *name_to_idx.get(&"start".to_string()).unwrap(), 
    *name_to_idx.get(&"end".to_string()).unwrap(), 
    0, 
    None).count()
}

#[aoc(day12, part2)]
fn part2(inp: &Vec<(String, String)>) -> usize {
  let (count_no_dup, names) = {
    let graph = build_graph(inp, None);
    let CaveGraph { graph, name_to_idx, cave_to_duplicate: _ } = graph;
    let names = name_to_idx.keys().cloned().collect::<Vec<_>>();
    (algo::all_simple_paths::<Vec<_>, _>(&graph, 
      *name_to_idx.get(&"start".to_string()).unwrap(), 
      *name_to_idx.get(&"end".to_string()).unwrap(), 
      0, 
      None).count(), names)
  };
  count_no_dup + names.into_iter().filter(|name| name != "start" && name != "end" && !is_upper(name)).map(|cave_to_duplicate| {
    let graph = build_graph(inp, Some(cave_to_duplicate));
    let CaveGraph { graph, name_to_idx, cave_to_duplicate } = graph;
    let cave_to_duplicate = cave_to_duplicate.unwrap();
    let dupped_count = algo::all_simple_paths::<Vec<_>, _>(&graph, 
      *name_to_idx.get(&"start".to_string()).unwrap(), 
      *name_to_idx.get(&"end".to_string()).unwrap(), 
      0, 
      None).filter(|path| {
        let name_counts = path.iter()
          .map(|idx| &graph[*idx])
          .filter(|name| !is_upper(name)).collect::<Counter<_>>();
        if name_counts[&cave_to_duplicate] != 2 {
          return false
        }
        name_counts.into_iter().filter(|(n,c)| n != &&&cave_to_duplicate && c > &&1).count() == 0
      }).count();
      // halve the counts (since the route finder will visit duplicate cave in each order but to us it's the same)
      if dupped_count % 2 != 0 {
        panic!();
      }
      dupped_count / 2
  }).sum::<usize>()
}
// guessed 401407, wrong
// 378004, also wrong
// 191398, also wrong