use crate::helpers::read_lines;
use interval_set::interval_set::ToIntervalSet;
use interval_set::IntervalSet;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use regex::Regex;
use std::collections::HashSet;

pub fn day15() {
    let re = Regex::new(r"([-]?\d+)").unwrap();

    if let Ok(lines) = read_lines("./inputs/input-d15.txt") {
        let lines = lines.filter_map(|l| l.ok()).collect_vec();

        let beacons = lines
            .clone()
            .into_iter()
            .flat_map(|l| {
                re.find_iter(&l)
                    .map(|m| String::from(m.as_str()))
                    .collect::<Vec<String>>()
            })
            .filter_map(|ch| ch.parse::<i32>().ok())
            .tuples::<(i32, i32, i32, i32)>()
            .map(|(_, _, bx, by)| (bx, by))
            .collect::<HashSet<_>>();

        let mut target_row = 0;
        let max = 4000000;

        let tuples = lines
            .iter()
            .flat_map(|l| {
                re.find_iter(&l)
                    .map(|m| String::from(m.as_str()))
                    .collect::<Vec<String>>()
            })
            .filter_map(|ch| ch.parse::<i32>().ok())
            .tuples::<(i32, i32, i32, i32)>()
            .collect_vec();

        while target_row != max {
            let negatives =
                tuples
                    .iter()
                    .fold_while(IntervalSet::empty(), |mut acc, (sx, sy, bx, by)| {
                        // println!("{:?}", acc);
                        let dist = (sx - bx).abs() + (sy - by).abs();
                        let dist_row = (sy - target_row).abs();

                        if dist_row <= dist {
                            let itv = vec![(
                                (sx - (dist - dist_row)).max(0).min(max) as u32,
                                (sx + (dist - dist_row)).max(0).min(max) as u32,
                            )]
                            .to_interval_set();
                            acc = acc.union(itv);
                        }

                        if acc.size() <= max as u32 {
                            Continue(acc)
                        } else {
                         Done(IntervalSet::empty())
                        }
                    }).into_inner();

            if negatives.size() != 0 && negatives.size() != (1 + max) as u32 {
                println!("{:4}: {:}, {:}", target_row, negatives, negatives.size());
                panic!("");
            }

            // if negatives.size() != 0 && negatives.size() < max as u32 {
            //     for j in minmax_y.0..=minmax_y.1 {
            //         // print!("{:3}: ", j);
            //         for i in minmax_x.0..=minmax_x.1 {
            //             if negatives.contains(&(i, j)) {
            //                 // print!("#");
            //             } else if beacons.contains(&(i, j)) {
            //                 // print!("B");
            //             } else {
            //                 // print!(".");
            //                 panic!("gotcha: {} {} = {}", i, j, j + (4000000 * i));
            //             }
            //         }
            //         // println!("");
            //     }
            // }

            if target_row % 10000 == 0 {
                // println!("negatives: {:?} {:?}", target_row, negatives.size());
            }

            target_row += 1;
        }

        // println!("negatives: {:?} {:?} {:?}", negatives, minmax_x, minmax_y);
    }
}
