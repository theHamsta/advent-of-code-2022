use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::{dot::Dot, visit::EdgeRef, Graph};
use regex::Regex;

#[derive(Debug, Clone)]
struct Valve {
    valve: String,
    flow_rate: i64,
    neighbors: Vec<String>,
}

//#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
//enum Action {
//Open(String),
//Move(String),
//}

//type RtnType = (Vec<Action>, i64);
type RtnType = (Vec<u32>, u64);

#[derive(Debug, Clone)]
struct Node {
    _name: String,
    flow_rate: u64,
}

fn main() {
    let input = include_str!("../../../input/day16_test.txt");

    let regex =
        Regex::new(r"Valve\s(\w\w) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)")
            .unwrap();
    let mut input: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            dbg!(&l);
            regex
                .captures(l)
                .map(|c| Valve {
                    valve: c[1].to_owned(),
                    flow_rate: c[2].parse().unwrap(),
                    neighbors: c[3].split(", ").map(|s| s.to_owned()).collect(),
                })
                .unwrap()
        })
        .collect();

    let graph: HashMap<_, Valve> = input.drain(..).map(|v| (v.valve.clone(), v)).collect();

    let mut petgraph = Graph::<Node, u64>::new();

    let node_list: HashMap<_, _> = graph
        .iter()
        .map(|(k, node)| {
            (
                k.clone(),
                petgraph.add_node(Node {
                    _name: node.valve.to_owned(),
                    flow_rate: node.flow_rate as u64,
                }),
            )
        })
        .collect();

    for (my_name, node) in graph.iter() {
        let my_idx = node_list[my_name];
        for n in node.neighbors.iter() {
            petgraph.add_edge(my_idx, node_list[n], 1);
        }
    }

    let aa_idx = node_list["AA"];
    let cool_nodes: HashSet<_> = petgraph
        .node_indices()
        .filter(|idx| petgraph.node_weight(*idx).unwrap().flow_rate > 0 || node_list["AA"] == *idx)
        .collect();

    let mut to_add = Vec::new();

    //for start in cool_nodes.iter() {
    //let paths = dijkstra(&petgraph, *start, None, |e| *petgraph.edge_weight(e.id()).unwrap());
    //for (goal, distance) in paths {
    //if goal != aa_idx && goal != *start && cool_nodes.contains(&goal) {
    //to_add.push((*start, goal, distance));
    //}
    //}
    //}
    //
    for n in cool_nodes {
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back((n, 0));
        visited.insert(n);
        while let Some((between, hops)) = to_visit.pop_front() {
            visited.insert(between);
            for neighbor in petgraph.neighbors(between) {
                if !visited.contains(&neighbor) {
                    if petgraph.node_weight(neighbor).unwrap().flow_rate == 0 {
                        to_visit.push_back((neighbor, hops + 1));
                    } else {
                        to_add.push((n, neighbor, hops + 1));
                    }
                }
            }
        }
    }
    for (a, b, w) in to_add {
        petgraph.update_edge(a, b, w);
    }

    petgraph.retain_edges(|graph, e| {
        let (a, b) = graph.edge_endpoints(e).unwrap();
        (graph.node_weight(a).unwrap().flow_rate > 0 && graph.node_weight(b).unwrap().flow_rate > 0)
            || a == aa_idx
    });
    std::fs::write_file();
    println!("{:?}", Dot::new(&petgraph));
     let mut file = File::create("foo.dot")?;
    file.write_all(b"Hello, world!")?;


    let mut open_state = vec![false; petgraph.raw_nodes().len()];
    let mut cache = HashMap::new();
    let mut part1 = release_pipes_graph(
        &petgraph,
        node_list["AA"].index() as u32,
        30,
        &mut open_state,
        &mut cache,
    );

    dbg!(&part1.1);

    while let Some(item) = part1.0.pop() {
        let name = node_list
            .iter()
            .find(|(_, idx)| item == idx.index() as u32)
            .unwrap()
            .0;
        println!("{name}");
    }
}

fn release_pipes_graph(
    graph: &Graph<Node, u64>,
    current: u32,
    minutes: i64,
    open_state: &mut Vec<bool>,
    cache: &mut HashMap<(u32, Vec<bool>, i64), RtnType>,
) -> RtnType {
    if minutes <= 0 {
        return (Vec::new(), 0);
    }
    if let Some(rtn) = cache.get(&(current, open_state.to_vec(), minutes)) {
        return rtn.clone();
    }
    let freshly_released: u64 = graph
        .raw_nodes()
        .iter()
        .enumerate()
        .filter(|(idx, _)| open_state[*idx])
        .map(|(_, n)| n.weight.flow_rate)
        .sum();
    let before_value = open_state[current as usize];
    open_state[current as usize] = true;
    let mut stay_here = release_pipes_graph(graph, current, minutes - 1, open_state, cache);
    stay_here.0.push(current);
    stay_here.1 += freshly_released;
    open_state[current as usize] = before_value;
    let mut rtn = graph
        .edges(current.into())
        //.filter(|e| !visited.borrow()[e.target().index()])
        .map(|e| {
            let mut rtn = release_pipes_graph(
                graph,
                e.target().index() as u32,
                minutes - *e.weight() as i64,
                open_state,
                cache,
            );
            rtn.0.push(current);
            rtn.1 += (minutes - *e.weight() as i64).clamp(0, *e.weight() as i64) as u64
                * freshly_released;
            rtn
        })
        .max_by_key(|k| k.1)
        .unwrap_or_else(|| (Vec::new(), 0));
    if stay_here.1 > rtn.1 {
        rtn = stay_here;
    }

    cache.insert(
        (current.to_owned(), open_state.to_vec(), minutes),
        rtn.clone(),
    );
    rtn
}
