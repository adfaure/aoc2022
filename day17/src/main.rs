use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn print_map(map: &Vec<Vec<u8>>, rock: Vec<Vec<i32>>, x: usize, y: usize) {
    let mut res: Vec<Vec<char>> = vec![];

    for j in 0..map.len() {
        res.push(vec![]);
        for i in 0..map[j].len() {
            match map[j][i] {
                0 => res[j].push('.'),
                1 => res[j].push('#'),
                _ => res[j].push('!'),
            }
        }
    }

    for j in 0..rock.len() {
        for i in 0..rock[j].len() {
            let xi = i + x;
            let yj = j + y;
            // print!("{:?}, {:?}", xi, yj);
            if yj < res.len() && xi < res[j].len() {
                // print!("<--- ok");
                match rock[j][i] {
                    2 => {
                        res[yj][xi] = '@';
                    }
                    _ => {}
                }
            }
            // println!()
        }
    }
    for i in (0..res.len()).rev() {
        println!("{}", res[i].iter().collect::<String>());
    }
}
fn main() -> std::io::Result<()> {
    // parse graph

    let jetpack: String = BufReader::new(File::open("input")?)
        .lines()
        .find_map(|l| l.ok())
        .unwrap();

    // let mut j = jetpack.chars().enumerate().cycle();

    // lets put 9 so we do not need to check boundaries
    // let mut chamber = vec![vec![2; 9]];

    let pieces = [
        vec![vec![1; 4]],
        vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
        vec![vec![1; 3], vec![0, 0, 1], vec![0, 0, 1]],
        vec![vec![1], vec![1], vec![1], vec![1]],
        vec![vec![1; 2], vec![1; 2]],
    ];

    let simu = 1_000_000_000_000;
    let mut chamber = vec![vec![2; 9]];
    if let Some((mut chamber, rocks_at_start, rocks, time, start_heigh, cycle_heigh)) =
        find_cycle(jetpack.clone(), &pieces, simu)
    {
        let need_to_play = simu - rocks_at_start; // 999999997514;

        let nb_play = need_to_play % rocks;
        let height = start_heigh + (cycle_heigh * (need_to_play / rocks));

        println!("cycle lens: {}", rocks);
        println!("found cycle with: {rocks} rocks and we need {} cycles", need_to_play);

        // 1532163742758

        println!(
            "max size: {}",
             height + play(chamber, jetpack, &pieces, nb_play, time)
        );
    } else {
        println!("max size: {}", play(chamber, jetpack, &pieces, simu, 0));
    }

    Ok(())
}

pub fn find_cycle(
    jetpack: String,
    pieces: &[Vec<Vec<i32>>; 5],
    simu: usize,
) -> Option<(Vec<Vec<u8>>, usize, usize, usize, usize, usize)> {
    println!("lol iter: {simu}");
    //  return None;
    let mut chamber = vec![vec![2; 9]];

    let mut real_height = 0;
    let mut total_time = 0;
    let mut monitor_max = 0;
    let mut cache: HashMap<(usize, usize, Vec<Vec<u8>>), (usize, usize, usize)> = HashMap::new();

    // stop sooner with cycles
    let mut iter_bump = 0;

    for (iter, rock) in pieces.iter().cycle().take(simu + 1).enumerate() {
        let highest = chamber.len();
        monitor_max = highest.max(monitor_max);

        let start_y = 3 + highest;
        let start_x = 3;

        // highest = highest + rock.0.len();
        if iter  == simu {
            // print_map(&chamber, vec![], 0, 0);
            println!(
                "max {:?} - max map size: {monitor_max}",
                real_height + highest - 1
            );
        }

        match cache.get(&(
            total_time % jetpack.len(),
            iter % pieces.len(),
            chamber.clone(),
        )) {
            None => {
                cache.insert(
                    (
                        total_time % jetpack.len(),
                        iter % pieces.len(),
                        chamber.clone(),
                    ),
                    (total_time, iter, real_height),
                );
            }
            Some((t, start_iter, start_heigh)) => {
                let cycle_size = iter - start_iter;
                let cycle_h = real_height - start_heigh;

                return Some((
                    chamber,
                    *start_iter,
                    cycle_size,
                    total_time,
                    *start_heigh,
                    cycle_h,
                ));
            }
        }

        for _ in 0..(3 + rock.len()) {
            chamber.push(vec![1, 0, 0, 0, 0, 0, 0, 0, 1]);
        }

        let mut fall_time = 0;
        let fall = jetpack
            .chars()
            .enumerate()
            .cycle()
            .skip(total_time)
            .fold_while((start_x, start_y), |(x, y), (jetidx, c)| {
                let after_push = match c {
                    '>' => (x + 1, y),
                    '<' => (x - 1, y),
                    _ => {
                        panic!("wrong jetpack direction: {}", c)
                    }
                };

                let mut colliding = None;
                for j in 0..rock.len() {
                    for i in 0..rock[j].len() {
                        let map_x = i + after_push.0;
                        let map_y = j + after_push.1;
                        if rock[j][i] > 0 && chamber[map_y][map_x] > 0 {
                            colliding = Some(())
                        }
                    }
                }

                let before_fall = match colliding {
                    None => after_push,
                    Some(_) => (x, y),
                };
                // println!("rock drops: {:?} {:?}", c, before_fall);

                let falling = (before_fall.0, before_fall.1 - 1);
                let mut colliding = None;
                for j in 0..rock.len() {
                    for i in 0..rock[j].len() {
                        let map_x = i + falling.0;
                        let map_y = j + falling.1;
                        if rock[j][i] > 0 && chamber[map_y][map_x] > 0 {
                            colliding = Some(())
                        }
                    }
                }

                // println!("Collides: {:?}", colliding);
                let res = match colliding {
                    None => Continue(falling),
                    Some(_) => Done(before_fall),
                };

                fall_time += 1;
                res
            })
            .into_inner();

        // print_map(&chamber, rock.to_vec(), fall.0, fall.1);
        rock.iter().enumerate().for_each(|(line_idx, line)| {
            for (col, cell) in line.iter().enumerate() {
                chamber[fall.1 + line_idx][fall.0 + col] =
                    if (*cell > 0 || chamber[fall.1 + line_idx][fall.0 + col] > 0) {
                        1
                    } else {
                        0
                    };
            }
        });

        chamber = chamber
            .into_iter()
            .filter(|l| l.iter().skip(1).take(l.len() - 2).any(|c| *c >= 1))
            .collect();

        // print_map(&chamber, vec![], 0, 0);

        // try to shorten the map
        match chamber
            .iter()
            .enumerate()
            .rev()
            .tuple_windows()
            .map(|((i, d), (iu, u))| {
                (
                    i,
                    d.iter().zip(u.iter()).map(|(a, b)| a.max(b)).collect_vec(),
                )
            })
            .find(|(i, c)| c.iter().all(|c| **c == 1))
        {
            None => {}
            Some((i, c)) => {
                // println!("lala: {:?}", (i, c));
                // print_map(&chamber, vec![], 0, 0);
                let d = chamber.drain(..(i - 1));
                real_height += d.len();
                // println!("shorten of {} elements {i}", d.len());
                // panic!("can filter {}", i);
            }
        }

        total_time += fall_time;
    }
    None
}

