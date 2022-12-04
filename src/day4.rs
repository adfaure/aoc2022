use crate::helpers::read_lines;
use interval_set::{IntervalSet, ToIntervalSet};


pub fn day4() {
    if let Ok(lines) = read_lines("./inputs/input-d4.txt") {
       let respart1 = lines.map(|res| res.unwrap()).map(|line: String| {

           let intervals = line.split(",").map(|i| {
               let iv = i.split("-").map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();
               vec![(iv[0], iv[1])].to_interval_set()
           }).collect::<Vec<IntervalSet>>();

           let dif1 = intervals[0].clone().difference(intervals[1].clone());
           let dif2 = intervals[1].clone().difference(intervals[0].clone());

           let sim = intervals[0].clone().symetric_difference(intervals[1].clone());

           if (dif1 == sim) | (dif2 == sim) {
                return 1
           }
           return 0
       }).sum::<i32>();
       println!("res part2: {}", respart1);
    }

    if let Ok(lines) = read_lines("./inputs/input-d4.txt") {
       let respart2 = lines.map(|res| res.unwrap()).map(|line: String| {

           let intervals = line.split(",").map(|i| {
               let iv = i.split("-").map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();
               vec![(iv[0], iv[1])].to_interval_set()
           }).collect::<Vec<IntervalSet>>();

           let union = intervals[0].clone().intersection(intervals[1].clone());
           if union.is_empty() {
               return 0
           }
           return 1

       }).sum::<i32>();

       println!("res part2: {}", respart2);
    }
}
