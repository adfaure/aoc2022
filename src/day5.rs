use crate::helpers::read_lines;

fn get_col(col: usize, line: &String) -> char {
    let item: char = line.as_bytes()[(4 * (col - 1)) + 1] as char;
    item
}

pub fn day5() {
    if let Ok(lines) = read_lines("./inputs/input-d5.txt") {
        let mut parse_stacks = true;
        let stacks: Vec<Vec<char>> = vec![];

        let result = lines
            .map(|l| l.unwrap())
            .fold(stacks, |mut stacks, content| {
                if content == "" {
                    parse_stacks = false;
                    return stacks
                        .into_iter()
                        .map(|mut s| {
                            s.reverse();
                            s
                        })
                        .collect();
                }

                if parse_stacks {
                    let number_of_stacks = 1 + content.len() / 4;
                    // init stacks if not done yet

                    for i in 1..=number_of_stacks {
                        if stacks.len() < number_of_stacks {
                            stacks.push(vec![]);
                        }

                        let item = get_col(i, &content);
                        if item != ' ' && !('1'..='9').contains(&item) {
                            stacks[i - 1].push(item);
                        }
                    }
                } else {
                    // actions
                    let action: Vec<usize> = content
                        .split(" ")
                        .map(|w| w.parse::<usize>())
                        .filter(|opt| opt.is_ok())
                        .map(|opt| opt.unwrap())
                        .collect();

                    for _ in 0..action[0] {
                        let item = stacks[action[1] - 1].pop();
                        stacks[action[2] - 1].push(item.unwrap());
                    }
                }
                stacks
            })
            .into_iter()
            .map(|mut stack| stack.pop().unwrap())
            .collect::<String>();
        println!("result: {}", result);
    }

    if let Ok(lines) = read_lines("./inputs/input-d5.txt") {
        let mut parse_stacks = true;
        let stacks: Vec<Vec<char>> = vec![];

        let result = lines
            .map(|l| l.unwrap())
            .fold(stacks, |mut stacks, content| {
                if content == "" {
                    parse_stacks = false;
                    return stacks
                        .into_iter()
                        .map(|mut s| {
                            s.reverse();
                            s
                        })
                        .collect();
                }

                if parse_stacks {
                    let number_of_stacks = 1 + content.len() / 4;
                    // init stacks if not done yet

                    for i in 1..=number_of_stacks {
                        if stacks.len() < number_of_stacks {
                            stacks.push(vec![]);
                        }

                        let item = get_col(i, &content);
                        if item != ' ' && !('1'..='9').contains(&item) {
                            stacks[i - 1].push(item);
                        }
                    }
                } else {
                    // actions
                    let action: Vec<usize> = content
                        .split(" ")
                        .map(|w| w.parse::<usize>())
                        .filter(|opt| opt.is_ok())
                        .map(|opt| opt.unwrap())
                        .collect();

                    let mut out_stack = stacks[action[1] - 1].clone();
                    let (s, items) =
                        out_stack.split_at_mut(stacks[action[1] - 1].len() - action[0]);

                    stacks[action[2] - 1].append(&mut items.to_vec());
                    stacks[action[1] - 1] = s.to_vec();
                }
                stacks
            })
            .into_iter()
            .map(|mut stack| stack.pop().unwrap())
            .collect::<String>();
        println!("result part2: {}", result);
    }
}
