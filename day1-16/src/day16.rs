use crate::helpers::read_lines;
use core::iter::once;
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, Hash)]
struct Valve<S> {
    rate: usize,
    name: S,
    tunnels: Vec<String>,
}

#[derive(PartialEq, Eq, Debug)]
struct State<T> {
    edge: T,
    cost: usize,
}

impl<S: Eq> PartialEq for Valve<S> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// `PartialOrd` needs to be implemented as well.
impl<T: PartialEq> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl<T: Eq> Ord for State<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub fn find_path<T: PartialEq + Hash + Eq + Clone + Debug>(
    start: &T,
    adj_list: &HashMap<T, Vec<T>>,
) -> HashMap<T, usize> {
    let mut dist: HashMap<_, _> = adj_list
        .clone()
        .into_iter()
        .map(|(k, _)| (k, usize::MAX))
        .collect();

    let mut heap = BinaryHeap::new();

    dist.insert(start.clone(), 0);

    heap.push(State {
        cost: 0,
        edge: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, edge }) = heap.pop() {
        // println!("get: {:?} cost {:?}", edge, cost);
        // Alternatively we could have continued to find all shortest paths
        // if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[edge] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for c_edge in &adj_list[edge] {
            let next = State {
                cost: cost + 1,
                edge: c_edge,
            };
            // println!("children: {:?} : {:?}", next, dist[next.edge]);

            // If so, add it to the frontier and continue
            if next.cost < dist[next.edge] {
                dist.insert(next.edge.clone(), next.cost);
                heap.push(next);
            }
        }
    }

    dist
}

pub fn find_best(
    start: &usize,
    time: usize,
    gain: usize,
    released: u64,
    t_max: &usize,
    rates: &HashMap<usize, usize>,
    distances: &HashMap<usize, HashMap<usize, usize>>,
    mut cache: &mut HashMap<(usize, u64, usize), usize>,
) -> usize {
    if let Some(answer) = cache.get(&(time, released, *start)) {
        return *answer;
    }

    let rate = rates[&start];
    // println!("  Releasing for {}", rate);

    let updated_release = released | (1_u64 << *start);
    // println!("  {:?}: {:b} -> {:b}", start, released, updated_release);
    let expected_gain = gain + (t_max - time) * rate;

    let time = time + 1;
    // println!("[{:?}] valve: {:?} max:{:?}", time, start, t_max);

    if time == *t_max {
        expected_gain
    } else {
        let closed_valves = rates
            .iter()
            .filter(|(e, v)| (v > &&0) && (updated_release & (1 << **e)) == 0)
            .map(|(e, _)| e)
            .collect_vec();

        // println!("  closed: {:?}", closed_valves);
        let mut max_gain = expected_gain;

        for valve in closed_valves {
            // println!("next: {:?}:{:?} on {:?}", start, valve, distances);
            distances.get(&start).expect(&format!("no: {:?}", start));
            let dist = distances[start][&valve];
            // println!(
            //     "  evaluate: {:?}->{:?}: dist:{:?} rel {:b}",
            //     start, valve, dist, updated_release
            // );

            if (time + dist) <= *t_max {
                let v = find_best(
                    valve,
                    time + dist,
                    expected_gain,
                    updated_release,
                    t_max,
                    rates,
                    distances,
                    &mut cache,
                );

                if max_gain < v {
                    // println!("  new best: [{:?}] {:?}={:?}", time, valve, v);
                    max_gain = v;
                }
            }
        }

        cache.insert((time, released, *start), max_gain);
        max_gain
    }
}

pub fn day16() {
    let rrate = Regex::new(r"(?:\d+)").unwrap();
    let rnodes = Regex::new(r"([A-Z][A-Z])").unwrap();

    if let Ok(lines) = read_lines("./inputs/input-d16.txt") {
        let lines = lines.filter_map(|l| l.ok()).collect_vec();

        let mapping_s2i = lines
            .clone()
            .iter()
            .enumerate()
            .map(|(i, l)| {
                let mut iter_nodes = rnodes.find_iter(&l).map(|m| String::from(m.as_str()));
                let node = iter_nodes.next().unwrap();
                // (Valve {rate: rate as usize, name: node, tunnels: nodes.clone()}, nodes)
                (node, i as usize)
            })
            .collect::<HashMap<String, usize>>();

        let mapping_i2s = lines
            .clone()
            .iter()
            .enumerate()
            .map(|(i, l)| {
                let mut iter_nodes = rnodes.find_iter(&l).map(|m| String::from(m.as_str()));
                let node = iter_nodes.next().unwrap();
                // (Valve {rate: rate as usize, name: node, tunnels: nodes.clone()}, nodes)
                (i as usize, node)
            })
            .collect::<HashMap<usize, String>>();

        let nodes = lines
            .clone()
            .iter()
            .map(|l| {
                let rate = rrate
                    .find_iter(&l)
                    .find_map(|s| s.as_str().parse::<i32>().ok())
                    .unwrap();

                let mut iter_nodes = rnodes
                    .find_iter(&l)
                    .map(|m| String::from(m.as_str()))
                    .filter_map(|v| mapping_s2i.get(&v));

                let node = iter_nodes.next().unwrap();
                let nodes = iter_nodes.map(|i| *i).collect_vec();

                // (Valve {rate: rate as usize, name: node, tunnels: nodes.clone()}, nodes)
                (*node, nodes)
            })
            .collect::<HashMap<usize, Vec<usize>>>();

        let rates = lines
            .iter()
            .map(|l| {
                let rate = rrate
                    .find_iter(&l)
                    .find_map(|s| s.as_str().parse::<usize>().ok())
                    .unwrap();

                let mut iter_nodes = rnodes
                    .find_iter(&l)
                    .map(|m| String::from(m.as_str()))
                    .filter_map(|v| mapping_s2i.get(&v));

                let node = iter_nodes.next().unwrap();
                // (Valve {rate: rate as usize, name: node, tunnels: nodes.clone()}, nodes)
                (*node, rate)
            })
            .collect::<HashMap<usize, usize>>();

        eprintln!("tst:{:?}", rates);
        nodes
            .iter()
            .inspect(|(k, u)| {
                println!(
                    "{} : {:?}",
                    mapping_i2s[k],
                    u.iter().map(|u| mapping_i2s[u].clone()).collect_vec()
                )
            })
            .collect_vec();

        let mut all_dists = HashMap::new();
        for i in 0..nodes.len() {
            let rs = find_path::<usize>(&i, &nodes);
            // println!("{:?} -> {:?}", node.0, rs);
            all_dists.insert(i, rs);
        }

        println!("nodes: {:?}", &mapping_i2s);
        let start = mapping_s2i.get("AA").unwrap();

        eprintln!(
            "{:?}",
            find_best(
                &start,
                0,
                0,
                0_u64,
                &30,
                &rates,
                &all_dists,
                &mut HashMap::new()
            )
        );

        for time in 26..=26 {
            // let time = 8;
            eprintln!(
                "{:?}: {:?}",
                time,
                find_best2(
                    &[*start, *start],
                    time,
                    0_u64,
                    &rates,
                    &nodes,
                    &mut HashMap::new(),
                    &mapping_i2s,
                )
            )
        }
    }
}

