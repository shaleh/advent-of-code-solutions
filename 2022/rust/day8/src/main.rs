use std::cmp::max;
use std::collections::HashMap;
use std::fs;

fn go_left(values: &[u8]) -> Vec<(usize, u8)> {
    values.iter().copied().enumerate().rev().collect()
}

fn go_right(values: &[u8]) -> Vec<(usize, u8)> {
    values.iter().copied().enumerate().collect()
}

fn next_greater(action: fn(&[u8]) -> Vec<(usize, u8)>, values: &[u8]) -> Vec<Option<u8>> {
    let mut result = vec![None; values.len()];
    let mut stack = Vec::new();

    for (i, num) in action(values) {
        while let Some(&idx) = stack.last() {
            if values[idx] > num {
                break;
            }
            let index = stack.pop().unwrap();
            result[index] = Some(num);
        }
        stack.push(i);
    }

    result
}

fn next_greater_left(values: &[u8]) -> Vec<Option<u8>> {
    next_greater(go_left, values)
}

fn next_greater_right(values: &[u8]) -> Vec<Option<u8>> {
    next_greater(go_right, values)
}

fn modified_next_greater(
    action: fn(&[u8]) -> Vec<(usize, u8)>,
    values: &[u8],
) -> Vec<Option<usize>> {
    let mut result = vec![None; values.len()];
    let mut stack = Vec::new();

    for (i, num) in action(values) {
        while let Some(&idx) = stack.last() {
            if values[idx] > num {
                break;
            }
            let index = stack.pop().unwrap();
            result[index] = Some(i);
        }
        stack.push(i);
    }

    result
}

fn modified_next_greater_left(values: &[u8]) -> Vec<Option<usize>> {
    modified_next_greater(go_left, values)
}

fn modified_next_greater_right(values: &[u8]) -> Vec<Option<usize>> {
    modified_next_greater(go_right, values)
}

fn part1(forest: &Vec<Vec<u8>>) -> usize {
    let transposed_forest = transpose(forest);

    let blocking_tree_left: Vec<Vec<Option<u8>>> =
        forest.iter().map(|row| next_greater_left(row)).collect();
    let blocking_tree_right: Vec<Vec<Option<u8>>> =
        forest.iter().map(|row| next_greater_right(row)).collect();
    let blocking_tree_above: Vec<Vec<Option<u8>>> = {
        let columns = transposed_forest
            .iter()
            .map(|row| next_greater_left(row))
            .collect();
        transpose(&columns)
    };
    let blocking_tree_below: Vec<Vec<Option<u8>>> = {
        let columns = transposed_forest
            .iter()
            .map(|row| next_greater_right(row))
            .collect();
        transpose(&columns)
    };

    let mut visible_trees: HashMap<(usize, usize), u8> = HashMap::new();
    for i in 0..forest.len() {
        for j in 0..forest[0].len() {
            if blocking_tree_left[i][j].is_none()
                || blocking_tree_right[i][j].is_none()
                || blocking_tree_above[i][j].is_none()
                || blocking_tree_below[i][j].is_none()
            {
                visible_trees.entry((i, j)).or_insert(forest[i][j]);
            }
        }
    }

    visible_trees.len()
}

fn part1_brute(forest: &Vec<Vec<u8>>) -> usize {
    let transposed_forest = transpose(forest);

    let mut visible_trees: HashMap<(usize, usize), u8> = HashMap::new();
    for (i, row) in forest.iter().enumerate() {
        for (j, column) in transposed_forest.iter().enumerate() {
            let is_visible = |&value| value < forest[i][j];
            if (row[0..j]).iter().all(is_visible)
                || row[j + 1..].iter().all(is_visible)
                || column[0..i].iter().all(is_visible)
                || column[i + 1..].iter().all(is_visible)
            {
                visible_trees.entry((i, j)).or_insert(forest[i][j]);
            }
        }
    }

    visible_trees.len()
}

