use crate::helpers::read_lines;
use std::collections::HashMap;

pub fn day7() {
    if let Ok(lines) = read_lines("./inputs/input-d7.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut sizes: HashMap<String, usize> = HashMap::new();
        let mut current_dir = vec![];

        for line_res in lines {
            if let Ok(line) = line_res {
                let instructions = line.split(" ").collect::<Vec<&str>>();
                match instructions[0] {
                    "$" => match instructions[1] {
                        "cd" => {
                            if instructions[2] != ".." {
                                current_dir.push(String::from(instructions[2]));
                                let path = current_dir.join("/");
                                sizes.insert(String::from(path), 0);
                            } else {
                                current_dir.pop();
                            }
                        }
                        "ls" => {}
                        _ => {}
                    },
                    "dir" => {}
                    _ => {
                        let size: usize = instructions[0].parse::<usize>().unwrap();
                        let mut name = String::from("");
                        for dir in current_dir.iter() {
                            name = format!("{}{}", name, dir);
                            *sizes.get_mut(&name).unwrap() += size;
                            name = format!("{}/", name);
                        }
                    }
                }
            }
        }
        println!(
            "{:?}",
            sizes
                .iter()
                .filter(|(_, v)| **v <= (100000 as usize))
                .map(|(_, v)| v)
                .sum::<usize>()
        );

        let total_space = 70000000;
        let total_used_space = sizes.get("/").unwrap();
        let unused = total_space - total_used_space;
        let needed = 30000000;

        let need_to_free = needed - unused;
        println!("need to free {}", need_to_free);

        println!(
            "total: {}, used: {}, needed: {}",
            total_space, total_used_space, needed
        );
        println!(
            "test: {:?}",
            sizes
                .into_iter()
                .filter(|(_, v)| *v >= (need_to_free as usize))
                .fold(total_space, |acc, (_, v)| {
                    if acc > v {
                        v
                    } else {
                        acc
                    }
                })
        );
    }
}
