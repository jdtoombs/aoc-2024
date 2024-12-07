use std::fs;
use std::cmp;
use std::iter::Iterator;
use std::iter::zip;

fn main() {
    print!("Day 1 Part 1: {}\n", day_one_part_one());
    print!("Day 1 Part 2: {}\n", day_one_part_two());
}

fn day_one_part_one() -> i32 {
    let file_contents = fs::read_to_string("data/day_1.txt").expect("Error reading input file");

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut diff: i32 = 0;

    for line in file_contents.lines() {
        let mut numbers = line.split_whitespace().filter_map(|n| n.parse::<i32>().ok());
        if let Some(first_number) = numbers.next() {
            left.push(first_number);
        }
        if let Some(second_number) = numbers.next() {
            right.push(second_number);
        }
    }

    left.sort();
    right.sort();

    let diffs = zip(left.clone(), right.clone());

    for (l, r) in diffs {
        let max = cmp::max(l, r);
        let min = cmp::min(l, r);
        let curr_diff = max - min;
        diff += curr_diff;
    }

    return diff;
}

fn day_one_part_two() -> i32 {
    let file_contents = fs::read_to_string("data/day_1.txt").expect("Error reading input file");

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut total: i32 = 0;

    for line in file_contents.lines() {
        let mut numbers = line.split_whitespace().filter_map(|n| n.parse::<i32>().ok());
        if let Some(first_number) = numbers.next() {
            left.push(first_number);
        }
        if let Some(second_number) = numbers.next() {
            right.push(second_number);
        }
    }

    for l in left.iter() {
        let multiple: i32 = right.iter().filter(|&&e| e == *l).count() as i32; total += multiple * *l; }
    return total;
}

