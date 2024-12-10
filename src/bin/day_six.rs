use std::{fs, isize};

static FREE_SPACE: &str = ".";
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
            Direction::Right => (1, 0)
        }
    }
}

fn main() {
    println!("Day six part one results: {}", day_six_part_one());
}


fn day_six_part_one() -> i32 {
    let mut npc = "^";
    let file_contents = fs::read_to_string("data/day_6.txt").expect("Error reading file");
    let mut map: Vec<Vec<String>> = Vec::new();

    // assign map
    for line in file_contents.lines() {
        let row: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        map.push(row);
    }

    // find start position
    for (y, row) in map.iter().enumerate() {
        for(x, cell) in row.iter().enumerate() {
            if cell == npc {
                let mut curr_y = y;
                let mut curr_x = x;
                while map[curr_y][curr_x] != OBSTACLE {
                    // check if can go forward
                    let new_y = add_two_indicies(curr_y, Direction::Up.value().1);
                    let new_x = add_two_indicies(curr_x, Direction::Up.value().0);
                    if map[new_y][new_x] == FREE_SPACE {
                        curr_x = new_x;
                        curr_y = new_y;
                        println!("Moving");
                    } else  {
                        println!("Ouch");
                        break;
                    }
                }
            }
        }
    }
    return 0;
}

// add two indicies of mixed types (usize and isize)
fn add_two_indicies(a: usize, b: isize) -> usize {
    return (a as isize + b) as usize;
}

