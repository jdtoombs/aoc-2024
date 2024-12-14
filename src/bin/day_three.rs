use regex::Regex;
use std::fs;

fn main() {
    println!("Result: {}", day_three());
}

// just part two because they are similar
fn day_three() -> i32 {
    let file_contents = fs::read_to_string("data/day_3.txt").expect("Error reading input file");
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
