use crate::helpers::read_lines;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn day10() {
    if let Ok(lines) = read_lines("./inputs/input-d10.txt") {
        let mut cycles: i32 = 0;
        let mut regs: Vec<i32> = vec![];
        regs.push(1);

        for line_res in lines {
            if let Ok(line) = line_res {
                let instr = line.split(" ").collect::<Vec<&str>>();

                match instr[0] {
                    "noop" => {
                        cycles += 1;
                        regs.push(regs[regs.len() - 1]);
                    }
                    "addx" => {
                        let val = instr[1].parse::<i32>().unwrap();

                        regs.push(regs[regs.len() - 1]);
                        regs.push(regs[regs.len() - 1] + val);
                    }
                    _ => {}
                }
            }
        }

        for (i, v) in regs.iter().enumerate() {
            println!("{}:{}", i, v);
        }

        let wanted_cycles: HashSet<i32> = HashSet::from_iter(vec![20, 60, 100, 140, 180, 220]);
        println!(
            "{:?}",
            regs.iter()
                .enumerate()
                .filter(|c| wanted_cycles.contains(&(c.0 as i32)))
                .inspect(|v| println!("{:?}", v))
                .map(|(c, r)| c as i32 * r)
                .sum::<i32>()
        );

        let screen: String = regs
            .iter()
            .enumerate()
            .map(|(c, p)| {
                let shift: i32 = (c as i32+ 1);
                println!("{:?}:{:?}", shift, p);
                if (p..=&(p + 2)).contains(&&(shift % 40)) {
                    return (shift, "#");
                } else {
                    return (shift, ".");
                }
            })

            .fold(String::new(), |acc, (c, v)| {
                println!("rec: {}", c);
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
