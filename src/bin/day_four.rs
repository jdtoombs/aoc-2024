
use std::fs;

fn main() {
    println!("Part one result: {}", day_four());
    println!("Part two result: {}", day_four_part_two());
}

fn day_four() -> i32 {
    let mut matrix: Vec<Vec<String>> = Vec::new();
    let mut word_count = 0;
    let file_contents = fs::read_to_string("data/day_4.txt").expect("Error reading day 4 input");
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
    let file_contents = fs::read_to_string("data/day_4.txt").expect("Error reading day 4 input");
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
