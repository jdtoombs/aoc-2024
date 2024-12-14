use std::{collections::HashSet, fs, isize, usize};

static OBSTACLE: &str = "#";

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn all() -> Vec<Direction> {
        return vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
    }
}

fn main() {
    println!("Day six part one results: {}", day_six_part_one());
}

fn day_six_part_one() -> i32 {
    let npc = "^";
    let file_contents = fs::read_to_string("data/day_6.txt").expect("Error reading file");
    let mut map: Vec<Vec<String>> = Vec::new();
    let mut i: usize = 0;
    let directions = Direction::all();
    let mut visited_positions: HashSet<(usize, usize)> = std::collections::HashSet::new();

    // assign map
    for line in file_contents.lines() {
        let row: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        map.push(row);
    }

    // find start position
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == npc {
                let mut curr_y = y;
                let mut curr_x = x;
                visited_positions.insert((curr_x, curr_y));
                while map[curr_y][curr_x] != OBSTACLE {
                    // check if can go forward
                    if let (Some(new_x), Some(new_y)) = (
                        add_two_indicies(curr_x, directions[i].value().0),
                        add_two_indicies(curr_y, directions[i].value().1),
                    ) {
                        if new_y >= map.len() || new_x >= map[new_y].len() {
                            return visited_positions.len() as i32;
                        }
                        if map[new_y][new_x] != OBSTACLE {
                            curr_x = new_x;
                            curr_y = new_y;
                            visited_positions.insert((curr_x, curr_y));

                        } else {
                            i = (i + 1) % directions.len();
                        }
                    } else {
                        // include initial position
                        return visited_positions.len() as i32;
                    }
                }
            }
        }
    }
    return 0;
}

fn day_six_part_two() -> i32 {
    return 0;
}

// add two indicies of mixed types (usize and isize)
fn add_two_indicies(a: usize, b: isize) -> Option<usize> {
    let sum = a as isize + b;
    if sum < 0 {
        return None;
    } else {
        return Some(sum as usize);
    }
}

// Went on a dumb tangent with this one
// fn traverse_map(
//     map: &Vec<Vec<String>>,
//     curr_x: usize,
//     curr_y: usize,
//     visited_positions: &mut HashSet<(usize, usize)>,
//     i: &mut usize,
//     halted: &mut bool
// ) {
//     let directions = Direction::all();
//
//     if *halted {
//         return;
//     }
//
//     visited_positions.insert((curr_x, curr_y));
//
//     // we would only need to check each direction once
//     for _ in 0..directions.len() {
//         if *halted {
//             return;
//         }
//
//         if let (Some(new_x), Some(new_y)) = (
//             add_two_indicies(curr_x, directions[*i].value().0),
//             add_two_indicies(curr_y, directions[*i].value().1),
//         ) {
//             if new_x >= map[0].len() || new_y >= map.len() {
//                 println!("Total distinct visits: {} ", visited_positions.len());
//                 *halted = true;
//                 return;
//             }
//             if map[new_y][new_x] != OBSTACLE {
//                 traverse_map(map, new_x, new_y, visited_positions, i, halted);
//             }
//             if map[new_y][new_x] == OBSTACLE {
//                 *i = (*i + 1) % directions.len();
//             }
//         } else {
//             return;
//         }
//     }
// }
