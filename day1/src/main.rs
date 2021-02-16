use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut expenses = HashSet::<u32>::new();
    expenses.reserve(200);
    for line in stdin.lock().lines() {
        expenses.insert(line.unwrap().parse().unwrap());
    }

    for x in expenses.iter() {
        match find_pair(2020 - x, &expenses) {
            Some(com) => {
                println!("Product: {}", com.0 * com.1 * x);
                break;
            }
            _ => continue,
        }
    }

    println!("Searched all {} entries", expenses.len());
}

fn find_pair(sum: u32, set: &HashSet<u32>) -> Option<(u32, u32)> {
    let mut result = None;
    for &x in set.iter() {
        if x > sum {
            continue;
        }
        if set.contains(&(sum - x)) {
            result = Some((sum - x, x));
            break;
        }
    }
    result
}
