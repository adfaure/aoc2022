use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Ore {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Ore {
    type Err = ();

    fn from_str(input: &str) -> Result<Ore, Self::Err> {
        match input {
            "ore" => Ok(Ore::Ore),
            "obsidian" => Ok(Ore::Obsidian),
            "clay" => Ok(Ore::Clay),
            "geode" => Ok(Ore::Geode),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct OreCollecting {
    produce: Ore,
    cost: Vec<(Ore, usize)>,
}

fn step<'a>(
    mut bag: HashMap<Ore, usize>,
    machines: &HashMap<Ore, usize>,
    blueprints: &'a Vec<OreCollecting>,
) -> (
    HashMap<Ore, usize>,    // bag
    Vec<&'a OreCollecting>, // Machine that can be consctructed
) {
    let canbuild = blueprints
        .iter()
        .filter_map(|blueprint| {
            blueprint
                .cost
                .iter()
                .all(|(ore, cost)| match bag.get(ore) {
                    None => false,
                    Some(amount) => amount >= cost,
                })
                .then_some(blueprint)
        })
        .collect_vec();

    // Collect ores
    machines.iter().for_each(|(ore, n)| {
        match bag.get_mut(&ore) {
            Some(v) => {
                *v += *n;
            }
            None => {
                panic!()
            }
        };
    });

    (bag, canbuild)
}

fn time_to_build<'a>(
    ore: &Ore,
    bag: &HashMap<Ore, usize>,
    machines: &HashMap<Ore, usize>,
    blueprints: &'a Vec<OreCollecting>,
) -> Option<usize> {
    let machine_to_build = blueprints.iter().find(|oc| oc.produce == *ore).unwrap();
    match machine_to_build
        .cost
        .iter()
        .all(|(need_ore, cost)| match bag.get(&need_ore) {
            // Check if we can just do it now
            Some(amount) => amount >= cost,
            None => false,
        })
        .then_some(0 /* still need to wait 1 m*/)
    {
        None => machine_to_build
            .cost
            .iter()
            .map(|(ore, amount)| match bag.get(&ore) {
                None => panic!("incomplete bag"),
                Some(quantity) => (ore, quantity.saturating_sub(*amount)),
            })
            .filter_map(|(ore, nb_missing)| (nb_missing > 0).then_some((ore, nb_missing)))
            .filter_map(|(ore, missing_quantity)| match machines.get(ore) {
                None => panic!("Incomplete machine set"),
                Some(0) => None,
                Some(nb_machines) => Some((missing_quantity / nb_machines)),
            })
            .max(),
        Some(v) => Some(v),
    }
}

fn compare_bags(a: &HashMap<Ore, usize>, o: &HashMap<Ore, usize>) -> Ordering {
    let ore = (
        a.get(&Ore::Ore).unwrap_or(&0),
        o.get(&Ore::Ore).unwrap_or(&0),
    );
    let cla = (
        a.get(&Ore::Clay).unwrap_or(&0),
        o.get(&Ore::Clay).unwrap_or(&0),
    );
    let obs = (
        a.get(&Ore::Obsidian).unwrap_or(&0),
        o.get(&Ore::Obsidian).unwrap_or(&0),
    );
    let geo = (
        a.get(&Ore::Geode).unwrap_or(&0),
        o.get(&Ore::Geode).unwrap_or(&0),
    );

    geo.0
        .cmp(&geo.1)
        .then(obs.0.cmp(&obs.1))
        .then(cla.0.cmp(&cla.1))
        .then(ore.0.cmp(&ore.1))
}

fn simulation(
    time: usize,
    bag: HashMap<Ore, usize>,
    machines: HashMap<Ore, usize>,
    blueprints: &Vec<OreCollecting>,
    cache: &mut HashMap<(usize, Vec<(Ore, usize)>, Vec<(Ore, usize)>), Option<HashMap<Ore, usize>>>,
) -> Option<HashMap<Ore, usize>> {
    let state = (
        time,
        bag.clone().into_iter().collect_vec(),
        machines.clone().into_iter().collect_vec(),
    );
    match cache.get(&state) {
        Some(v) => {
            return v.clone();
        }
        None => {}
    }

    let (new_bag, can_build) = step(bag.clone(), &machines, blueprints);
    if time == 0 {
        return Some(bag);
    }

    let max = can_build
        .iter()
        .filter(|or| {
            blueprints.iter().find(|oc| {
                oc.cost
                    .iter()
                    .any(|(ore, cost)| (*ore == or.produce && *cost > machines[&or.produce]) || or.produce == Ore::Geode )
            }).is_some()
        })
        .map(|orecol| Some(orecol))
        .chain(core::iter::once(None))
        .filter_map(|orecollecting| {
            match orecollecting {
                Some(orecollecting) => {
                    // println!("[{time}] will build: {orecollecting:?} bag is: {bag:?} and machines: {machines:?}");
                    let mut new_machines = machines.clone();
                    let mut new_bag = new_bag.clone();

                    for (ore, cost) in orecollecting.cost.iter() {
                        *new_bag.entry(*ore).or_insert(0) -= cost;
                    }
                    *new_machines.entry(orecollecting.produce).or_insert(0) += 1;
                    simulation(time - 1, new_bag.clone(), new_machines, blueprints, cache)
                }
                None => simulation(
                    time - 1,
                    new_bag.clone(),
                    machines.clone(),
                    blueprints,
                    cache,
                ),
            }
        })
        .max_by(|bag, other| compare_bags(bag, other));

    cache.insert(state, max.clone());

    max
}

fn main() -> std::io::Result<()> {
    let re = Regex::new(r"Blueprint (\d+):").unwrap();
    let ore = Regex::new(r" Each ([a-z]+) robot costs (\d+) ([a-z]+)\.").unwrap();
    let and =
        Regex::new(r" Each ([a-z]+) robot costs (\d+) ([a-z]+) and (\d+) ([a-z]+)\.").unwrap();

    let bps = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| {
            let n = re
                .captures(&l)
                .unwrap()
                .iter()
                .flatten()
                .find_map(|tok| tok.as_str().parse::<i32>().ok())
                .unwrap();

            let cost = ore
                .captures_iter(&l)
                .chain(and.captures_iter(&l))
                // .unwrap()
                .filter_map(|d| {
                    // println!("{d:?} {}", d.len());
                    if d.len() == 4 {
                        let ore = Ore::from_str(d.get(1).unwrap().as_str()).unwrap();
                        let cost = d.get(2).unwrap().as_str().parse::<usize>().ok().unwrap();
                        let ore_cost = Ore::from_str(d.get(3).unwrap().as_str()).unwrap();
                        Some(OreCollecting {
                            produce: ore,
                            cost: vec![(ore_cost, cost)],
                        })
                    } else if d.len() == 6 {
                        let ore = Ore::from_str(d.get(1).unwrap().as_str()).unwrap();

                        let cost = d.get(2).unwrap().as_str().parse::<usize>().ok().unwrap();
                        let ore_cost = Ore::from_str(d.get(3).unwrap().as_str()).unwrap();

                        let cost2 = d.get(4).unwrap().as_str().parse::<usize>().ok().unwrap();
                        let ore_cost2 = Ore::from_str(d.get(5).unwrap().as_str()).unwrap();
                        Some(OreCollecting {
                            produce: ore,
                            cost: vec![(ore_cost, cost), (ore_cost2, cost2)],
                        })
                    } else {
                        None
                    }
                })
                .collect_vec();

            Some((n, cost))
        })
        // .inspect(|bp| println!("{bp:?}"))
        .collect_vec();

    let sum = bps
        .par_iter()
        .map(|(id, bp)| {
            println!("compute for {id:?}");

            let mut machines = bp
                .clone()
                .into_iter()
                .map(|machine| (machine.produce, 0))
                .collect::<HashMap<Ore, usize>>();

            machines.insert(Ore::Ore, 1);

            let bag = HashMap::from([
                (Ore::Ore, 0),
                (Ore::Clay, 0),
                (Ore::Obsidian, 0),
                (Ore::Geode, 0),
            ]);

            let res = simulation(24, bag, machines, &bp, &mut HashMap::new()).unwrap();

            println!(
                "{id} : {} -> {}",
                res[&Ore::Geode],
                id * res[&Ore::Geode] as i32
            );
            (id, res[&Ore::Geode] as i32)
        })
        .map(|(id, res)| res * id);

    // println!("total: {:?}", sum.sum::<i32>());
    // for (indice, bp) in blueprints {
    //     println!(
    //         "{:?}",
    //         simulation(24, bag, machines, &bp, &mut HashMap::new())
    //     );
    // }
    //
    let p2: i32 = bps
        .par_iter()
        .take(3)
        .map(|(id, bp)| {
            println!("compute for {id:?}");

            let mut machines = bp
                .clone()
                .into_iter()
                .map(|machine| (machine.produce, 0))
                .collect::<HashMap<Ore, usize>>();

            machines.insert(Ore::Ore, 1);

            let bag = HashMap::from([
                (Ore::Ore, 0),
                (Ore::Clay, 0),
                (Ore::Obsidian, 0),
                (Ore::Geode, 0),
            ]);

            let res = simulation(32, bag, machines, &bp, &mut HashMap::new()).unwrap();

            println!(
                "{id} : {} -> {}",
                res[&Ore::Geode],
                id * res[&Ore::Geode] as i32
            );
            (id, res[&Ore::Geode] as i32)
        })
        .map(|(id, res)| res * id)
        .product();

    println!("total p2: {:?}", p2);
    Ok(())
}
