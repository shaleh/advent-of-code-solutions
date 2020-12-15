use std::io;
use std::ops::RangeInclusive;

use regex::Regex;
use serde::{Deserialize, Serialize};

static VALID_EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Serialize, Deserialize, Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn valid_year(year: &str, range: RangeInclusive<u32>) -> bool {
    if year.len() != 4 {
        return false;
    }
    let value = match year.parse::<u32>() {
        Ok(v) => v,
        Err(_) => {
            return false;
        }
    };
    range.contains(&value)
}

impl Passport {
    fn is_valid(&self) -> bool {
        if self
            .byr
            .as_ref()
            .filter(|s| valid_year(&s, 1920..=2002))
            .is_none()
        {
            return false;
        }
        if self
            .iyr
            .as_ref()
            .filter(|s| valid_year(&s, 2010..=2020))
            .is_none()
        {
            return false;
        }
        if self
            .eyr
            .as_ref()
            .filter(|s| valid_year(&s, 2020..=2030))
            .is_none()
        {
            return false;
        }
        let hcl_re = Regex::new(r"^#[a-z0-9]{6}$").unwrap();
        if self.hcl.as_ref().filter(|s| hcl_re.is_match(s)).is_none() {
            return false;
        }
        let hgt_re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        match &self.hgt {
            Some(hgt) => match hgt_re.captures_iter(&hgt).next() {
                Some(capture) => {
                    let potential_number = &capture[1];
                    let number = match potential_number.parse::<u32>() {
                        Ok(number) => number,
                        Err(_) => {
                            return false;
                        }
                    };
                    let system = &capture[2];
                    if system == "cm" {
                        if !(150..=193).contains(&number) {
                            return false;
                        }
                    } else if system == "in" {
                        if !(59..=76).contains(&number) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            },
            _ => {
                return false;
            }
        }

        if self
            .ecl
            .as_ref()
            .filter(|s| VALID_EYE_COLORS.contains(&&s[..]))
            .is_none()
        {
            return false;
        }
        let pid_re = Regex::new(r"^\d{9}$").unwrap();
        if self.pid.as_ref().filter(|s| pid_re.is_match(s)).is_none() {
            return false;
        }

        true
    }
}

fn main() {
    let data: Vec<Passport> = match serde_json::from_reader(io::stdin()) {
        Ok(data) => data,
        Err(msg) => {
            panic!("Error: {}", msg);
        }
    };
    println!(
        "Valid count: {}",
        data.iter().filter(|x| x.is_valid()).count()
    );
}
