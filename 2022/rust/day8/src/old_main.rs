use std::fmt::Debug;
use std::fs;

fn part1(data: &Vec<Vec<i8>>) -> u32 {
    let size = data.len() as u32;
    println!("size: {}", size);
    let mut count = 4 * (size - 1);
    for i in 1..data.len() - 1 {
        for j in 1..data.len() - 1 {
            if data[i][j - 1] < data[i][j] {
                dbg!(i, j, 'a');
                count += 1;
            } else if data[i - 1][j] < data[i][j] {
                dbg!(i, j, 'b');
                count += 1;
            } else if data[i][j + 1] < data[i][j] {
                dbg!(i, j, 'c');
                count += 1;
            } else if data[i + 1][j] < data[i][j] {
                dbg!(i, j, 'd');
                count += 1;
            }
        }
    }
    count
}

fn part2() {}

fn count_visible<'a>(items: impl Iterator<Item = &'a i8>) -> u32 {
    let (count, _) = items.fold((0, -1), |(count, height), current| {
        if height < *current {
            (count + 1, *current)
        } else {
            (count, height)
        }
    });
    count as u32
}

fn main() {
    let input_raw = fs::read_to_string("../../inputs/8").expect("data");
    let data: Vec<Vec<_>> = input_raw
        .lines()
        .map(|line| line.chars().map(|c| (c as u8 - b'0') as i8).collect())
        .collect();
    // let width = data[0].len();
    // let height = data.len();
    // let base = (2 * (width - 1)) + (2 * (height - 1));
    // println!("{}: {} x {}", base, width, height);
    println!("part 1: {}", part1(&data));
    part2();
}

fn transpose<T: Copy>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .copied()
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible() {
        let data = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(part1(&data), 21);
    }
}
