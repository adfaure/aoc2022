use itertools::Itertools;
use regex::Regex;
use std::iter::repeat_n;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    solve_p1()
}

type Grid<T> = Vec<Vec<T>>;

fn solve_p1() -> std::io::Result<()> {
    let re = Regex::new(r"\d+|[LR]").unwrap();

    let grid: Grid<char> = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .take_while(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let in_path = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .skip_while(|l| !l.is_empty())
        .nth(1);

    let path = re
        .find_iter(&in_path.unwrap())
        .map(|e| e.as_str().to_string())
        .collect_vec();

    let max_size = grid.iter().map(|e| e.len()).max().unwrap();
    println!();

    let padded_grid = grid
        .clone()
        .into_iter()
        .map(|line| {
            if line.len() < max_size - 1 {
                let mut padded = line.clone();
                padded.extend(std::iter::repeat(' ').take(max_size - line.len()));
                return padded;
            }
            line
        })
        .collect_vec();

    let mut dir = (1 as i32, 0 as i32);
    let mut pos = (8 as i32, 0 as i32);
    let mut path_taken = vec![];

    for cur in path {
        // println!("must move {}", cur);
        if cur == "R" {
            dir = rotate(dir, false);
        } else if cur == "L" {
            dir = rotate(dir, true);
        } else {
            let mut nb_step = cur.parse::<i32>().unwrap();
            while nb_step != 0 {
                path_taken.push((pos, dir));

                let next_pos = find_neighbour(&padded_grid, pos, dir);
                if padded_grid[next_pos.1 as usize][next_pos.0 as usize] == '#' {
                    nb_step = 0;
                } else {
                    pos = next_pos;
                    nb_step -= 1;
                }
            }
        }
    }

    // show_grid_and_path(&padded_grid, &path_taken);

    let pos_score = 1000 * (pos.1 + 1) + 4 * (pos.0 + 1);
    let dir_score = match dir {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    };

    println!("p1: {}", pos_score + dir_score);

    Ok(())
}

fn find_neighbour(grid: &[Vec<char>], pos: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    let mut found = false;

    let mut next_pos = pos;
    while !found {
        next_pos = (next_pos.0 + dir.0, next_pos.1 + dir.1);
        if next_pos.0 < 0 || next_pos.0 >= grid[0].len() as i32 {
            next_pos.0 = next_pos.0.rem_euclid(grid[0].len() as i32);
        } else if next_pos.1 < 0 || next_pos.1 >= grid.len() as i32 {
            next_pos.1 = next_pos.1.rem_euclid(grid.len() as i32);
        }

        if grid[next_pos.1 as usize][next_pos.0 as usize] != ' ' {
            found = true;
        }
    }

    next_pos
}

fn rotate<T>(v: (T, T), clockwise: bool) -> (T, T)
where
    T: std::ops::Neg<Output = T> + Copy,
{
    if clockwise { (v.1, -v.0) } else { (-v.1, v.0) }
}

fn solve_p2() -> std::io::Result<()> {
    let re = Regex::new(r"\d+|[LR]").unwrap();

    let grid: Grid<char> = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .take_while(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let in_path = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .skip_while(|l| !l.is_empty())
        .nth(1);

    println!("grid: {:?}", grid);

    let path = re
        .find_iter(&in_path.unwrap())
        .map(|e| e.as_str().to_string())
        .collect_vec();

    println!("path: {:?}", path);

    let max_size = grid.iter().map(|e| e.len()).max().unwrap();
    println!();

    let padded_grid = grid
        .clone()
        .into_iter()
        .map(|line| {
            if line.len() < max_size - 1 {
                let mut padded = line.clone();
                padded.extend(std::iter::repeat(' ').take(max_size - line.len()));
                return padded;
            }
            line
        })
        .collect_vec();

    println!("max {max_size:?}");
    let start_pos = (0, padded_grid[0].iter().position(|c| *c != ' ').unwrap());
    println!("start_pos: {:?}", start_pos);

    show_grid(&padded_grid);
    println!("{:?}", padded_grid);

    let dimensions: (usize, usize) = (4, 3);
    // section dimensions
    let sdim: (usize, usize) = (4, 4);

    let patron_dimensions: (usize, usize) = (padded_grid[0].len(), padded_grid.len());
    println!("padded dims: {:?}", patron_dimensions);

    let mut sections: Grid<Grid<char>> = vec![];

    for y in 0..dimensions.1 {
        let mut line = vec![];
        for x in 0..dimensions.0 {
            let xs = (
                x * sdim.0,
                x * sdim.0 + (patron_dimensions.0 / dimensions.0),
            );
            let ys = (
                y * sdim.1,
                y * sdim.1 + (patron_dimensions.1 / dimensions.1),
            );

            line.push(grid_from(&padded_grid, xs.0, xs.1, ys.0, ys.1));
        }
        sections.push(line);
    }

    Ok(())
}

fn grid_from(
    grid: &Grid<char>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
) -> Grid<char> {
    let mut result = vec![];

    #[allow(clippy::needless_range_loop)]
    for y in y_min..y_max {
        let mut line = vec![];
        for x in x_min..x_max {
            line.push(grid[y][x])
        }
        result.push(line);
    }

    result
}

fn show_grid_and_path(grid: &[Vec<char>], path: &[((i32, i32), (i32, i32))]) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if let Some(pp) = path
                .iter()
                .find(|pp| pp.0.0 == x as i32 && pp.0.1 == y as i32)
            {
                match pp.1 {
                    (0, 1) => print!("V"),
                    (0, -1) => print!("^"),
                    (1, 0) => print!(">"),
                    (-1, 0) => print!("<"),
                    _ => unreachable!(),
                }
            } else if grid[y][x] == ' ' {
                print!(" ");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }
}

fn show_grid(grid: &[Vec<char>]) {
    for line in grid {
        for c in line {
            if *c == ' ' {
                print!(" ");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}
