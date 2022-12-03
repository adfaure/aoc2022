use crate::helpers::read_lines;
use itertools::izip;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

fn scan_bag(rucsaks: &str) -> i32 {
    let (first, last) = rucsaks.split_at(rucsaks.len() / 2);
    let (left, right) = first.chars().zip(last.chars()).fold(
        (HashSet::new(), HashSet::new()),
        |(mut rf, mut rl), (f, l)| {
            rf.insert(f);
            rl.insert(l);
            return (rf, rl);
        },
    );

    left.intersection(&right)
        .map(|e| {
            let score = match e {
                'a'...'z' => 1 + (*e as i32) - ('a' as i32),
                _ => 27 + (*e as i32) - ('A' as i32),
            };
            score
        })
        .next()
        .unwrap()
}

pub fn day3() {
    if let Ok(lines) = read_lines("./inputs/input-d3.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(content) = line {
                sum += scan_bag(&content);
            }
        }
        // part two
        println!("part1: {}", sum);
    }

    if let Ok(lines) = read_lines("./inputs/input-d3.txt") {
        let v: i32= lines
            .enumerate()
            .map(|(idx, line)| (idx / 3, line.unwrap()))
            .into_group_map_by(|(idx, line)| *idx)
            .into_iter()
            .map(|(key, v)| {
                v.iter()
                    .map(|e| {
                        let r: HashSet<char> = HashSet::from_iter(e.1.chars());
                        r
                    })
                    .reduce(|acc: HashSet<char>, v| acc.intersection(&v).map(|c| *c).collect())
                    .unwrap()
                    .iter()
                    .map(|badge| {
                        // println!("badge: {:?}", badge);
                        match badge {
                            'a'...'z' => 1 + (*badge as i32) - ('a' as i32),
                            _ => 27 + (*badge as i32) - ('A' as i32),
                        }
                    })
                    .next()
                    .unwrap()
            }).sum();
        println!("part2: {}", v);
    }
}
