use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use std::str::FromStr;
use fraction::{Fraction, Sign};  // choose the type accordingly with your needs (see prelude module docs)


#[derive(Clone, Debug)]
enum Yell {
    Op((String, char, String)),
    Num(Fraction),
}

#[derive(Clone, Debug)]
struct Monkey {
    name: String,
    yell: Yell,
}

fn find_self(cur: &Monkey, monkeys: &HashMap<String, Monkey>) -> Vec<String> {
    let mut res = vec![];
    let mut target = Some(&cur.name);

    while let Some(target_name) = target {
        res.push(target_name.clone());

        target = match monkeys.iter().find(|(_, m)| match &m.yell {
            Yell::Op((m1, _, _)) if m1 == target_name => true,
            Yell::Op((_, _, m2)) if m2 == target_name => true,
            _ => false,
        }) {
            None => None,
            Some((name, _)) => Some(name),
        };
    }

    res.pop();
    res.into_iter().rev().collect()
}

fn solve_match2(monkeys: &HashMap<String, Monkey>) -> Fraction {
    let root = &monkeys["root"];
    let path_to_self = find_self(&monkeys["humn"], monkeys);
    println!("{path_to_self:?}");

    let mut res = Fraction::from(0);
    let mut number_to_match = Fraction::from(0);
    if let Yell::Op((a, _, b)) = &root.yell {
        if a != &path_to_self[0] {
            number_to_match = solve_rec(&monkeys[a], monkeys);
        } else {
            number_to_match = solve_rec(&monkeys[b], monkeys);
        }

        println!("{a}");
        res = solve2_rec2(
            &monkeys[&path_to_self[0]],
            monkeys,
            &path_to_self,
            Some(number_to_match),
        );
    }

    let mut twisted = monkeys.clone();
    match twisted.get_mut("humn") {
        None => panic!(),
        Some(m) => (*m).yell = Yell::Num(Fraction::from(res)),
    };
    println!("I yell: {}", res);

    println!("numbers should be equals: {}, {}", number_to_match, solve_rec(&twisted[&path_to_self[0]], &twisted));
    return res;
}

fn solve2_rec2(
    cur: &Monkey,
    monkeys: &HashMap<String, Monkey>,
    critical_path: &Vec<String>,
    target: Option<Fraction>,
) -> Fraction {
    match target {
        None => solve_rec(cur, &monkeys),
        Some(target) => {
            match cur.yell {
                Yell::Num(_) => target, // should be humn
                Yell::Op((ref m1, o, ref m2)) => {
                    let (left, solve_first, x) = if critical_path.contains(m1) {
                        (false, m2, m1)
                    } else {
                        (true, m1, m2)
                    };

                    let value = solve_rec(&monkeys[solve_first], &monkeys);

                    match o {
                        '+' => {
                            // target + value
                            solve2_rec2(&monkeys[x], monkeys, critical_path, Some(target - value))
                        }
                        '-' => {
                            // target - value
                            if left {
                                solve2_rec2(&monkeys[x], monkeys, critical_path, Some(-target + value))
                            } else {
                                solve2_rec2(&monkeys[x], monkeys, critical_path, Some(target + value))
                            }
                        }
                        '*' => {
                            // target / value
                            solve2_rec2(&monkeys[x], monkeys, critical_path, Some(target / value))
                        }
                        '/' => {
                            if left {
                                // target * value
                                println!("left: {target} {value}");
                                solve2_rec2(
                                    &monkeys[x],
                                    monkeys,
                                    critical_path,
                                    Some(value / target),
                                )
                            } else {
                                //  value / target
                                println!("right");
                                solve2_rec2(
                                    &monkeys[x],
                                    monkeys,
                                    critical_path,
                                    Some(value * target),
                                )
                            }
                        }
                        _ => panic!(),
                    }
                }
            }
        }
    }
}

fn solve_rec(
    cur: &Monkey,
    monkeys: &HashMap<String, Monkey>
) -> Fraction {


    let res = match cur.yell {
        Yell::Num(n) => n,
        Yell::Op((ref m1, o, ref m2)) => match o {
            '+' => {
                solve_rec(&monkeys[m1], &monkeys) + solve_rec(&monkeys[m2], &monkeys)
            }
            '-' => {
                solve_rec(&monkeys[m1], &monkeys) - solve_rec(&monkeys[m2], &monkeys)
            }
            '*' => {
                solve_rec(&monkeys[m1], &monkeys) * solve_rec(&monkeys[m2], &monkeys)
            }
            '/' => {
                solve_rec(&monkeys[m1], &monkeys) / solve_rec(&monkeys[m2], &monkeys)
            }
            _ => panic!(),
        },
    };

    return res;
}

fn main() -> std::io::Result<()> {
    let re = Regex::new(r"([a-z]+): ([a-z]+) ([+-/*]) ([a-z]+)").unwrap();
    let re_d = Regex::new(r"([a-z]+): (\d+)").unwrap();

    let monkeys = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .flat_map(|l| {
            re.captures_iter(&l)
                .chain(re_d.captures_iter(&l))
                // .unwrap()
                .filter_map(|d| {
                    // println!("{d:?} {}", d.len());
                    if d.len() == 5 {
                        let name = d.get(1).unwrap().as_str();
                        let m1 = d.get(2).unwrap().as_str();
                        let op = d.get(3).unwrap().as_str();
                        let m2 = d.get(4).unwrap().as_str();

                        Some(Monkey {
                            name: String::from(name),
                            yell: Yell::Op((
                                String::from(m1),
                                String::from(op).chars().next().unwrap(),
                                String::from(m2),
                            )),
                        })
                    } else if d.len() == 3 {
                        let name = d.get(1).unwrap().as_str();
                        let number = d.get(2).unwrap().as_str().parse::<i64>().ok().unwrap();
                        Some(Monkey {
                            name: String::from(name),
                            yell: Yell::Num(Fraction::from(number)),
                        })
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        // .inspect(|m| println!("{m:?}"))
        .map(|monkey| (monkey.name.clone(), monkey))
        .collect::<HashMap<_, _>>();

    // println!("{:?}", monkeys);

    let root = &monkeys[&String::from("root")];
    let res = solve_rec(&root, &monkeys);
    println!("root yells: {res}");
    println!("part 2 {}", solve_match2(&monkeys));

    Ok(())
}
