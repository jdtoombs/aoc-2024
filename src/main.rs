use std::{fs, usize};
use std::iter::zip;
use std::cmp;
use regex::Regex;

// If part one and two are very similar I only kept part two

fn main() {
    println!("Day one part one: {}", day_one_part_one());
    println!("Day one part two: {}", day_one_part_two());
    println!("Day two: {}", day_two());
    println!("Day three: {}", day_three());
    println!("Day four pt 1: {}", day_four());
    println!("Day four pt 2: {}", day_four_part_two());
}

fn day_one_part_one() -> i32 {
    let file_contents = fs::read_to_string("day_1.txt").expect("Error reading input file");

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
    let file_contents = fs::read_to_string("day_1.txt").expect("Error reading input file");

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
    let file_contents = fs::read_to_string("day_2.txt").expect("Error reading input file");
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


// DAY FOUR
fn day_four() -> i32 {
    let mut matrix: Vec<Vec<String>> = Vec::new();
    let mut word_count = 0;
    let file_contents = fs::read_to_string("day_4.txt").expect("Error reading day 4 input");
    for line in file_contents.lines() {
        let row: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        matrix.push(row);
    }
    for (row_index, row) in matrix.iter().enumerate() {
        for(col_index, col) in row.iter().enumerate() {
            if col == "X" {
                // horizontal check
                if position_exists(&matrix, row_index, col_index + 3) {
                    if matrix[row_index][col_index + 1] == "M" && matrix[row_index][col_index + 2] == "A" && matrix[row_index][col_index + 3] == "S" {
                        word_count += 1;
                    }
                }
                // horizontal right to left
                if col_index >= 3 && position_exists(&matrix, row_index, col_index - 3) {
                    if matrix[row_index][col_index - 1] == "M" && matrix[row_index][col_index - 2] == "A" && matrix[row_index][col_index - 3] == "S" {
                        word_count += 1;
                    }

                }
                // vertical normal
                if position_exists(&matrix, row_index + 3, col_index) {
                    if matrix[row_index + 1][col_index] == "M" && matrix[row_index + 2][col_index] == "A" && matrix[row_index + 3][col_index] == "S" {
                        word_count += 1;
                    }
                }
                // vertical reverse
                if row_index >= 3 && position_exists(&matrix, row_index - 3, col_index) {
                    if matrix[row_index - 1][col_index] == "M" && matrix[row_index - 2][col_index] == "A" && matrix[row_index - 3][col_index] == "S" {
                        word_count += 1;
                    }
                }
                // diagnol
                if position_exists(&matrix, row_index + 3, col_index + 3){
                    if matrix[row_index + 1][col_index + 1] == "M" && matrix[row_index + 2][col_index + 2] == "A" && matrix[row_index + 3][col_index + 3] == "S" {
                        word_count += 1;
                    }

                }
                if row_index >= 3 && col_index >= 3 && position_exists(&matrix, row_index - 3, col_index - 3){
                    if matrix[row_index - 1][col_index - 1] == "M" && matrix[row_index - 2][col_index - 2] == "A" && matrix[row_index - 3][col_index - 3] == "S" {
                        word_count += 1;
                    }

                }

                if col_index >= 3 && position_exists(&matrix, row_index + 3, col_index - 3){
                    if matrix[row_index + 1][col_index - 1] == "M" && matrix[row_index + 2][col_index - 2] == "A" && matrix[row_index + 3][col_index - 3] == "S" {
                        word_count += 1;
                    }
                }
                if row_index >= 3 && position_exists(&matrix, row_index - 3, col_index + 3){
                    if matrix[row_index - 1][col_index + 1] == "M" && matrix[row_index - 2][col_index + 2] == "A" && matrix[row_index - 3][col_index + 3] == "S" {
                        word_count += 1;
                    }
                }
            }
        }
    }
    return word_count; 
}

// ugly but works :) idk rust
fn day_four_part_two() -> i32 {
    let mut matrix: Vec<Vec<String>> = Vec::new();
    let mut word_count = 0;
    let mut visited_intersects: Vec<(isize, isize)> = Vec::new();
    let file_contents = fs::read_to_string("day_4.txt").expect("Error reading day 4 input");
    for line in file_contents.lines() {
        let row: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        matrix.push(row);
    }

    // directions [(down right), (down left), (up right), (up left)]
    let directions: [((isize, isize), (isize, isize)); 4] = [((1,1), (2,2)), ((1, -1), (2, -2)), ((-1, 1), (-2, 2)), ((-1, -1), (-2, -2))];
    for (row_index, row) in matrix.iter().enumerate() {
        for(col_index, col) in row.iter().enumerate() {
            if col == "M" {
                for ((x1, y1), (x2, y2)) in &directions {
                    let r1 = row_index as isize + y1;
                    let c1 = col_index as isize + x1;
                    let r2 = row_index as isize + y2;
                    let c2 = col_index as isize + x2;

                    if !in_bounds(r1, c1, &matrix) || !in_bounds(r2, c2, &matrix) {
                        continue;
                    }

                   if matrix[r1 as usize][c1 as usize] == "A" && matrix[r2 as usize][c2 as usize] == "S" {
                       let a_pos = (r1, c1);
                       // stop if we have been already to avoid dupes
                       if visited_intersects.contains(&a_pos) {
                           continue;
                       }
                       visited_intersects.push(a_pos);
                       let m_pos = (row_index, col_index);
                       let dir = (a_pos.0 as isize - m_pos.0 as isize, a_pos.1 as isize - m_pos.1 as isize);
                        match dir {
                            (1, 1) => {
                                if matrix[(a_pos.0 as isize + 1) as usize][(a_pos.1 as isize - 1) as usize] == "S"
                                    && matrix[(a_pos.0 as isize - 1) as usize][(a_pos.1 as isize + 1) as usize] == "M"
                                {
                                    word_count += 1;
                                }
                                if matrix[(a_pos.0 as isize + 1) as usize][(a_pos.1 as isize - 1) as usize] == "M"
                                    && matrix[(a_pos.0 as isize - 1) as usize][(a_pos.1 as isize + 1) as usize] == "S"
                                {
                                    word_count += 1;
                                }
                            }
                            (1, -1) => {
                                if matrix[(a_pos.0 as isize - 1) as usize][(a_pos.1 as isize - 1) as usize] == "S"
                                    && matrix[(a_pos.0 as isize + 1) as usize][(a_pos.1 as isize + 1) as usize] == "M"
                                {
                                    word_count += 1;
                                }
                                if matrix[(a_pos.0 as isize - 1) as usize][(a_pos.1 as isize - 1) as usize] == "M"
                                    && matrix[(a_pos.0 as isize + 1) as usize][(a_pos.1 as isize + 1) as usize] == "S"
                                {
                                    word_count += 1;
                                }
                            }
                            (-1, 1) => {
                                if matrix[(a_pos.0 as isize - 1) as usize][(a_pos.1 as isize - 1) as usize] == "S"
                                    && matrix[(a_pos.0 as isize + 1) as usize][(a_pos.1 as isize + 1) as usize] == "M"
                                {
                                    word_count += 1;
                                }
                                if matrix[(a_pos.0 as isize - 1) as usize][(a_pos.1 as isize - 1) as usize] == "M"
                                    && matrix[(a_pos.0 as isize + 1) as usize][(a_pos.1 as isize + 1) as usize] == "S"
                                {
                                    word_count += 1;
                                }
                            }
                            (-1, -1) => {
                                if matrix[(a_pos.0 as isize + 1) as usize][(a_pos.1 as isize - 1) as usize] == "S"
                                    && matrix[(a_pos.0 as isize - 1) as usize][(a_pos.1 as isize + 1) as usize] == "M"
                                {
                                    word_count += 1;
                                }
                                if matrix[(a_pos.0 as isize + 1) as usize][(a_pos.1 as isize - 1) as usize] == "M"
                                    && matrix[(a_pos.0 as isize - 1) as usize][(a_pos.1 as isize + 1) as usize] == "S"
                                {
                                    word_count += 1;
                                }
                            }
                            _ => {}
                        }

                   }
                }
            }
        }
    }
    return word_count; 
}

fn in_bounds(row: isize, col: isize, matrix: &[Vec<String>]) -> bool {
    row >= 0 && col >= 0 && row < matrix.len() as isize && col < matrix[0].len() as isize
}

fn position_exists(matrix: &Vec<Vec<String>>, row: usize, col: usize) -> bool {
    row < matrix.len() && col < matrix[0].len()
}

