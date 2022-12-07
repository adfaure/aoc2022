use crate::helpers::read_lines;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn day6() {
    if let Ok(lines) = read_lines("./inputs/input-d6.txt") {
        lines
            .map(|l| l.unwrap())
            .map(|s| {
                let r = s
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(4)
                    .fold_while(4, |acc, chars| {
                        let h: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
                        if h.len() == chars.len() {
                            return Done(acc);
                        }
                        Continue(acc + 1)
                    })
                    .into_inner();
                println!("res: {}", r);
            })
            .for_each(drop);
    }
    if let Ok(lines) = read_lines("./inputs/input-d6.txt") {
        lines
            .map(|l| l.unwrap())
            .map(|s| {
                let r = s
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(14)
                    .fold_while(14, |acc, chars| {
                        let h: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
                        if h.len() == chars.len() {
                            return Done(acc);
                        }
                        Continue(acc + 1)
                    })
                    .into_inner();
                println!("res: {}", r);
            })
            .for_each(drop);
    }
}
