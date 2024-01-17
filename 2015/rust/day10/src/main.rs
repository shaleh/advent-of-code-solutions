use itertools::Itertools;

fn main() {
    let input = "1321131112";
    let result = input.chars().group_by(|x| x).collect::<Vec<char>>().into_iter().flat_map(|(key, group)| format!("{}{}", group.count(), key)).collect();
    println!("Result: {}", result);
}
