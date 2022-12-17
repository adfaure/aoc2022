use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn print_map(map: &Vec<Vec<u8>>, rock: Vec<Vec<u8>>, x: usize, y: usize) {
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

    println!();
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

    let mut j = jetpack.chars().cycle();

    // lets put 9 so we do not need to check boundaries
    let mut chamber = vec![vec![1; 9]];

    let pieces = [
        vec![vec![2; 4]],
        vec![vec![0, 2, 0], vec![2, 2, 2], vec![0, 2, 0]],
        vec![vec![2; 3], vec![0, 0, 2], vec![0, 0, 2]],
        vec![vec![2], vec![2], vec![2], vec![2]],
        vec![vec![2; 2], vec![2; 2]],
    ];

    let simu = 2022;
    for (iter, rock) in pieces.iter().cycle().take(2022).enumerate() {

        let highest = chamber.len();
        let start_y = 3 + highest;

        let start_x = 3;

        for _ in 0..(3 + rock.len()) {
            chamber.push(vec![1, 0, 0, 0, 0, 0, 0, 0, 1]);
        }

        // print_map(&chamber, rock.to_vec(), start_x, start_y);
        let fall = j
            .fold_while((start_x, start_y), |(x, y), c| {
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
                println!("rock drops: {:?} {:?}", c, before_fall);

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

        // highest = highest + rock.0.len();

        chamber = chamber
            .into_iter()
            .filter(|l| l.iter().skip(1).take(l.len() - 2).any(|c| *c >= 1))
            .collect();

        if iter == simu {
            eprintln!("max {:?}", chamber.len() - 1);
        }

        println!("end!");

        print_map(&chamber, vec![], 0, 0);
    }

    Ok(())
}
