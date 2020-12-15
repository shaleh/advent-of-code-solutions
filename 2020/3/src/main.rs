use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
enum MapElement {
    Open,
    Tree,
}

fn read_input() -> Vec<Vec<MapElement>> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input: Vec<Vec<MapElement>> = Vec::new();
    for line in handle.lines() {
        input.push(
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => MapElement::Open,
                    '#' => MapElement::Tree,
                    _ => panic!("What??"),
                })
                .collect(),
        );
    }
    input
}

fn compute_path(right: usize, down: usize, map: &[Vec<MapElement>]) -> Vec<MapElement> {
    let mut path = Vec::new();
    let width = map[0].len();
    let height = map.len();
    println!("Map width: {}, height: {}", width, height);
    let mut pos = 0;
    for row in map.iter().step_by(down) {
        path.push(row[pos]);
        pos = (pos + right) % width;
    }
    path
}

fn main() {
    let map = read_input();
    let mut trees = Vec::<usize>::new();

    for (right, down) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let path = compute_path(right, down, &map);
        println!("Path length: {}", path.len());
        let tree_count = path
            .iter()
            .filter(|x| matches!(x, MapElement::Tree))
            .count();
        println!("Trees: {}", tree_count);
        trees.push(tree_count);
    }

    let trees_product: usize = trees.iter().product();
    println!("Trees product: {}", trees_product);
}
