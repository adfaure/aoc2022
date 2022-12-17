use crate::helpers::read_lines;
use itertools::Itertools;
use std::cmp::Ordering;
use std::iter::FromIterator;

#[derive(Debug, Eq)]
pub enum List {
    Val(i32),
    List(Vec<List>),
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (List::Val(v1), List::Val(v2)) => return v1 == v2,
            (List::List(l1), List::List(l2)) => return l1.eq(l2),
            _ => false,
        }
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        let res = match (self, other) {
            (List::Val(v1), List::Val(v2)) => Some(v1.cmp(v2)),
            (List::List(l1), List::List(l2)) => l1.partial_cmp(l2),
            (List::List(l1), List::Val(v2)) => self.partial_cmp(&List::List(vec![List::Val(*v2)])),
            (List::Val(v1), List::List(l2)) => List::List(vec![List::Val(*v1)]).partial_cmp(other),
            _ => None,
        };
        return res.unwrap();
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (List::Val(v1), List::Val(v2)) => return Some(v1.cmp(v2)),
            (List::List(l1), List::List(l2)) => return l1.partial_cmp(l2),
            (List::List(l1), List::Val(v2)) => {
                return self.partial_cmp(&List::List(vec![List::Val(*v2)]))
            }
            (List::Val(v1), List::List(l2)) => {
                return List::List(vec![List::Val(*v1)]).partial_cmp(other)
            }
            _ => None,
        }
    }
}

pub fn construct<S, I>(mut n: &mut I) -> Option<List>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    match construct0(n) {
        Some(List::List(mut l)) => {
            return l.pop();
        }
        _ => panic!("error"),
    }
}

pub fn construct0<S, I>(mut n: &mut I) -> Option<List>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    let mut l = vec![];

    while let Some(i) = n.next() {
        let i = i.as_ref();

        match i {
            "," => {}
            "]" => return Some(List::List(l)),
            "[" => {
                match construct0(n) {
                    Some(res) => l.push(res),
                    None => {}
                };
            }
            e => {
                l.push(List::Val(i.parse::<i32>().unwrap()));
            }
        };
    }

    if l.is_empty() {
        return None;
    } else {
        return Some(List::List(l));
    }
}

pub fn day13() {
    if let Ok(lines) = read_lines("./inputs/input-d13.txt") {
        let mut t = lines
            .map(|r| r.unwrap())
            .filter(|l| l != "")
            .map(|l| {
                // l.chars().fold(vec![], |c| {})
                l.chars()
                    .batching(|it| match it.next() {
                        None => None,
                        Some(x) => match x {
                            '[' | ']' | ',' => Some(String::from(x)),
                            '0'..='9' => Some(
                                it.peeking_take_while(|c| ('0'..='9').contains(c))
                                    .fold(format!("{}", x), |acc, c| format!("{}{}", acc, c)),
                            ),
                            _ => None,
                        },
                    })
                    .collect::<Vec<String>>()
            })
            .filter_map(|l| construct(&mut l.iter()))
            .tuples()
            .map(|(l, r)| l < r)
            .enumerate()
            .filter(|e| e.1)
            .map(|e| e.0 + 1)
            .sum::<usize>();

        println!("{:#?}", t);
    }

    if let Ok(lines) = read_lines("./inputs/input-d13.txt") {
        let mut lists = lines
            .map(|r| r.unwrap())
            .filter(|l| l != "")
            .map(|l| {
                // l.chars().fold(vec![], |c| {})
                l.chars()
                    .batching(|it| match it.next() {
                        None => None,
                        Some(x) => match x {
                            '[' | ']' | ',' => Some(String::from(x)),
                            '0'..='9' => Some(
                                it.peeking_take_while(|c| ('0'..='9').contains(c))
                                    .fold(format!("{}", x), |acc, c| format!("{}{}", acc, c)),
                            ),
                            _ => None,
                        },
                    })
                    .collect::<Vec<String>>()
            })
            .filter_map(|l| construct(&mut l.iter()))
            .sorted()
            .enumerate()
            .filter(|(u, l)| {
                (l == &List::List(vec![List::List(vec![List::Val(2)])]))
                    || (l == &List::List(vec![List::List(vec![List::Val(6)])]))
            })
            .map(|e| e.0 + 1)
            .product::<usize>();

        println!("{:?}", lists);
    }
}