pub fn find_best2(
    starts: &[usize; 2],
    time: usize,
    released: u64,
    rates: &HashMap<usize, usize>,
    adj_list: &HashMap<usize, Vec<usize>>,
    mut cache: &mut HashMap<(usize, u64, [usize; 2]), usize>,
    better_print: &HashMap<usize, String>,
) -> usize {
    // eprintln!("cache get {:?}", (time, released, starts.clone()));

    // let time = time + 1;
    //println!("[{:?}] valve: {:?}", time, starts);

    if time == 0 {
        return 0;
    }

    if let Some(answer) = cache.get(&(time, released.clone(), starts.clone())) {
        return *answer;
    }
    // starts.iter().sorted().tuples().map(|(elve, elephant)| {

    // }).collect::<Vec<(&usize, &usize)>>();
    println!(
        "[{:?}] {:?}",
        time,
        starts
            .iter()
            .filter_map(|u| better_print.get(u))
            .collect_tuple::<(&String, &String)>()
            .unwrap(),
    );

    // let mut expected_gain = gain;
    // let mut updated_release = released;

    let list = adj_list
        .iter()
        .map(|(k, _)| k)
        .filter(|node| (released & (1 << (**node as usize))) != 0)
        .sorted()
        .map(|k| format!("{}", better_print[k]))
        .collect_vec();

    println!("  opened: {:?}", list);

    let mut wait = vec![];

    print!("  releasing:");
    for i in 0..2 {
        let start = &starts[i];
        let rate = rates[&start];

        if (released & (1_u64 << *start)) == 0 && rate > 0 {
            // updated_release = updated_release | (1_u64 << *start);
            // print!(
            //     " {:?}({:?})" ,
            //     better_print[start], start
            // );
            wait.push((start, true));
        }
        wait.push((start, false));
    }

    println!("");

    let poss = starts
        .iter()
        .map(|node| {
            let mut to_explore = adj_list[node]
                .clone()
                .into_iter()
                .map(|n: usize| (n as usize, false)).collect_vec();

            if (released & (1_u64 << *node)) == 0 && rates[node] > 0 {
                // updated_release = updated_release | (1_u64 << *start);
                // print!(
                //     " {:?}({:?})" ,
                //     better_print[start], start
                // );
                // wait.push((start, true));
                to_explore.push((*node, true));
            }
            to_explore.into_iter()
        })
        .multi_cartesian_product()
            .filter_map(|mut v| {
                v.sort_unstable();
                if v[0] != v[1] || !v[0].1 {
                    Some(v)
                } else {
                    None
                }
            })
        .collect_vec();

    println!("  [{}] all: {:?}", time, poss);

    let res = poss
        .iter()
        .map(|next_pos| {
            println!(
                "Cartesian:[{:?}]  {:?} -> {:?} and {:?} -> {:?}",
                time,
                better_print[&starts[0]],
                better_print[&next_pos[0].0],
                better_print[&starts[1]],
                better_print[&next_pos[1].0]
            );

            let val: usize = adj_list
                .iter()
                .map(|(k, _)| k)
                .filter(|node| (released & (1 << (**node as usize))) != 0)
                .map(|k| rates[k])
                .sum();

            let mut new_release = released;

            for (n, release) in next_pos.iter() {
                if *release {
                    new_release = new_release | (1_u64 << *n);
                }
            }

            println!("[{:?}] {:?} are open for {:?}", time, list, val);

            val + find_best2(
                &[next_pos[0].0, next_pos[1].0],
                time - 1,
                new_release,
                rates,
                adj_list,
                &mut cache,
                &better_print,
            )
        })
        .max()
        .unwrap();

    // println!("store {:?}", (time, released, starts.clone()));
    cache.insert((time, released, starts.clone()), res);
    res
}
