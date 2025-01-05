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
        .fold(file.clone(), |mut acc, (idx, n)| {
            let pos = acc.iter().position(|e| idx == e.0).unwrap();

            acc.remove(pos);

            let new_idx = (pos as i64 + n).rem_euclid(acc.len() as i64);
            acc.insert(new_idx as usize, (idx, n));

            acc
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

    let magic_number = 811589153;
    let file_with_key = file.iter().map(|e| (e.0, e.1 * magic_number)).collect_vec();

    let res = file_with_key
        .clone()
        .into_iter()
        .cycle()
        .take(file_with_key.len() * 10)
        .enumerate()
        .fold(file_with_key.clone(), |mut acc, (debug, (idx, n))| {
            let pos = acc.iter().position(|e| idx == e.0).unwrap();

            acc.remove(pos);

            let new_idx = (pos as i64 + n).rem_euclid(acc.len() as i64);
            acc.insert(new_idx as usize, (idx, n));
            acc
        });

    assert!(res.iter().any(|(_idx, n)| *n == 0));

    let p2 = res
        .into_iter()
        .cycle()
        .map(|e| e.1)
        .skip_while(|e| *e != 0)
        .enumerate()
        .filter(|(idx, _)| [1000, 2000, 3000].contains(idx))
        .take(3)
        .map(|e| e.1)
        .sum::<i64>();

    println!("p2: {:?}", p2);
    Ok(())
}
