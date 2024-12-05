use std::fs;
use std::iter::zip;
use std::cmp;
use regex::Regex;

// If part one and two are very similar I only kept part two

fn main() {
    println!("Day one part one: {}", day_one_part_one());
    println!("Day one part two: {}", day_one_part_two());
    println!("Day two: {}", day_two());
    println!("Day three: {}", day_three());
}

fn day_one_part_one() -> i32 {
    let file_contents = fs::read_to_string("input.txt").expect("Error reading input file");

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
    let file_contents = fs::read_to_string("input.txt").expect("Error reading input file");

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

// DAY TWO
fn day_two() -> i32 {
    let file_contents = fs::read_to_string("input.txt").expect("Error reading input file");
    let mut safe_count: i32 = 0;

    for row in file_contents.lines() {
        let current_row: Vec<i32> = row.split_whitespace().filter_map(|n| n.parse::<i32>().ok()).collect();
        if process_numbers(current_row) {
            safe_count += 1;
        }
    }

    return safe_count;
}

fn is_safe(row: &[i32]) -> bool {
    if row.len() == 1 {
        return true;
    } 

    let mut increasing = false;
    let mut decreasing = false;

    for i in 0..row.len() - 1 {
        let diff = (row[i + 1] - row[i]).abs();
        if diff < 1 || diff > 3 {
            return false;
        } 
        if row[i + 1] > row[i] {
            increasing = true;
        } else {
            decreasing = true;
        }
    }
    return increasing != decreasing;
}

fn is_safe_after_remove(row: &[i32]) -> bool {
    for i in 0..row.len() {
        // will need a modified row for each index remove
        let mut modified_row = Vec::<i32>::with_capacity(row.len() - 1);
        modified_row.extend_from_slice(&row[..i]);
        modified_row.extend_from_slice(&row[i + 1..]);
        if is_safe(&modified_row) {
            return true;
        }
    }
    return false;
}

fn process_numbers(row: Vec<i32>) -> bool {
    if is_safe(&row) {
        return true;
    }
    return is_safe_after_remove(&row);
}

// DAY THREE
fn day_three() -> i32 {
    let file_contents = fs::read_to_string("day_3.txt").expect("Error reading input file");
    let mut sum: i32 = 0;
    let mut calc = true;

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    for cap in re.captures_iter(&file_contents) {

        let can_calc = cap.get(0).unwrap().as_str();
        if can_calc == "do()" {
            calc = true;
        } else if can_calc == "don't()" {
            calc = false;
        }

        if let Some(x) = cap.get(1) {
           if let Some(y) = cap.get(2) {
               let x = x.as_str().parse::<i32>().unwrap();
               let y = y.as_str().parse::<i32>().unwrap();
               if calc {
                   sum += x * y;
               } 
           } 
        }
    }
    return sum;
}


