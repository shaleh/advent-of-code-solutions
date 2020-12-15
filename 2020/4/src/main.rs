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

impl Passport {
    fn is_valid(&self) -> Result<&str, String> {
        let byr = self.byr.as_ref().ok_or("missing birth year")?;
        parse_year(&byr, 1920..=2002)?;
        let iyr = self.iyr.as_ref().ok_or("missing issue year")?;
        parse_year(&iyr, 2010..=2020)?;
        let eyr = self.eyr.as_ref().ok_or("missing expiration year")?;
        parse_year(&eyr, 2020..=2030)?;
        let hcl = self.hcl.as_ref().ok_or("missing hair color")?;
        parse_hair_color(&hcl)?;
        let hgt = self.hgt.as_ref().ok_or("missing height")?;
        parse_height(&hgt)?;
        let ecl = self.ecl.as_ref().ok_or("missing eye color")?;
        parse_eye_color(&ecl)?;
        let pid = self.pid.as_ref().ok_or("missing passport id")?;
        parse_passport_id(&pid)?;

        Ok("valid")
    }
}

fn parse_year(year: &str, range: RangeInclusive<u32>) -> Result<&str, String> {
    if year.len() != 4 {
        return Err(format!("{}: invalid year format, not 4 digits", year));
    }
    let value = year
        .parse::<u32>()
        .map_err(|_| format!("{}: year is not a number", year))?;
    if range.contains(&value) {
        Ok(year)
    } else {
        Err(format!("invalid year format, not within {:?}", range))
    }
}

fn parse_hair_color(value: &str) -> Result<&str, String> {
    let hcl_re = Regex::new(r"^#[a-z0-9]{6}$").unwrap();
    if hcl_re.is_match(value) {
        Ok(value)
    } else {
        Err(format!("{}: invalid hair color", value))
    }
}

fn parse_height(value: &str) -> Result<&str, String> {
    let hgt_re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    if let Some(capture) = hgt_re.captures_iter(value).next() {
        let potential_number = &capture[1];
        let number = potential_number
            .parse::<u32>()
            .map_err(|_| format!("{}: height is not a valid number", potential_number))?;
        match &capture[2] {
            "cm" if (150..=193).contains(&number) => {
                return Ok(value);
            }
            "in" if (59..=76).contains(&number) => {
                return Ok(value);
            }
            _ => (),
        }
    }

    Err(format!("{}: invalid height", value))
}

fn parse_eye_color(value: &str) -> Result<&str, String> {
    if VALID_EYE_COLORS.contains(&value) {
        Ok(value)
    } else {
        Err(format!("{}: invalid eye color", value))
    }
}

fn parse_passport_id(value: &str) -> Result<&str, String> {
    let pid_re = Regex::new(r"^\d{9}$").unwrap();
    pid_re.find(value).map_or_else(
        || Err(format!("{}: invalid passport id", value)),
        |_| Ok(value),
    )
}

fn main() {
    let data: Vec<Passport> = match serde_json::from_reader(io::stdin()) {
        Ok(data) => data,
        Err(msg) => {
            panic!("Error: {}", msg);
        }
    };
    let (valid, invalid): (Vec<_>, Vec<_>) =
        data.iter().map(|x| x.is_valid()).partition(Result::is_ok);
    println!("Valid count: {}", valid.len());
    println!("Invalid count: {}", invalid.len());
    invalid.iter().for_each(|v| println!("{:?}", v));
}
