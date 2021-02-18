use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
struct Passport<'a> {
    fields: Vec<&'a str>,
}
impl Passport<'_> {
    fn validate(&self, entries: &HashMap<&str, &str>) -> bool {
        if self.fields.iter().map(|x| entries.contains_key(x) ).all(|x| x) {
            entries.iter().map( |(&key, val)| {
                match key {
                    "byr" => {
                        let cond = val.parse::<u32>().unwrap_or_default();
                        cond >= 1920 && cond <= 2002
                    },
                    "iyr" => {
                        let cond = val.parse::<u32>().unwrap_or_default();
                        cond >= 2010 && cond <= 2020
                    },
                    "eyr" => {
                        let cond = val.parse::<u32>().unwrap_or_default();
                        cond >= 2020 && cond <= 2030
                    },
                    "hgt" => self.hgt_check(val),
                    "hcl" => self.hcl_check(val),
                    "ecl" => self.ecl_check(val),
                    "pid" => self.pid_check(val),
                    "cid" => true,
                    _ => false,
                }
            }).all(|x| x)
        } else {
            false
        }
    }
    fn new() -> Self {
        Passport {
            fields: vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"],
        }
    }
    fn hgt_check(&self, val: &str) -> bool {
        match val.get((val.len()-2)..) {
            Some("cm") => {
                let hgt: u32 = val.strip_suffix("cm").unwrap_or_default().parse().unwrap_or_default();
                hgt >= 150 && hgt <= 193
            },
            Some("in") => {
                let hgt: u32 = val.strip_suffix("in").unwrap_or_default().parse().unwrap_or_default();
                hgt >= 59 && hgt <= 76
            },
            _ => false
        }
    }

    fn hcl_check(&self, val: &str) -> bool {
        lazy_static! {
            static ref HCL_FORM: Regex = Regex::new(r"#[[:xdigit:]]{6}").unwrap();
        }
        HCL_FORM.is_match(val)
    }
    fn ecl_check(&self, val: &str) -> bool {
        lazy_static! {
            static ref ECL_FORM: Regex = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
        }
        ECL_FORM.is_match(val)
    }
    fn pid_check(&self, val: &str) -> bool {
        lazy_static! {
            static ref PID_FORM: Regex = Regex::new(r"\d{9}").unwrap();
        }
        PID_FORM.is_match(val)
    }
}
        

fn main() {
    let stdin = io::stdin();
    let ruleset = Passport::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hgt() {
        let passport = Passport::new();
        assert!(passport.hgt_check("60in"));
        assert!(passport.hgt_check("190cm"));
        assert!(!passport.hgt_check("190in"));
        assert!(!passport.hgt_check("190"));
    }

    #[test]
    fn test_hcl() {
        let passport = Passport::new();
        assert!(passport.hcl_check("#123abc"));
        assert!(!passport.hcl_check("#123az"));
        assert!(!passport.hcl_check("123abc"));
    }

    #[test]
    fn test_ecl() {
        let passport = Passport::new();
        assert!(passport.ecl_check("brn"));
        assert!(!passport.ecl_check("wat"));
    }

    #[test]
    fn test_pid() {
        let passport = Passport::new();
        assert!(passport.pid_check("000000001"));
        assert!(!passport.pid_check("0123456789"));
    }
}
