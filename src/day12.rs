use crate::helpers::read_lines;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost).then_with(|| {
            self.position
                .0
                .cmp(&other.position.0)
                .then_with(|| self.position.1.cmp(&other.position.1))
        })
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(
    elevations: &Vec<Vec<i8>>,
    start: (usize, usize),
    target: (usize, usize),
) -> Option<usize> {
    // println!("start {:?}, end: {:?}", start, target);

    let mut heap = BinaryHeap::new();
    let mut dist: Vec<Vec<_>> = elevations
        .iter()
        .map(|line| vec![usize::MAX; line.len()])
        .collect();

    let bounds = (elevations[0].len() as i32, elevations.len() as i32);

    dist[start.1][start.0] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == target {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position.1][position.0] {
            continue;
        }

        let x = position.0 as i32;
        let y = position.1 as i32;

        for j in (y - 1)..=(y + 1) {
            for i in (x - 1)..=(x + 1) {
                if !(i == x && j == y)
                        && (i == x || j == y) // diagonal are forbiden
                        && (i >= 0)
                        && (j >= 0)
                        && (i < bounds.0)
                        && (j < bounds.1)
                {
                    let next = State {
                        cost: cost + 1,
                        position: (i as usize, j as usize),
                    };

                    if next.cost < dist[next.position.1][next.position.0]
                        && ((elevations[j as usize][i as usize]
                            - elevations[position.1][position.0])
                            < 2)
                    {
                        heap.push(next);
                        dist[next.position.1][next.position.0] = next.cost;
                    }
                }
            }
        }
    }

    None
}

pub fn day12() {
    if let Ok(res_lines) = read_lines("./inputs/input-d12.txt") {
        let lines = res_lines.map(|r| r.unwrap()).collect::<Vec<String>>();

        let elevations = lines
            .iter()
            .map(|s| {
                s.chars().map(|c| match c {
                'S' => 'a',
                'E' => 'z',
                 c => c
            } as i8 - 'a' as i8).collect()
            })
            .collect::<Vec<Vec<i8>>>();

        let start_end: Vec<(usize, usize, char)> = lines
            .iter()
            .enumerate()
            .map(|(idc, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(i, c)| c == &'S' || c == &'E')
                    .map(|(i, c)| (i, idc, c))
                    .collect::<Vec<(usize, usize, char)>>()
            })
            .flatten()
            .collect();

        let start = match start_end[0].2 {
            'S' => (start_end[0].0, start_end[0].1),
            _ => (start_end[1].0, start_end[1].1),
        };
        let target = match start_end[0].2 {
            'E' => (start_end[0].0, start_end[0].1),
            _ => (start_end[1].0, start_end[1].1),
        };

        println!("target: {:?}", target);
        println!("{:?}", solve(&elevations, start, target));
    }

    if let Ok(res_lines) = read_lines("./inputs/input-d12.txt") {
        let lines = res_lines.map(|r| r.unwrap()).collect::<Vec<String>>();

        let elevations = lines
            .iter()
            .map(|s| {
                s.chars().map(|c| match c {
                'S' => 'a',
                'E' => 'z',
                 c => c
            } as i8 - 'a' as i8).collect()
            })
            .collect::<Vec<Vec<i8>>>();

        let start_end: Vec<(usize, usize, char)> = lines
            .iter()
            .enumerate()
            .map(|(idc, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(i, c)| c == &'S' || c == &'E' || c == &'a')
                    .map(|(i, c)| (i, idc, c))
                    .collect::<Vec<(usize, usize, char)>>()
            })
            .flatten()
            .collect();

        let target = start_end
            .iter()
            .find(|e| e.2 == 'E')
            .map(|(x, y, _)| (*x, *y))
            .unwrap();

        println!("target: {:?}", target);
        let starts = start_end
            .iter()
            .filter(|(x, y, c)| c != &'E')
            .map(|(x, y, _)| solve(&elevations, (*x, *y), target))
            .filter_map(|r| r)
            .min();

        println!("{:?}", starts);
    }
}
