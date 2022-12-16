use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
struct Valve {
    valve: String,
    flow_rate: i64,
    neighbors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Open(String),
    Move(String),
}

//type RtnType = (Vec<Action>, i64);
type RtnType = i64;

fn release_pipes<'state>(
    graph: &HashMap<String, Valve>,
    current: &str,
    minutes: i64,
    open_state: &'state mut Vec<bool>,
    cache: &'state mut HashMap<(String, Vec<bool>, i64), RtnType>,
) -> RtnType {
    if minutes == 0 {
        //return (Vec::new(), 0);
        return  0;
    }
    if let Some(rtn) = cache.get(&(current.to_owned(), open_state.to_vec(), minutes)) {
        return rtn.clone();
    }
    let freshly_released: i64 = graph
        .iter()
        .zip(open_state.iter())
        .filter(|(_, open)| **open)
        .map(|((_, v), _)| v.flow_rate)
        .sum();
    let cur_idx = graph.iter().position(|(_, n)| n.valve == current).unwrap();
    let before_value = open_state[cur_idx];
    open_state[cur_idx] = true;
    let mut stay_here = release_pipes(graph, current, minutes - 1, open_state, cache);
    open_state[cur_idx] = before_value;

    let mut rtn = graph
        .get(current)
        .unwrap()
        .clone()
        .neighbors
        .iter()
        .map(|n| {
            let rtn = release_pipes(graph, &n.clone(), minutes - 1, open_state, cache);
            //rtn.0.push(Action::Move(n.to_owned()));
            rtn
        })
        //.max_by_key(|(_, max)| *max)
        .max()
        .unwrap();
    if stay_here > rtn {
        //stay_here.0.push(Action::Open(current.to_owned()));
        rtn = stay_here;
    }

    //rtn = (rtn.0, rtn.1 + freshly_released);
    rtn  += freshly_released;
    cache.insert(
        (current.to_owned(), open_state.to_vec(), minutes),
        rtn.clone(),
    );
    rtn
}

fn main() {
    let input = include_str!("../../../input/day16.txt");

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
    let mut open_state = vec![false; graph.len()];
    let mut cache = HashMap::new();

    let part1 = release_pipes(&graph, "AA", 30, &mut open_state, &mut cache);
    dbg!(&part1);
}
