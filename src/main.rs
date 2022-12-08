use std::env;
extern crate interval_set;

mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod helpers;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;
use day8::day8;
use day9::day9;

use crate::helpers::read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "1" {
        day1();
    } else if args[1] == "2" {
        day2();
    } else if args[1] == "3" {
        day3();
    } else if args[1] == "4" {
        day4();
    } else if args[1] == "5" {
        day5();
    } else if args[1] == "6" {
        day6();
    } else if args[1] == "7" {
        day7();
    } else if args[1] == "8" {
        day8();
    } else if args[1] == "9" {
        day9();
    }

}

fn day2() {
    if let Ok(lines) = read_lines("./input-d2.txt") {
        let mut sum = 0;
        let mut sump2 = 0;
        for line in lines {
            if let Ok(content) = line {
                let ax = content.split(" ").collect::<Vec<&str>>();

                let mut sumline = 0;
                let col1 = ax[0];
                let mut col2 = ax[1];

                match (col1, col2) {
                    (_, "X") => {
                        sumline += 1;
                        sump2 += 0;
                        col2 = "A";
                        match col1 {
                            "A" => {
                                // faire C
                                sump2 += 3;
                            }
                            "B" => {
                                // faire A
                                sump2 += 1;
                            }
                            "C" => {
                                // faire B
                                sump2 += 2;
                            }
                            _ => {}
                        }
                    }
                    (_, "Y") => {
                        sump2 += 3;
                        sumline += 2;
                        col2 = "B";
                        match col1 {
                            "A" => {
                                // faire C
                                sump2 += 1;
                            }
                            "B" => {
                                // faire A
                                sump2 += 2;
                            }
                            "C" => {
                                // faire B
                                sump2 += 3;
                            }
                            _ => {}
                        }
                    }
                    (_, "Z") => {
                        sump2 += 6;
                        sumline += 3;
                        col2 = "C";
                        match col1 {
                            "A" => {
                                // faire C
                                sump2 += 2;
                            }
                            "B" => {
                                // faire A
                                sump2 += 3;
                            }
                            "C" => {
                                // faire B
                                sump2 += 1;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

                println!("battle: {} {}", col1, col2);
                if col1 == col2 {
                    sumline += 3;
                } else if (col1 == "A" && col2 == "C")
                    || (col1 == "B" && col2 == "A")
                    || (col1 == "C" && col2 == "B")
                {
                    sumline += 0;
                } else {
                    sumline += 6;
                }
                println!("final score: {:?} de la ligne", sumline);
                sum += sumline;
            }
        }
        println!("final score: {:?}", sum);
        println!("final score p2: {:?}", sump2);
    }
}

fn day1() {
    println!("Hello, world!");
    if let Ok(lines) = read_lines("./input-d1.txt") {
        let mut sums = vec![];
        let mut sum = 0;
        let mut max = 0;
        for line in lines {
            if let Ok(content) = line {
                if content == "" {
                    println!("{}", content);
                    if max < sum {
                        max = sum;
                    }
                    sums.push(sum);
                    sum = 0;
                } else {
                    let my_int = content.parse::<i32>().unwrap();
                    sum += my_int;
                    println!("prout {}", content);
                }
            }
        }

        sums.sort();
        sums.reverse();
        println!("result: {:?}", sums);
        let sum3 = sums.into_iter().take(3).sum::<i32>();
        println!("result: {}", sum3);
    }
}
