use std::collections::HashSet;
fn main() {
    let input = include_str!("../input.txt").trim();

    let mut counter = 0;
    for window in input.chars().collect::<Vec<char>>().windows(4) {
        let mut uniq = HashSet::new();

        if window.into_iter().all(move |x| uniq.insert(x)) {
            break;
        }

        counter += 1;
    }

    println!("On the {}th index", counter + 4);
}
