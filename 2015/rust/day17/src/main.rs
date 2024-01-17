use itertools::Itertools;

fn main() {
    let data = include_str!("../input");
    let containers: Vec<u64> = data.lines().map(|x| x.parse().unwrap()).collect();
    let number_of_exact_matches = containers.iter().powerset().map(|x| x.into_iter().sum()).filter(|x: &u64| *x == 150).count();
    
    println!("Matches: {}", number_of_exact_matches);

    let mut iter = containers.iter().powerset().filter_map(|x| if x.clone().into_iter().sum::<u64>() == 150 { Some(x) } else { None }).sorted_by_key(|x| x.len());
    let first_item = iter.next().unwrap();
    let mut count = 1;
    for item in iter {
        if item.len() == first_item.len() {
            count += 1;
        } else {
            break;
        }
    }
    println!("number of {} element options {}", first_item.len(), count);
}