fn part2(forest: &Vec<Vec<u8>>) -> usize {
    let transposed_forest = transpose(forest);
    let w = forest[0].len();
    let h = forest.len();

    let blocking_trees_left: Vec<Vec<Option<usize>>> = forest
        .iter()
        .map(|row| modified_next_greater_left(row))
        .collect();
    let blocking_trees_right: Vec<Vec<Option<usize>>> = forest
        .iter()
        .map(|row| modified_next_greater_right(row))
        .collect();
    let blocking_trees_above: Vec<Vec<Option<usize>>> = {
        let tmp = transposed_forest
            .iter()
            .map(|column| modified_next_greater_left(column))
            .collect();
        transpose(&tmp)
    };
    let blocking_trees_below: Vec<Vec<Option<usize>>> = {
        let tmp = transposed_forest
            .iter()
            .map(|column| modified_next_greater_right(column))
            .collect();
        transpose(&tmp)
    };

    let mut highest_dist = 0;
    for i in 0..forest.len() {
        for j in 0..transposed_forest.len() {
            let left_dist = blocking_trees_left[i][j].map_or(j, |distance| j - distance);
            let right_dist = blocking_trees_right[i][j].map_or(w - j - 1, |distance| distance - j);
            let above_dist = blocking_trees_above[i][j].map_or(i, |distance| i - distance);
            let below_dist = blocking_trees_below[i][j].map_or(h - i - 1, |distance| distance - i);

            let viewing_dist = right_dist * left_dist * above_dist * below_dist;
            highest_dist = max(highest_dist, viewing_dist);
        }
    }

    highest_dist
}

fn scenic_score<I>(values: I, sentinel: u8) -> usize
where
    I: Iterator<Item = u8> + ExactSizeIterator,
{
    let mut count = 0;
    for value in values {
        count += 1;
        if value >= sentinel {
            break;
        }
    }
    count
}

fn part2_brute(forest: &Vec<Vec<u8>>) -> usize {
    let transposed_forest = transpose(forest);

    let mut max_scenic_score = 0;
    for (i, row) in forest.iter().enumerate() {
        for (j, column) in transposed_forest.iter().enumerate() {
            let score = scenic_score(row[0..j].iter().rev().copied(), forest[i][j])
                * scenic_score(row[j + 1..].iter().copied(), forest[i][j])
                * scenic_score(column[0..i].iter().rev().copied(), forest[i][j])
                * scenic_score(column[i + 1..].iter().copied(), forest[i][j]);
            max_scenic_score = max(max_scenic_score, score);
        }
    }

    max_scenic_score
}

fn main() {
    let input_raw = fs::read_to_string("../../inputs/8").expect("data");
    let data: Vec<Vec<_>> = input_raw
        .lines()
        .map(|line| line.chars().map(|c| (c as u8 - b'0') as u8).collect())
        .collect();
    println!("part 1: {}", part1_brute(&data));
    println!("part 1: {}", part1(&data));
    println!("part 2: {}", part2_brute(&data));
    println!("part 2: {}", part2(&data));
}

fn transpose<T: Copy>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.iter().map(|n| n.iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .copied()
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible() {
        let data: Vec<Vec<u8>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        assert_eq!(part1(&data), 21);
    }

    #[test]
    fn test_part2() {
        let data: Vec<Vec<u8>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let transposed_data = transpose(&data);

        assert_eq!(
            scenic_score(transposed_data[2][0..3].iter().rev().copied(), 5, 3),
            2
        );
        assert_eq!(scenic_score(data[3][0..2].iter().rev().copied(), 5, 2), 2);
        assert_eq!(
            scenic_score(transposed_data[2][4..].iter().copied(), 5, 1),
            1
        );
        assert_eq!(scenic_score(data[3][3..].iter().copied(), 5, 2), 2);
        assert_eq!(part2_brute(&data), 8);
    }
}
