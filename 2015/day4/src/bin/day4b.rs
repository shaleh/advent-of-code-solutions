use md5::{Digest, Md5};

use advent_support::read_input;

fn main() {
    let lines = read_input::<String>().expect("Invalid input");
    let base = lines[0].clone();

    let mut current: i64 = 1;
    loop {
        let mut hasher = Md5::new();

        let value = format!("{}{}", base, current);
        dbg!(&value);
        hasher.update(value);

        let digest = hasher.finalize();
        let hex_digest = format!("{:x}", digest);
        if &hex_digest[0..6] == "000000" {
            break;
        }

        current += 1;
    }
    println!("Value: {}", current);
}
