use std::error::Error;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::str::FromStr;

use anyhow::{Context, Result};

pub fn read_input_parsed<T: FromStr>() -> Result<Vec<T>>
where
    <T as FromStr>::Err: Error + Send + Sync + 'static,
    Result<Vec<T>, anyhow::Error>: FromIterator<Result<T, <T as FromStr>::Err>>
{
    let stdin = io::stdin();
    let handle = stdin.lock();

    handle
        .lines()
        .map(|line| {
            let line = line.expect("failed to read");
            line.parse::<T>()
        })
        .collect()
}

pub fn read_input() -> Result<Vec<String>>
{
    let stdin = io::stdin();
    let handle = stdin.lock();

    handle
        .lines()
        .map(|line| line.context("failed to read"))
        .collect()
}
