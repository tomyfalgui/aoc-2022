use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Cant read file");
    let split = contents.split('\n');

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
