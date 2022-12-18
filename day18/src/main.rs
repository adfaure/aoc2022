use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn find_faces(x: i32, y: i32, z: i32) -> Vec<(Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>)> {
    let dots = [x, y, z]
        .iter()
        .map(|i| *i..=(i + 1))
        .multi_cartesian_product()
        .collect_vec();

    [x, y, z]
        .iter()
        .enumerate()
        .flat_map(|(pos, cord)| {
            dots.clone()
                .into_iter()
                .filter(move |dot| dot[pos] == *cord)
                .chain(
                    dots.clone()
                        .into_iter()
                        .filter(move |dot| dot[pos] == *cord + 1),
                )
        })
        .tuples::<(_, _, _, _)>()
        .collect_vec()
}

pub fn edges(x: i32, y: i32, z: i32) -> [(i32, i32, i32); 8] {
    [x, y, z]
        .iter()
        .map(|i| *i..=(i + 1))
        .multi_cartesian_product()
        .flatten()
        .tuples::<(i32, i32, i32)>()
        .collect_vec()
        .try_into()
        .unwrap()
}

pub fn steam(
    x: i32,
    y: i32,
    z: i32,
    cache: &HashSet<(i32, i32, i32)>,
) -> Vec<(i32, (i32, i32, i32))> {
    let mut seen = HashSet::new();
    let mut outside_faces = vec![];
    let mut file = vec![(x, y, z)];

    while let Some((x, y, z)) = file.pop() {
        // println!("fifo size {:?}", (x, y, z));
        for next_cube in vec![
            (0, (x - 1, y, z)),
            (1, (x + 1, y, z)),
            (2, (x, y - 1, z)),
            (3, (x, y + 1, z)),
            (4, (x, y, z - 1)),
            (5, (x, y, z + 1)),
        ]
        .into_iter()
        .filter(|(dir, e)| e.0 < 30 && e.1 < 30 && e.2 < 30 && e.0 >= -1 && e.1 >= -1 && e.2 >= -1)
        .filter_map(|e| seen.insert(e).then_some(e))
        {
            let (dir, next_pos) = next_cube;
            match cache.get(&next_pos) {
                None => {
            file.push(next_pos);
                }
                Some(_) => {
                    // println!("cube here: {:?}", next_cube);
                    outside_faces.push((dir, next_pos));
                }
            };
        }
    }

    println!("{:?}", outside_faces.len());
    outside_faces
}

fn main() -> std::io::Result<()> {
    let faces = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .flat_map(|l| {
            l.split(",")
                .map(|s| String::from(s))
                .collect::<Vec<String>>()
        })
        .filter_map(|s| s.parse::<i32>().ok())
        .tuples::<(_, _, _)>()
        .flat_map(|coord| find_faces(coord.0, coord.1, coord.2))
        .collect_vec();

    let mut faces_set = HashMap::new();

    for face in faces {
        if let Some(nb_time) = faces_set.get_mut(&face) {
            *nb_time += 1;
        } else {
            faces_set.insert(face, 1);
        }
    }

    println!(
        "total visible: {:?}",
        faces_set
            .iter()
            .filter_map(|(_, v)| (*v == 1).then_some(1))
            .sum::<i32>(),
    );

    let cubes = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .flat_map(|l| {
            l.split(",")
                .map(|s| String::from(s))
                .collect::<Vec<String>>()
        })
        .filter_map(|s| s.parse::<i32>().ok())
        .tuples::<(_, _, _)>()
        .collect::<HashSet<(_, _, _)>>();

    steam(0, 0, 0, &cubes);

    Ok(())
}
