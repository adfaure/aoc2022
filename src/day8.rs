use crate::helpers::read_lines;
use std::collections::HashSet;

pub fn day8() {
    if let Ok(lines) = read_lines("./inputs/input-d8.txt") {
        let mut forest: Vec<Vec<i32>> = vec![];

        for line_res in lines {
            if let Ok(line) = line_res {
                forest.push(
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .map(|u| u as i32)
                        .collect::<Vec<i32>>(),
                );
            }
        }

        let forest_size_y = forest.len();
        let forest_size_x = forest[0].len();

        let mut trees: HashSet<(i32, i32)> = HashSet::new();

        for i in 0..forest_size_y {
            let mut max_l2r = -1;
            let mut max_r2l = -1;

            for j in 0..forest_size_x {
                // left -> right
                if forest[i][j] > max_l2r {
                    trees.insert((i as i32 , j as i32));
                    max_l2r = forest[i][j];
                }
                // right -> left
                if forest[i][forest_size_x - j - 1] > max_r2l {
                    trees.insert((i as i32, (forest_size_x - j) as i32 - 1));
                    max_r2l = forest[i][forest_size_x - j - 1];
                }

            }
        }

        for i in 0..forest_size_x {
            let mut max_t2b = -1;

            let mut max_b2t = -1;

            for j in 0..forest_size_y {
                // left -> right
                if forest[j][i] > max_t2b {
                    trees.insert((j as i32 , i as i32));
                    max_t2b = forest[j][i];
                }
                // right -> left
                if forest[forest_size_x - j - 1][i] > max_b2t {
                    trees.insert(((forest_size_x - j) as i32 - 1, i as i32));
                    max_b2t = forest[forest_size_x - j - 1][i];
                }
            }
        }

        let mut scores = vec![];
        for tree in trees {
            let (x, y) : (usize, usize) = (tree.0 as usize, tree.1 as usize);
            let h = forest[x][y] as i32;

            let mut visibilities:Vec<i32>  = vec![];

            let mut v = 0;
            for i in (x+1)..forest_size_x {
                v += 1;
                if h > forest[i][y] {
                } else {
                    break;
                }
            }
            visibilities.push(v);

            v = 0;
            for i in 0..x {
                let idx = x - i - 1;
                v += 1;
                if h > forest[idx][y] {
                } else {
                    break;
                }
            }
            visibilities.push(v);

            v = 0;
            for i in (y+1)..forest_size_y {
                v += 1;
                if h > forest[x][i] {
                } else {
                    break;
                }
            }
            visibilities.push(v);

            v = 0;
            for i in 0..y {
                let idx = y - i - 1;
                v += 1;
                if h > forest[x][idx] {
                } else {
                    break;
                }
            }
            visibilities.push(v);
            scores.push(visibilities.iter().product::<i32>());
        }
        println!("part2 {}", scores.iter().max().unwrap())
    }
}
