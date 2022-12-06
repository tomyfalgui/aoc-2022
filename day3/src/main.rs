use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    part_one(input);
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
            let mut code_point = *val as u32;
            // a - z
            if code_point >= 97 {
                code_point -= 96;
            } else if code_point >= 65 {
                code_point = code_point - 65 + 27;
            }
            total_priorities += code_point;
        }
    }

    println!("Total: {total_priorities}");
}
