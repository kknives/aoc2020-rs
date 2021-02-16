use num_format::{Locale, ToFormattedString};
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

struct File {
    trees: usize,
    col: usize,
    len: usize,
}

struct Line {
    pos: (usize, usize),
    slope: (usize, usize),
}

impl Line {
    fn new(grad: (usize, usize)) -> Line {
        Line {
            pos: (0, 0),
            slope: grad,
        }
    }
}

impl Iterator for Line {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.pos = ((self.pos.0 + self.slope.0), (self.pos.1 + self.slope.1));
        Some(self.pos)
    }
}

fn main() {
    // Obtained by examining input file
    let input = File {
        trees: 2227,
        col: 31,
        len: 323,
    };
    let stdin = io::stdin();
    let mut forest = HashSet::new();
    forest.reserve(input.trees);
    for (y, line) in stdin.lock().lines().enumerate() {
        let line_str = line.unwrap();
        let mut inserts = line_str
            .match_indices('#')
            .map(|(x, _)| forest.insert((x, y)));
        // The iterators will not execute until we use inserts somewhere
        if inserts.any(|x| !x) {
            println!("Error processing line at {}", y);
        }
    }
    let mut acc = 1;
    for grad in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let path = Line::new(*grad);
        let trees_found = path
            .take(input.len - 1)
            .filter(|(x, y)| forest.contains(&((*x % input.col), *y)))
            .count();
        acc *= trees_found;
        println!(
            "Encountered {} trees en route for gradient {:?}",
            trees_found, grad
        );
    }

    println!(
        "Product: {} or {}",
        acc,
        acc.to_formatted_string(&Locale::en)
    );
}
