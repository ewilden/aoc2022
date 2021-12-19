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
}

impl CaveGraph {
  fn get_or_create_node(&mut self, name: &String) -> Vec<NodeIndex> {
    let Self { graph, name_to_idx } = self;
    match name_to_idx.get_vec(name).cloned() {
      Some(idx) => idx,
      None => {
        let mut idxes = vec![graph.add_node(name.clone())];
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

fn build_graph(inp: &Vec<(String, String)>) -> CaveGraph {
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
  let mut graph: CaveGraph = CaveGraph { graph, name_to_idx };
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
  let graph = build_graph(inp);
  let CaveGraph { graph, name_to_idx} = graph;
  algo::all_simple_paths::<Vec<_>, _>(&graph, 
    *name_to_idx.get(&"start".to_string()).unwrap(), 
    *name_to_idx.get(&"end".to_string()).unwrap(), 
    0, 
    None).count()
}

#[aoc(day12, part2)]
fn part2(inp: &Vec<(String, String)>) -> usize {
  let small_cave_names = inp.iter().cloned().flat_map(|(a,b)| IntoIterator::into_iter([a,b])).filter(
    |name| name != "start" && name != "end" && !is_upper(name)
  ).unique().collect::<Vec<_>>();

  let mut count = 0;

  count += {
    let graph = build_graph(inp);
    let CaveGraph { graph, name_to_idx } = graph;
    algo::all_simple_paths::<Vec<_>, _>(&graph, 
      *name_to_idx.get(&"start".to_string()).unwrap(), 
      *name_to_idx.get(&"end".to_string()).unwrap(), 
      0, 
      None).count()
  };

  // let (count_no_dup, names) = {
  // };

  for cave_to_duplicate in small_cave_names {
    let dupname = format!("{}duplicate", cave_to_duplicate);
    let amended_inp = inp.iter().cloned().flat_map(|(a,b)| {
      if a == cave_to_duplicate {
        IntoIterator::into_iter(vec![(a,b.clone()), (dupname.clone(),b)])
      } else if b == cave_to_duplicate {
        IntoIterator::into_iter(vec![(a.clone(),b), (a,dupname.clone())])
      } else {
        IntoIterator::into_iter(vec![(a,b)])
      }
    }).collect::<Vec<_>>();
    let graph = build_graph(&amended_inp);
    let CaveGraph { graph, name_to_idx } = graph;
    count += algo::all_simple_paths::<Vec<_>, _>(&graph, 
      *name_to_idx.get(&"start".to_string()).unwrap(), 
      *name_to_idx.get(&"end".to_string()).unwrap(), 
      0, 
      None).filter(|path| {
        let name_path = path.into_iter().map(|idx| &graph[*idx]).cloned().collect::<Vec<_>>();
        let small_name_path = name_path.into_iter().filter(|name| !is_upper(&name)).collect::<Vec<_>>();
        if small_name_path.len() != small_name_path.iter().unique().count() {
          return false
        }
        let namepos = small_name_path.iter().position(|s| s == &cave_to_duplicate);
        let duppos = small_name_path.iter().position(|s| s == &dupname);
        match (namepos, duppos) {
          (None, _) | (_, None) => { return false }
          (Some(namepos), Some(duppos)) => {
            if namepos > duppos {
              return false
            }
          }
        }
        true
      }).count();
  }
  count
}
