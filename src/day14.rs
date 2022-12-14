use crate::helpers::read_lines;
use core::iter::once;
use core::iter::repeat;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn day14() {
    if let Ok(lines) = read_lines("./inputs/input-d14.txt") {
        let walls = lines
            .filter_map(|l| l.ok())
            .map(|l| {
                l.split(" -> ")
                    .map(|coords| {
                        coords
                            .split(",")
                            .tuples()
                            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                    })
                    .flatten()
                    .collect::<Vec<(i32, i32)>>()
            })
            .map(|wall| {
                wall.iter()
                    .tuple_windows::<(_, _)>()
                    .flat_map(|((x1, y1), (x2, y2))| {
                        if *x1 == *x2 {
                            let (from, to) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                            repeat(*x1).zip(*from..=*to).collect_vec()
                        } else {
                            let (from, to) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                            (*from..=*to).zip(repeat(*y1)).collect_vec()
                        }
                    })
                    .collect::<Vec<(i32, i32)>>()
            })
            .flatten()
            .collect::<Vec<(i32, i32)>>();

        let minmax_x = match once(&(500, 0))
            .chain(walls.iter())
            .map(|e| e.0)
            .minmax_by(|e, o| e.cmp(&o))
        {
            MinMax(min, max) => (min, max),
            _ => panic!(""),
        };

        let minmax_y = match once(&(500, 0))
            .chain(walls.iter())
            .map(|e| e.1)
            .minmax_by(|e, o| e.cmp(&o))
        {
            MinMax(min, max) => (min, max),
            _ => panic!(""),
        };
        println!("minmax_x: {:?}", minmax_x);

        // let dims = minmax
        let normalize = walls
            .iter()
            .map(|(x, y)| (x - minmax_x.0, y - minmax_y.0))
            .collect_vec();

        let mut map = vec![
            vec![0; 1 + (minmax_x.1 - minmax_x.0) as usize];
            1 + (minmax_y.1 - minmax_y.0) as usize
        ];

        let generator = (500 - minmax_x.0 as usize, 0 - minmax_y.0 as usize);
        map[generator.1][generator.0] = 2;

        for (x, y) in normalize {
            map[y as usize][x as usize] = 1;
        }

        let successors: Vec<(i32, i32)> = vec![(0, 1), (-1, 1), (1, 1)];

        let mut sand = Some((generator.0 as i32, generator.1 as i32));
        let mut total = 0;
        'main: while let Some((x, y)) = sand {
            sand = None;

            for succ in &successors {
                let pos = (x + succ.0, y + succ.1);
                if pos.0 < 0
                    || pos.0 >= 1 + (minmax_x.1 - minmax_x.0)
                    || pos.1 < 0
                    || pos.1 >= 1 + (minmax_y.1 - minmax_y.0)
                {
                    println!("end of program! {:?} total: {:?}", pos, total);
                    sand = None;
                    break 'main;
                }

                if map[pos.1 as usize][pos.0 as usize] == 0 {
                    sand = Some(pos);
                    break;
                }
            }

            match sand {
                Some(_) => {}
                None => {
                    map[y as usize][x as usize] = 3;
                    sand = Some((generator.0 as i32, generator.1 as i32));
                    total += 1;
                }
            }
        }
    }

    if let Ok(lines) = read_lines("./inputs/input-d14.txt") {
        let walls = lines
            .filter_map(|l| l.ok())
            .map(|l| {
                l.split(" -> ")
                    .map(|coords| {
                        coords
                            .split(",")
                            .tuples()
                            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                    })
                    .flatten()
                    .collect::<Vec<(i32, i32)>>()
            })
            .map(|wall| {
                wall.iter()
                    .tuple_windows::<(_, _)>()
                    .flat_map(|((x1, y1), (x2, y2))| {
                        if *x1 == *x2 {
                            let (from, to) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                            repeat(*x1).zip(*from..=*to).collect_vec()
                        } else {
                            let (from, to) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                            (*from..=*to).zip(repeat(*y1)).collect_vec()
                        }
                    })
                    .collect::<Vec<(i32, i32)>>()
            })
            .flatten()
            .collect::<Vec<(i32, i32)>>();

        let minmax_y = match once(&(500, 0))
            .chain(walls.iter())
            .map(|e| e.1)
            .minmax_by(|e, o| e.cmp(&o))
        {
            MinMax(min, max) => (min, max),
            _ => panic!(""),
        };

        let minmax_x = match once(&(500, 0))
            .chain(walls.iter())
            .map(|e| e.0)
            .minmax_by(|e, o| e.cmp(&o))
        {
            MinMax(min, max) => (min, max),
            _ => panic!(""),
        };
        let x_shift = 1 + (minmax_x.1 - minmax_x.0) / 2;

        println!("floor: {:?}", 2 + (minmax_y.1 - minmax_y.0));
        let floor = 2 + (minmax_y.1 - minmax_y.0);
        let mapsize_x = 5 + floor * 2;

        println!("map size: {:?}", mapsize_x);
        println!(
            "floor: {:?} x size: {:?}",
            2 + (minmax_y.1 - minmax_y.0),
            minmax_x
        );

        // let dims = minmax
        let normalize = walls
            .iter()
            .map(|(x, y)| (x - minmax_x.0 + (mapsize_x/2) - (x_shift), y - minmax_y.0))
            .collect_vec();

        let mut map = vec![vec![0; 2 + mapsize_x as usize]; floor as usize];
        map.push(vec![1; 2 + mapsize_x as usize]);

        let generator = (
            (500 - minmax_x.0  + mapsize_x/2 - x_shift) as i32,
            0 - minmax_y.0 as usize,
        );
        println!("generator: {:?} {:?}", generator, x_shift);
        map[generator.1 as usize][generator.0 as usize] = 2;
        println!("map size: {:?}", mapsize_x);

        for (x, y) in normalize {
            // println!("{:?}", (x,y));
            map[y as usize][x as usize] = 1;
        }

        let successors: Vec<(i32, i32)> = vec![(0, 1), (-1, 1), (1, 1)];

        // println!("{:?}", generator);
        let mut sand = Some((generator.0 as i32, generator.1 as i32));
        let mut total = 0;
        'main: while let Some((x, y)) = sand {
            sand = None;

            for succ in &successors {
                let pos = (x + succ.0, y + succ.1);
                if pos.0 >= 0
                    && pos.0 < mapsize_x
                    && pos.1 >= 0
                    && pos.1 < 2 + (minmax_y.1 - minmax_y.0)
                {
                    if map[pos.1 as usize][pos.0 as usize] == 0 {
                        sand = Some(pos);
                        break;
                    }
                }
            }

            match sand {
                Some(_) => {}
                None => {
                    if x == generator.0 as i32 && y == generator.1 as i32 {
                        sand = None;
                        break 'main;
                    }

                    map[y as usize][x as usize] = 3;
                    sand = Some((generator.0 as i32, generator.1 as i32));
                    total += 1;
                }
            }
        }
        for m in &map {
            let repr = m
                .iter()
                .map(|t| match t {
                    0 => String::from("."),
                    1 => String::from("#"),
                    _ => String::from("o"),
                })
                .collect::<String>();
             println!("{}", repr);
        }

        println!("reach top in {:?} steps", total + 1);
    }
}
