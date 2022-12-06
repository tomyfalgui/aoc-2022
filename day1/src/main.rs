use std::fs;

/*
 *
 *
 *
 *

 // SMART SOLUTION
 // https://www.reddit.com/r/rust/comments/z9w169/comment/iyjufd0/
fn main() {

let mut elves = include_str!("../../inputs/input01.txt")
    .split("\n\n")
    .map(|x| {
        return x.lines().flat_map(str::parse::<usize>).sum();
    })
    .collect::<Vec<usize>>();

elves.sort_by(|a,b| b.cmp(a));

println!("{:?}", elves[0]);
println!("{:?}", elves.iter().take(3).sum::<usize>());

}


 */
fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Cant read file"); // include_str!()
    let split = contents.split('\n');
    let tite = contents.split("\n\n");
    // split ("\n\n") to have [
    // "
    //  1
    //  2
    //  3
    //  4
    // ",
    // "
    // 5
    // 6
    // 7
    // "
    // ]
    // lines returns an iterator with element as each line
    // flat_map( ) acts as a map(fn).flat() then sums

    println!("{:?}", tite);

    let mut values: Vec<i32> = Vec::new();
    let mut curr_value = 0;
    // shit code
    // double loop
    for element in split {
        let non_int_default = -1;

        let converted: i32 = element.parse().unwrap_or(non_int_default);
        if converted == -1 {
            values.push(curr_value);
            curr_value = 0;
        } else {
            curr_value += converted;
        }
    }

    values.sort();
    let len = values.len();

    let bottom_three = &values[len - 3..];

    println!("{:?}", bottom_three);
    println!("{:?}", bottom_three.iter().sum::<i32>());
}

#[allow(dead_code)]
fn part_one() -> i32 {
    let contents = fs::read_to_string("./input.txt").expect("Cant read file");
    let split = contents.split('\n');

    let mut max_value = 0;
    let mut curr_value = 0;
    for element in split {
        let non_int_default = -1;

        let converted: i32 = element.parse().unwrap_or(non_int_default);
        // compare
        if converted == -1 {
            if curr_value > max_value {
                max_value = curr_value;
            }

            curr_value = 0;
        } else {
            curr_value += converted;
        }
    }

    max_value
}
