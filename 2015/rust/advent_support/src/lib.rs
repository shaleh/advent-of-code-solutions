use std::error::Error;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::Result;

pub fn read_input<T: FromStr>() -> Result<Vec<T>>
where
    <T as FromStr>::Err: Error + Send + Sync + 'static,
{
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut results = Vec::<T>::new();
    for line in handle.lines() {
        let line = line?;
        let value = line.parse::<T>()?;
        results.push(value);
    }

    Ok(results)
}
