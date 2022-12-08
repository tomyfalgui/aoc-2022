fn main() {
    let input = include_str!("../input.txt");

    let answ = part_one(input);
    println!("{answ}");
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let pair = line.split(',').collect::<Vec<&str>>();
            let range_one = pair[0];
            let range_two = pair[1];
            let split = range_one
                .split('-')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();
            let split_two = range_two
                .split('-')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();

            let (a, b) = (split[0]..=split[1], split_two[0]..=split_two[1]);
            let a_copy = a.clone();
            let a_vec = a.collect::<Vec<usize>>().iter().all(|z| b.contains(&z));
            let b_vec = b
                .collect::<Vec<usize>>()
                .iter()
                .all(|z| a_copy.contains(&z));
            if a_vec || b_vec {
                Some(1)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .len()
}
