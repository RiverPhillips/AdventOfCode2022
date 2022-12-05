extern crate core;

use std::{env, fs};
use std::collections::HashSet;

fn main() {
    let args : Vec<String> = env::args().skip(1).collect();

    let file_contents = fs::read_to_string(&args[0]).expect("Could not read file");

    let rows: Vec<_>= file_contents.trim().split("\n").collect();

    let mut badge_priority: Vec<i32> = Vec::new();

    let mut i = 0;
    while i < rows.len() {
        let mut sets: Vec<HashSet<char>> = Vec::new();
        sets.push(rows[i].chars().collect());
        sets.push(rows[i+1].chars().collect());
        sets.push(rows[i+2].chars().collect());

        let badge = sets[0].iter().filter(|c| sets[1].contains(c) && sets[2].contains(c)).collect::<Vec<&char>>()[0];

        let prio = get_priority(badge);
        badge_priority.push(prio);

        i = i + 3;
    }



    let prio: i32 = badge_priority.iter().sum();
    println!("{:?} ", prio);

}

fn get_priority(x: &char) -> i32 {
    match x {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("Unknown item")
    }
}