pub fn play(
    mut chamber: Vec<Vec<u8>>,
    jetpack: String,
    pieces: &[Vec<Vec<i32>>; 5],
    simu: usize,
    start: usize,
) -> usize {
    let mut real_height = 0;
    let mut total_time = start;

    for (iter, rock) in pieces.iter().cycle().take(simu + 1).enumerate() {
        let highest = chamber.len();

        let start_y = 3 + highest;
        let start_x = 3;

        // highest = highest + rock.0.len();
        if iter == simu {
            // print_map(&chamber, vec![], 0, 0);
            return real_height + highest - 1;
        }

        for _ in 0..(3 + rock.len()) {
            chamber.push(vec![1, 0, 0, 0, 0, 0, 0, 0, 1]);
        }

        let mut fall_time = 0;
        let fall = jetpack
            .chars()
            .enumerate()
            .cycle()
            .skip(total_time)
            .fold_while((start_x, start_y), |(x, y), (jetidx, c)| {
                let after_push = match c {
                    '>' => (x + 1, y),
                    '<' => (x - 1, y),
                    _ => {
                        panic!("wrong jetpack direction: {}", c)
                    }
                };

                let mut colliding = None;
                for j in 0..rock.len() {
                    for i in 0..rock[j].len() {
                        let map_x = i + after_push.0;
                        let map_y = j + after_push.1;
                        if rock[j][i] > 0 && chamber[map_y][map_x] > 0 {
                            colliding = Some(())
                        }
                    }
                }

                let before_fall = match colliding {
                    None => after_push,
                    Some(_) => (x, y),
                };

                // println!("---");
                // print_map(&chamber, rock.to_vec(), before_fall.0, before_fall.1);

                let falling = (before_fall.0, before_fall.1 - 1);
                let mut colliding = None;
                for j in 0..rock.len() {
                    for i in 0..rock[j].len() {
                        let map_x = i + falling.0;
                        let map_y = j + falling.1;
                        if rock[j][i] > 0 && chamber[map_y][map_x] > 0 {
                            colliding = Some(())
                        }
                    }
                }

                // println!("Collides: {:?}", colliding);
                let res = match colliding {
                    None => Continue(falling),
                    Some(_) => Done(before_fall),
                };

                fall_time += 1;
                res
            })
            .into_inner();

        rock.iter().enumerate().for_each(|(line_idx, line)| {
            for (col, cell) in line.iter().enumerate() {
                chamber[fall.1 + line_idx][fall.0 + col] =
                    if (*cell > 0 || chamber[fall.1 + line_idx][fall.0 + col] > 0) {
                        1
                    } else {
                        0
                    };
            }
        });

        chamber = chamber
            .into_iter()
            .filter(|l| l.iter().skip(1).take(l.len() - 2).any(|c| *c >= 1))
            .collect();

        // print_map(&chamber, vec![], 0, 0);

        // try to shorten the map
        match chamber
            .iter()
            .enumerate()
            .rev()
            .tuple_windows()
            .map(|((i, d), (iu, u))| {
                (
                    i,
                    d.iter().zip(u.iter()).map(|(a, b)| a.max(b)).collect_vec(),
                )
            })
            .find(|(i, c)| c.iter().all(|c| **c == 1))
        {
            None => {}
            Some((i, c)) => {
                // println!("lala: {:?}", (i, c));
                // print_map(&chamber, vec![], 0, 0);
                // println!("len of chmber: {}", chamber.len());
                let d = chamber.drain(..(i - 1));
                real_height += d.len();
                // println!("shorten of {} elements {i}", d.len());
                // panic!("can filter {}", i);
            }
        }

        total_time += fall_time;
    }
    0
}
