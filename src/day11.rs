use crate::helpers::read_lines;
use core::cell::Cell;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

struct Monkey {
    id: i64,
    items: RefCell<VecDeque<i64>>,
    items_prime: RefCell<VecDeque<HashMap<i64, i64>>>,
    operator: String,
    test: i64,
    actions: (i64, i64),
    total: Cell<i64>,
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkey = Monkey {
            id: 0,
            items: RefCell::new(VecDeque::new()),
            items_prime: RefCell::new(VecDeque::new()),
            operator: String::new(),
            test: 0,
            actions: (0, 0),
            total: Cell::new(0),
        };

        for (line, content) in s.split("\n").enumerate() {
            match line {
                0 => {
                    monkey.id = content.split(" ").collect::<Vec<&str>>()[1]
                        .replace(":", "")
                        .parse::<i64>()
                        .unwrap();
                }
                1 => {
                    monkey.items = RefCell::new(
                        content
                            .chars()
                            .skip_while(|&ch| ch != ':')
                            .skip(1)
                            .collect::<String>()
                            .split(",")
                            .map(|s| s.trim())
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect::<VecDeque<i64>>(),
                    );
                }
                2 => {
                    monkey.operator = content.to_string();
                }
                3 => {
                    let lex = content.trim().split(" ").collect::<Vec<&str>>();
                    let modu = lex[3].parse::<i64>().unwrap();
                    monkey.test = modu;
                }
                4 => {
                    monkey.actions.0 = content.trim().split(" ").collect::<Vec<&str>>()[5]
                        .parse::<i64>()
                        .unwrap();
                }
                5 => {
                    monkey.actions.1 = content.trim().split(" ").collect::<Vec<&str>>()[5]
                        .parse::<i64>()
                        .unwrap();
                }
                6 => {
                    // empty line is okay
                }
                _ => {
                    panic!("cannot parse monkey: {:?}", (line, content))
                }
            }
        }
        Ok(monkey)
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monkey {} -> total {}:\n", self.id, self.total.get());
        write!(f, "  items :");
        for item in self.items.borrow().iter() {
            write!(f, "{},", item);
        }
        write!(f, "\n");
        write!(f, "  Test :{}\n", self.test);
        write!(f, "     if true: throw to {}\n", self.actions.0);
        write!(f, "     if false: throw to {}\n", self.actions.1)
    }
}

pub fn operation(operator: &String, old: &i64, modulo: i64) -> i64 {
    let lex = operator.trim().split(" ").collect::<Vec<&str>>();

    let v1 = match lex[3] {
        "old" => *old,
        number => number.parse::<i64>().unwrap(),
    };

    let v2 = match lex[5] {
        "old" => *old,
        number => number.parse::<i64>().unwrap(),
    };

    let res = match lex[4] {
        "+" => (v1) + (v2),
        "*" => (v1) * (v2),
        _ => {
            panic!("unknown operation {:?}", lex[1]);
        }
    };

    res
}

pub fn throw(test: String, value: &i64) -> i64 {
    let lex = test.split(" ").collect::<Vec<&str>>();
    let monkey = lex[5].parse::<i64>().unwrap();
    monkey
}

impl Monkey {
    pub fn add_item(&self, item: i64) {
        self.items.borrow_mut().push_back(item);
    }

    pub fn do_turn2(&self, monkeys: &HashMap<i64, RefCell<Monkey>>) -> i64 {
        println!("Monkey {}:", self.id);
        let result = self.items.borrow().len();
        while let Some(item) = self.items.borrow_mut().pop_front() {
            println!("  Monkey inspects an item {:?}", item);
            let new = operation(&self.operator, &item, self.test);

            let div: i64 = monkeys.iter().map(|(k, v)| v.borrow().test).product();
            let relief = new % div;

            println!("    bored with item. new worry {:?}", relief);

            if relief % self.test == 0 {
                println!(
                    "    item with worry if {} throwed to {}",
                    relief, self.actions.0
                );
                monkeys
                    .get(&self.actions.0)
                    .unwrap()
                    .borrow_mut()
                    .add_item(relief);
            } else {
                println!(
                    "    item with worry if {} throwed to {}",
                    relief, self.actions.1
                );
                monkeys
                    .get(&self.actions.1)
                    .unwrap()
                    .borrow_mut()
                    .add_item(relief);
            }
        }
        println!("");
        self.total.set(self.total.get() + result as i64);
        result as i64
    }

