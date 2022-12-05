use std::{env, fs};
use std::borrow::Borrow;
use std::collections::HashSet;

struct Range {
    lower_bound: i32,
    upper_bound: i32
}

fn main() {
    let args : Vec<String> = env::args().skip(1).collect();

    let file_contents = fs::read_to_string(&args[0]).expect("Could not read file");

    println!("File contents lines: {}", file_contents.split("\n").count());

    let overlap_count = file_contents.trim().split("\n").map(|line| {
        let ranges = line.split(",").map(|range| {
            let pair: Vec<_> = range.split("-").collect();
            assert_eq!(2, pair.len());
            return Range {
                lower_bound: pair[0].parse::<i32>().unwrap(),
                upper_bound: pair[1].parse::<i32>().unwrap(),
            }
        }).collect::<Vec<_>>();

        let range_1 = ranges[0].borrow();
        let range_2 = ranges[1].borrow();

        return range_1.lower_bound >= range_2.lower_bound && range_1.upper_bound <= range_2.upper_bound ||
            range_2.lower_bound >= range_1.lower_bound && range_2.upper_bound <= range_1.upper_bound;
    }).filter(|pred|{pred == &true}).count();

    println!("{:?}", overlap_count);
}
