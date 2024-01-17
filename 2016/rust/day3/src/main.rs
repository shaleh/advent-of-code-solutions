use itertools::Itertools;

fn is_valid_triangle(sides: &[i64]) -> bool {
    (sides[0] < sides[1] + sides[2])
        && (sides[1] < sides[0] + sides[2])
        && (sides[2] < sides[0] + sides[1])
}

fn count_valid_triangles(maybe_triangles: &[Vec<i64>]) -> usize {
    maybe_triangles
        .iter()
        .filter(|x| is_valid_triangle(x))
        .count()
}

fn main() {
    let data = include_str!("../input");

    let triangles: Vec<Vec<i64>> = data
        .lines()
        .map(|line| {
            line.to_string()
                .trim()
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    //println!("{triangles:?}");
    println!("{}", count_valid_triangles(&triangles));

    // Transpose rows -> columns, then collect in groups of 3.
    let triangles_by_column: Vec<Vec<i64>> = (0..3)
        .flat_map(|col| {
            (0..triangles.len())
                .map(|row| triangles[row][col])
                .chunks(3)
                .into_iter()
                .map(|x| x.into_iter().collect::<Vec<i64>>())
                .collect::<Vec<_>>()
        })
        .collect();
    println!("{}", count_valid_triangles(&triangles_by_column));
}
