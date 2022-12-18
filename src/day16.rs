use std::collections::{BTreeSet, HashMap};

use petgraph::{
    algo::{bellman_ford, floyd_warshall},
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

#[aoc(day16, part1)]
pub fn part1(input: &[(String, i32, Vec<String>)]) -> f64 {
    let mut graph: Graph<String, i32, Directed> = Graph::new();
    let mut name_to_node: HashMap<String, NodeIndex<DefaultIx>> = HashMap::new();
    let mut name_to_flow_rate: HashMap<String, i32> = HashMap::new();
    for (valve, flow_rate, tunnels) in input {
        // println!("{valve} -> {{ {} }} [dir=none]", tunnels.join(", "));
        if *flow_rate > 0 {
            name_to_flow_rate.insert(valve.to_owned(), *flow_rate);
            // println!("{valve} [shape=Mdiamond]");
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

    // reduce to just the nodes with positive flow rate, plus AA.

    let mut pared_name_to_node: HashMap<String, NodeIndex<DefaultIx>> = HashMap::new();
    let mut pared_graph: Graph<String, (String, i32), Directed> = Graph::new();

    for ((a_ind, b_ind), dist) in from_to_dists {
        if dist < 30 {
            let a_name = graph.node_weight(a_ind).unwrap();
            let b_name = graph.node_weight(b_ind).unwrap();
            if !(name_to_flow_rate.contains_key(a_name) || a_name == "AA")
                || !(name_to_flow_rate.contains_key(b_name)) /* disallow AA on rhs */
                || a_name == b_name
            {
                continue;
            }
            let a_pared = *pared_name_to_node
                .entry(a_name.to_owned())
                .or_insert_with(|| pared_graph.add_node(a_name.to_owned()));
            let b_pared = *pared_name_to_node
                .entry(b_name.to_owned())
                .or_insert_with(|| pared_graph.add_node(b_name.to_owned()));
            pared_graph.add_edge(
                a_pared,
                b_pared,
                (b_name.to_owned(), dist /* FOR OPENING */ + 1),
            );
            if a_name < b_name {
                println!(
                    "{a_name}_{} -> {b_name}_{} [dir=none]",
                    name_to_flow_rate.get(a_name).unwrap_or(&0),
                    name_to_flow_rate.get(b_name).unwrap_or(&0)
                );
            }
        }
    }
    let graph = pared_graph;
    let name_to_node = pared_name_to_node;

    // Now is it time to make the synthetic graph?
    let mut name_time_to_node_index: HashMap<
        (String, i32, BTreeSet<String>),
        NodeIndex<DefaultIx>,
    > = HashMap::new();
    let start_node = (String::from("AA"), 0, BTreeSet::new());
    let mut nodes_just_added = vec![start_node.clone()];
    let mut time_graph: Graph<(String, i32, BTreeSet<String>), f64, Directed> =
        Graph::with_capacity(16 * 30, 2);
    let start_index = *name_time_to_node_index
        .entry(start_node.clone())
        .or_insert_with(|| time_graph.add_node(start_node.clone()));

    let end_index = time_graph.add_node(("END".to_owned(), 30, BTreeSet::new()));

    loop {
        println!("Graph has {} nodes", time_graph.node_count());
        if nodes_just_added.is_empty() {
            // We're done building the graph.
            break;
        }

        for node in std::mem::replace(&mut nodes_just_added, Vec::new()).into_iter() {
            let node_index = *name_time_to_node_index
                .entry(node.clone())
                .or_insert_with(|| time_graph.add_node(node.clone()));
            let (name, time, visited) = &node;
            let visited = {
                let mut visited = visited.clone();
                visited.insert(name.clone());
                visited
            };
            for edge in graph.edges(name_to_node[name]) {
                let (dest, dist) = edge.weight();
                if visited.contains(dest) {
                    continue;
                }
                let next_time = *time + *dist;
                if next_time > 30 {
                    continue;
                }
                // We haven't visited this yet, and we can get to it in time, so let's add it to the graph.
                let dest_node = (dest.to_owned(), next_time, visited.clone());
                let dest_index = *name_time_to_node_index
                    .entry(dest_node.clone())
                    .or_insert_with(|| time_graph.add_node(dest_node.clone()));
                nodes_just_added.push(dest_node.clone());
                time_graph.add_edge(
                    node_index,
                    dest_index,
                    f64::from(-(30 - next_time) * name_to_flow_rate[dest]),
                );
                time_graph.add_edge(node_index, end_index, 0.0);
            }
        }
    }

    let paths = bellman_ford(&time_graph, start_index).unwrap();
    paths.distances[end_index.index()]
    // bellman_ford(time_graph, source)

    // Now that we have this complete graph with extra edges representing
    // traveling without opening anything, maybe it makes it easier
    // to make the temporal paths thing work, or in general
    // refuse to revisit nodes.
    //
    // Now it's simply about picking the right order to visit in.
    // Unfortunately, 15! is a big number. How do we prune it?
    //
    // When we're comparing two paths P, Q that both start with A and end with Z,
    // path P dominates path Q if all of the following hold:
    //   P's total cost <= Q's total cost
    //   P's score >= Q's score
    //   P visits a subset of Q
    //
    // What are other dominance properties?
    // Is there any way to say that node A should always precede node B?
    // If A -> B -> C -> D -> E is better than A -> D -> C -> B -> E,
    // does that imply A -> C -> D -> B -> E is better than A -> D -> C -> B -> E?
    //
    // Is there a way to say that going to X next is worse than never going to X at all?

    // println!("{:?}", graph.edge_weights().max());
}
