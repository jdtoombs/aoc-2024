use std::{fs, i32, usize};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("Result: {}", day_five_part_one());
    println!("Result: {}", day_five_part_two());
}

fn day_five_part_one() -> i32 {
    let file_contents = fs::read_to_string("data/day_5.txt").expect("Error reading file");
    let mut print_rules = HashMap::new();
    let mut sum = 0;
    // note to self, unwrap can panic and is discouraged
    // start using \d+ instead of [0-9]*
    let rules = Regex::new(r"([0-9]*)\|([0-9]*)|(\d+(,\d+)*)").unwrap();
    for cap in rules.captures_iter(&file_contents) {
        if let Some(key) = cap.get(1) {
            if let Some(value) = cap.get(2){
                // nts: as_str() is a &str which is immutable and we eneed 
                let key: &str = key.as_str(); 
                // nts: default hashmap expects String, as_str() is &str into put to inferred type (aka
                // String)
                let value = value.as_str().into();
                if print_rules.contains_key(&key) {
                    let new_val = format!("{},{}", print_rules.get(&key).unwrap(), value);
                    print_rules.insert(key, new_val);
                }else {
                    print_rules.insert(key, value);
                }
            }
        }
        if let Some(update) = cap.get(3){
            let mut valid = true;
            let update_values: Vec<&str> = update.as_str().split(",").collect();
            for (i, k) in update_values.iter().enumerate() {
                if print_rules.contains_key(k) {
                    if i == 0 {
                        continue;
                    }
                    // nts: 0..i-1 will not run when i = 1 in rust
                    for n in 0..i {
                        let pattern = format!(r"{}", regex::escape(update_values[n]));
                        let re = Regex::new(&pattern).unwrap();
                        if re.is_match(&print_rules.get(k).unwrap()) {
                            valid = false;
                            break;
                        } 
                    }
                }
            }
            if valid {
                sum += update_values[(update_values.len() - 1) / 2 ].parse::<i32>().unwrap();
            }
        }
    }
    return sum;
}

fn day_five_part_two() -> i32 {
    let file_contents = fs::read_to_string("data/day_5.txt").expect("Error reading file");
    let mut print_rules = HashMap::new();
    let mut sum = 0;

    let rules = Regex::new(r"([0-9]*)\|([0-9]*)|(\d+(,\d+)*)").unwrap();
    for cap in rules.captures_iter(&file_contents) {
        if let Some(key) = cap.get(1) {
            if let Some(value) = cap.get(2){
                let key: &str = key.as_str(); 
                let value = value.as_str().into();
                if print_rules.contains_key(&key) {
                    let new_val = format!("{},{}", print_rules.get(&key).unwrap(), value);
                    print_rules.insert(key, new_val);
                }else {
                    print_rules.insert(key, value);
                }
            }
        }
        if let Some(update) = cap.get(3){
            let mut valid = true;
            let mut update_values: Vec<&str> = update.as_str().split(",").collect();
            let updates: Vec<(usize, &str)> = update_values.iter().enumerate().map(|(i, &val)| (i, val)).collect();
            // nts: if we pass updates to loop instead of &updates we can no longer use it after, that
            // is why we are passing a ref
            for (i, k) in  &updates {
                if print_rules.contains_key(k) {
                    if *i == 0 {
                        continue;
                    }
                    for n in 0..*i {
                        let pattern = format!(r"{}", regex::escape(update_values[n]));
                        let re = Regex::new(&pattern).unwrap();
                        if re.is_match(&print_rules.get(k).unwrap()) {
                            valid = false;
                            // issue would arise here if for loop iterated updates instead of
                            // &updates
                            update_values.swap(n, *i);
                        } 
                    }
                }
                if *i == updates.len() - 1 && !valid {
                    // last iteration we can safely add
                    sum += update_values[(update_values.len() - 1) / 2 ].parse::<i32>().unwrap();
                }
            }
        }
    }
    return sum;
}

