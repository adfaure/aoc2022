use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .filter_map(|l| l.parse::<i64>().ok())
        .enumerate()
        .map(|(idx, n)| (idx as i64, n))
        .collect_vec();

    let res = file
        .clone()
        .into_iter()
        .fold(file.clone(), |mut acc, (idx, mut n)| {
            // print!(
            //     "acc (move {n:>2}): {:?}",
            //     acc.iter().map(|e| e.1).collect_vec()
            // );
            let pos = acc.iter().position(|e| idx == e.0).unwrap();

            acc.remove(pos);

            let new_idx = (pos as i64 + n).rem_euclid(acc.len() as i64);
            acc.insert(new_idx as usize, (idx, n));

            // println!(" ==> {:?}", acc.iter().map(|e| e.1).collect_vec());
            acc

            // acc.into_iter()
            //     .map(|(old_idx, n)| {
            //         if idx > old_idx {
            //             (idx - n, n)
            //         } else if idx < old_idx {
            //             (idx + n, n)
            //         } else {
            //             (idx, n)
            //         }
            //     })
            //     .collect_vec()
        });

    assert!(res.iter().any(|(_idx, n)| *n == 0));

    let p1 = res
        .into_iter()
        .cycle()
        .map(|e| e.1)
        .skip_while(|e| *e != 0)
        .enumerate()
        .filter(|(idx, _)| [1000, 2000, 3000].contains(idx))
        .take(3)
        .map(|e| e.1)
        .sum::<i64>();

    println!("p1: {:?}", p1);

    Ok(())
}
