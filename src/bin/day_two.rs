use std::fs;

fn main() {
    print!("Day 2: {}\n", day_two());
}

// just part two as they are similar
fn day_two() -> i32 {
    let file_contents = fs::read_to_string("data/day_2.txt").expect("Error reading input file");
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
