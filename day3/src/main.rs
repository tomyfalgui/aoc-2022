use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    part_one(input);
    part_two(input);
}

fn part_two(input: &str) {
    let split = input.split('\n').collect::<Vec<&str>>();
    let chunked = split.chunks(3);
    let mut total_priorities = 0;

    for chunk in chunked {
        if chunk.len() < 3 {
            continue;
        }
        let first = chunk[0].to_string().chars().collect::<HashSet<_>>();
        let second = chunk[1].to_string().chars().collect::<HashSet<_>>();
        let third = chunk[2].chars().collect::<HashSet<_>>();

        let first_second = first
            .intersection(&second)
            .copied()
            .collect::<HashSet<char>>();
        let mut final_intersection = first_second.intersection(&third);

        if let Some(val) = final_intersection.next() {
            let code_point = calculate_priority(val);
            total_priorities += code_point;
        }
    }

    println!("{}", total_priorities);
}

fn part_one(input: &str) {
    let mut total_priorities = 0;
    for rucksack in input.lines() {
        let rucksack_length = rucksack.len();
        let compartment_one = rucksack[..rucksack_length / 2]
            .to_string()
            .chars()
            .collect::<HashSet<_>>();
        let compartment_two = rucksack[rucksack_length / 2..]
            .to_string()
            .chars()
            .collect::<HashSet<_>>();

        let mut intersection = compartment_one.intersection(&compartment_two);
        // only one intersection

        if let Some(val) = intersection.next() {
            let code_point = calculate_priority(val);
            total_priorities += code_point;
        }
    }

    println!("Total: {total_priorities}");
}

fn calculate_priority(char: &char) -> u32 {
    let mut code_point = *char as u32;
    // a - z
    if code_point >= 97 {
        code_point -= 96;
    } else if code_point >= 65 {
        code_point = code_point - 65 + 27;
    }

    code_point
}
