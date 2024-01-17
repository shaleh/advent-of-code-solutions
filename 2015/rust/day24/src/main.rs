use itertools::Itertools;

fn balance(package_weights: &[usize], number_of_groups: usize) -> (Vec<usize>, usize) {
    let total: usize = package_weights.iter().sum();
    let weight_average = total / number_of_groups;
    let group_size = package_weights.len() / number_of_groups;
    println!(
        "{total} {weight_average} {} group sizes {group_size}",
        package_weights.len()
    );

    let result = (4..=group_size)
        .flat_map(|size| {
            package_weights
                .iter()
                .copied()
                .combinations(size)
                .filter(|item| item.iter().sum::<usize>() == weight_average)
                .map(|item| (item.to_vec(), item.iter().product::<usize>()))
                .sorted_by_key(|item| item.1)
                .next()
        })
        .sorted_by_key(|item| item.1)
        .next()
        .unwrap();
    result
}

fn main() {
    let data = include_str!("../input");
    let packages: Vec<usize> = data.lines().map(|x| x.parse().unwrap()).collect();
    println!("{:?}", balance(&packages, 3));
    println!("{:?}", balance(&packages, 4));
}
