use crate::helpers::read_lines;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn day10() {
    if let Ok(lines) = read_lines("./inputs/input-d10.txt") {
        let mut regs: Vec<i32> = vec![];
        let mut last_val = 1 as i32;

        for line_res in lines {
            if let Ok(line) = line_res {
                let instr = line.split(" ").collect::<Vec<&str>>();

                match instr[0] {
                    "noop" => {
                        regs.push(last_val);
                    }
                    "addx" => {
                        let val = instr[1].parse::<i32>().unwrap();
                        regs.push(last_val);
                        regs.push(last_val);

                        last_val = last_val + val;
                    }
                    _ => {}
                }
            }
        }

        for (i, v) in regs.iter().enumerate() {
            // println!("{}:{}", i, v);
        }

        let wanted_cycles: HashSet<i32> = HashSet::from_iter(vec![20, 60, 100, 140, 180, 220]);
        println!(
            "{:?}",
            regs.iter()
                .enumerate()
                .map(|(c, v)| (c + 1, v))
                // .inspect(|c| println!("{:?}", c))
                .filter(|c| wanted_cycles.contains(&(c.0 as i32)))
                .map(|(c, r)| c as i32 * r)
                .sum::<i32>()
        );

        let screen: String = regs
            .iter()
            .enumerate()
            .map(|(c, v)| (c + 1, v))
            // .inspect(|c| println!("{:?}", c))
            .map(|(c, p)| {
                let shift: i32 = c as i32;
                println!("pos:{}  mod:{:?}, reg:{:?} -- sprite{:?}", shift, (shift - 1) % 40, p, (p..=&(p + 2)));

                if (p..=&(p + 2)).contains(&&(1 + (shift -1) % 40)) {
                    return (c, "#");
                } else {
                    return (c, ".");
                }
            })
            .fold(String::new(), |acc, (c, v)| {
                // println!("rec: {}", c);
                if c % 40 == 0 {
                    return format!("{}{}\n", acc, v);
                } else {
                    return format!("{}{}", acc, v);
                }
            });

        println!("screen: \n{}", screen);
    }
}
// ...............#
