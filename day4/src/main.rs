use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
#[derive(Debug)]
struct Passport<'a> {
    fields: Vec<&'a str>,
}
impl Passport<'_> {
    fn validate(&self, entries: &HashMap<&str, &str>) -> bool {
        self.fields.iter().map(|x| entries.contains_key(x) ).all(|x| x)
    }
}
        

fn main() {
    let stdin = io::stdin();
    let ruleset = Passport {
            fields: vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"],
        };
    let mut count: u32 = 0;
    for line in stdin.lock().lines() {
        let line_str = line.unwrap();
        if line_str.is_empty() {
            break;
        }
        let entries: HashMap<&str, &str> = line_str.split(' ').map(|x| { let mut splt = x.split(':');
           (splt.next().unwrap(), splt.next().unwrap()) }).collect();

        if ruleset.validate(&entries) {
            count += 1;
        }
    }

    println!("Found {} valid passports", count);
}
