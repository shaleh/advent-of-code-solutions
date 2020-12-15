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

fn compute_path(map: &Vec<Vec<MapElement>>) -> Vec<MapElement> {
    let mut path = Vec::new();
    let width = map[0].len();
    let height = map.len();
    println!("Map width: {}, height: {}", width, height);
    let mut pos = 0;
    for row in map {
        path.push(row[pos]);
        pos = (pos + 3) % width;
    }
    path
}

fn main() {
    let map = read_input();
    dbg!(&map);
    let path = compute_path(&map);
    dbg!(&path);
    println!("Path length: {}", path.len());
    let tree_count = path.iter().filter(|x| match x { MapElement::Tree => true, _ => false }).count();
    println!("Trees: {}", tree_count);
}
