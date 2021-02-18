use std::convert::TryInto;
use std::io;
use std::io::prelude::*;
use std::num::ParseIntError;

#[derive(Debug)]
struct Password<'a> {
    test_chr: char,
    passwd: &'a str,
    upper_rng: u32,
    lower_rng: u32,
}

impl<'a> Password<'a> {
    fn test_policy(&self) -> bool {
        let times: u32 = self
            .passwd
            .matches(self.test_chr)
            .count()
            .try_into()
            .unwrap();
        ((self.lower_rng)..=(self.upper_rng)).contains(&times)
    }
    fn new(s: &'a str) -> Result<Self, ParseIntError> {
        let tokens: Vec<&str> = s.splitn(3, ' ').collect();

        let bounds: Vec<&str> = tokens[0].splitn(2, '-').collect();
        let upper_fromstr: u32 = bounds[1].parse()?;
        let lower_fromstr: u32 = bounds[0].parse()?;

        let chr_fromstr: char = tokens[1].chars().next().unwrap();

        Ok(Password {
            test_chr: chr_fromstr,
            passwd: tokens[2],
            upper_rng: upper_fromstr,
            lower_rng: lower_fromstr,
        })
    }
    fn test_occur(&self) -> bool {
        let charat_up = self
            .passwd
            .chars()
            .nth(self.upper_rng as usize - 1)
            .unwrap();
        let charat_lo = self
            .passwd
            .chars()
            .nth(self.lower_rng as usize - 1)
            .unwrap();

        (charat_up == self.test_chr) ^ (charat_lo == self.test_chr)
    }
}
fn main() {
    let stdin = io::stdin();
    let mut count: u32 = 0;
    let mut tobo_count: u32 = 0;
    for line in stdin.lock().lines() {
        let line_str = line.unwrap();
        let passwd = Password::new(&line_str).unwrap();
        if passwd.test_policy() {
            count += 1;
        }
        if passwd.test_occur() {
            tobo_count += 1;
        }
    }

    println!("Found {} valid passwords", count);
    println!("Found {} Toboggan valid passwords", tobo_count);
}
