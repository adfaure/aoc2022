use crate::helpers::read_lines;
use std::collections::HashSet;

pub fn move_dir(dir: &str, x: &i32, y: &i32) -> (i32, i32) {
    match dir {
        "R" => (*x + 1, *y),
        "L" => (*x - 1, *y),
        "U" => (*x, *y + 1),
        "D" => (*x, *y - 1),
        _ => {
            panic!("error")
        }
    }
}

pub fn adjacent(x1: &i32, y1: &i32, x2: &i32, y2: &i32) -> bool {
    for i in (*x1 - 1)..=(*x1 + 1) {
        for j in (*y1 - 1)..=(*y1 + 1) {
            if i == *x2 && j == *y2 {
                return true;
            }
        }
    }
    return false;
}

pub fn keep_up(x1: &i32, y1: &i32, x2: &i32, y2: &i32) -> (i32, i32) {
    let aligned_x = x1 == x2;
    let aligned_y = y1 == y2;
    let adj = adjacent(x1, y1, x2, y2);

    if !adj {
        if aligned_x {
            for i in (*y1 - 1)..=(*y1 + 1) {
                if adjacent(x1, y1, &x1, &i) && adjacent(x2, y2, &x1, &i) {
                    return (*x1, i);
                }
            }
        }

        if aligned_y {
            for j in (*x1 - 1)..=(*x1 + 1) {
                if adjacent(x1, y1, &j, &y1) && adjacent(x2, y2, &j, &y1) {
                    return (j, *y1);
                }
            }
        }

        for i in (*x1 - 1)..=(*x1 + 1) {
            for j in (*y1 - 1)..=(*y1 + 1) {
                if adjacent(x1, y1, &i, &j) && adjacent(x2, y2, &i, &j) && ((*x1 == i) || (*y1 == j)) {
                    return (i, j);
                }
            }
        }

        for i in (*x1 - 1)..=(*x1 + 1) {
            for j in (*y1 - 1)..=(*y1 + 1) {
                if adjacent(x1, y1, &i, &j) && adjacent(x2, y2, &i, &j) {
                    return (i, j);
                }
            }
        }
    }

    (*x2, *y2)
}

pub fn day9() {
    if let Ok(lines) = read_lines("./inputs/input-d9") {
        let mut results: HashSet<(i32, i32)> = HashSet::new();

        let mut knots = vec![];
        let nb_knots = 10;

        for i in 0..nb_knots {
            knots.push((0, 0))
        }
        results.insert((0, 0));

        for line in lines.map(|c| c.unwrap()) {
            let instructions = line.split(" ").collect::<Vec<&str>>();
            let mut total = instructions[1].parse::<usize>().unwrap();

            for i in 0..total {
                let mut last_pos: (i32, i32) = knots[0];
                knots[0] = move_dir(instructions[0], &knots[0].0, &knots[0].1);

                for k in 1..nb_knots {
                    let tail = knots[k];
                    let prev = knots[k - 1];

                    knots[k] = keep_up(&prev.0, &prev.1, &tail.0, &tail.1);
                }

                // println!("{:?}", knots[nb_knots - 1]);
                results.insert((knots[nb_knots - 1].0, knots[nb_knots - 1].1));
                println!("all: {:?}", knots);
            }

            // println!("all: {:?}", knots);
            // println!("res: {:?}", results);
        }

        println!("{:?}", results.len());
    }
}