    pub fn do_turn(&self, monkeys: &HashMap<i64, RefCell<Monkey>>) -> i64 {
        println!("Monkey {}:", self.id);
        let result = self.items.borrow().len();
        while let Some(item) = self.items.borrow_mut().pop_front() {
            println!("  Monkey inspects an item {}", item);
            let new = operation(&self.operator, &item, self.test);
            let relief = new / 3;
            println!("    bored with item. new worry {}", relief);

            if relief % self.test == 0 {
                println!(
                    "    item with worry if {} throwed to {}",
                    relief, self.actions.0
                );
                monkeys
                    .get(&self.actions.0)
                    .unwrap()
                    .borrow_mut()
                    .add_item(relief);
            } else {
                println!(
                    "    item with worry if {} throwed to {}",
                    relief, self.actions.1
                );
                monkeys
                    .get(&self.actions.1)
                    .unwrap()
                    .borrow_mut()
                    .add_item(relief);
            }
        }
        println!("");
        self.total.set(self.total.get() + result as i64);
        result as i64
    }
}

pub fn day11() {
    if let Ok(lines) = read_lines("./inputs/input-d11.txt") {
        let mut monkeys = lines
            .map(|res_line| res_line.unwrap() as String)
            .batching(|it| match it.next() {
                None => None,
                Some(l) => Some(
                    it.fold_while(format!("{}\n", l), |acc, line| match line.as_str() {
                        "" => Done(acc),
                        _ => Continue(format!("{}{}\n", acc, line)),
                    })
                    .into_inner(),
                ),
            })
            .map(|text| Monkey::from_str(&text).unwrap())
            .map(|monkey| (monkey.id, RefCell::new(monkey)))
            .collect::<HashMap<i64, RefCell<Monkey>>>();

        let nb_turn = 20;

        for i in 0..nb_turn {
            for mid in 0..monkeys.len() {
                let m = monkeys.get(&(mid as i64)).unwrap();
                println!("monkey: {:?}", m.borrow());
                m.borrow().do_turn(&monkeys);
                println!("monkey: {:?}", m.borrow());
            }
            println!("print turn {}", i);
            for mid in 0..monkeys.len() {
                let m = monkeys.get(&(mid as i64)).unwrap();
                println!("monkey: {:?}", m.borrow());
            }
        }

        println!("{:?}", monkeys);
        let mut worries = monkeys
            .iter()
            .map(|(_, m)| m.borrow().total.get())
            .collect::<Vec<i64>>();
        worries.sort();
        worries.reverse();

        println!("{}", worries.iter().take(2).product::<i64>());
    }

    if let Ok(lines) = read_lines("./inputs/input-d11.txt") {
        let mut monkeys = lines
            .map(|res_line| res_line.unwrap() as String)
            .batching(|it| match it.next() {
                None => None,
                Some(l) => Some(
                    it.fold_while(format!("{}\n", l), |acc, line| match line.as_str() {
                        "" => Done(acc),
                        _ => Continue(format!("{}{}\n", acc, line)),
                    })
                    .into_inner(),
                ),
            })
            .map(|text| Monkey::from_str(&text).unwrap())
            .map(|monkey| (monkey.id, RefCell::new(monkey)))
            .collect::<HashMap<i64, RefCell<Monkey>>>();

        let nb_turn = 10000;

        for i in 0..nb_turn {
            for mid in 0..monkeys.len() {
                let m = monkeys.get(&(mid as i64)).unwrap();
                println!("monkey: {:?}", m.borrow());
                m.borrow().do_turn2(&monkeys);
                println!("monkey: {:?}", m.borrow());
            }
            println!("print turn {}", i);
            for mid in 0..monkeys.len() {
                let m = monkeys.get(&(mid as i64)).unwrap();
                println!("monkey: {:?}", m.borrow());
            }
        }

        println!("{:?}", monkeys);
        let mut worries = monkeys
            .iter()
            .map(|(_, m)| m.borrow().total.get())
            .collect::<Vec<i64>>();

        worries.sort();
        worries.reverse();

        println!("{}", worries.iter().take(2).product::<i64>());
    }
}
