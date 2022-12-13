use std::collections::HashSet;

fn main() {
    part_two();
}

fn is_touching(head_pos: (i64, i64), tail_pos: (i64, i64)) -> bool {
    // same column
    if head_pos.0 == tail_pos.0 && (head_pos.1 - tail_pos.1).abs() == 1 {
        true
    } else if head_pos.1 == tail_pos.1 && (head_pos.0 - tail_pos.0).abs() == 1 {
        true
    } else if (head_pos.0 - tail_pos.0).abs() == 1 && (head_pos.1 - tail_pos.1).abs() == 1 {
        true
    } else if (head_pos.0 == tail_pos.0) && (tail_pos.1 == head_pos.1) {
        true
    } else {
        false
    }
}

fn part_two() {
    let heads: [String; 10] = [
        "H".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string(),
        "8".to_string(),
        "9".to_string(),
    ];
    let input = include_str!("../input-test.txt").trim();

    let input_lines: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .collect();

    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    let mut tails: [(i64, i64); 10] = [(0, 0); 10];
    visited.insert((0, 0));

    'outer: for line in input_lines {
        let movement = line[0];
        let translation: u64 = line[1].parse().expect("can't read number");

        println!(" == {} {} ==", movement, translation);

        for _d in 0..translation {
            let mut tails_index = 0;

            match movement {
                "R" => tails[0] = (tails[0].0 + 1, tails[0].1),
                "L" => tails[0] = (tails[0].0 - 1, tails[0].1),
                "U" => tails[0] = (tails[0].0, tails[0].1 + 1),
                "D" => tails[0] = (tails[0].0, tails[0].1 - 1),
                _ => {
                    continue 'outer;
                } // /*    */                println!("{} moved: {:?} ", heads[tails_index], left[tails_index]);
            }
            while tails_index < 9 {
                if tails_index == 0 {
                    tails_index += 1;
                }
                let (left, right) = tails.split_at_mut(tails_index + 1);

                match movement {
                    "R" => left[tails_index] = (left[tails_index].0 + 1, left[tails_index].1),
                    "L" => left[tails_index] = (left[tails_index].0 - 1, left[tails_index].1),
                    "U" => left[tails_index] = (left[tails_index].0, left[tails_index].1 + 1),
                    "D" => left[tails_index] = (left[tails_index].0, left[tails_index].1 - 1),
                    _ => {
                        continue 'outer;
                    } // /*    */                println!("{} moved: {:?} ", heads[tails_index], left[tails_index]);
                }
                println!("{:?} {:?}", left, right);

                let change_x = (left[tails_index].0 - right[0].0).abs();
                let change_y = (left[tails_index].1 - right[0].1).abs();

                if change_x >= 2 || change_y >= 2 {
                    if left[tails_index].0 > right[0].0 {
                        right[0] = (right[0].0 + 1, right[0].1);
                    } else if left[tails_index].0 < right[0].0 {
                        right[0] = (right[0].0 - 1, right[0].1);
                    }

                    if left[tails_index].1 > right[0].1 {
                        right[0] = (right[0].0, right[0].1 + 1);
                    } else if left[tails_index].1 < right[0].1 {
                        right[0] = (right[0].0, right[0].1 - 1);
                    }
                }

                tails_index += 1;
            }
            visited.insert(tails[9]);
        }
    }
    println!("Tail visited {} at least once.", visited.len());
}

/*
*/

fn part_one() {
    let input = include_str!("../input.txt").trim();

    let input_lines: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .collect();

    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    visited.insert(tail_pos);

    println!("Start: {:?} ", tail_pos);
    for line in input_lines {
        let movement = line[0];
        let translation: u64 = line[1].parse().expect("can't read number");

        println!(" == {} {} ==", movement, translation);
        for _d in 0..translation {
            match movement {
                "R" => head_pos = (head_pos.0 + 1, head_pos.1),
                "L" => head_pos = (head_pos.0 - 1, head_pos.1),
                "U" => head_pos = (head_pos.0, head_pos.1 + 1),
                "D" => head_pos = (head_pos.0, head_pos.1 - 1),
                _ => continue,
            }
            println!(" == {} {} ==", movement, translation);
            println!("Head moved: {:?} ", head_pos);

            if is_touching(head_pos, tail_pos) {
                continue;
            }

            let inc = if movement == "R" || movement == "U" {
                1
            } else {
                -1
            };
            // same column
            if head_pos.0 == tail_pos.0 {
                tail_pos = (tail_pos.0, tail_pos.1 + inc);
                // same row
            } else if head_pos.1 == tail_pos.1 {
                tail_pos = (tail_pos.0 + inc, tail_pos.1);
                // diagonal
                // y-axis coordinated so head moved
                // up or down
            } else if (head_pos.0 - tail_pos.0).abs() == 1 {
                // get x-inc
                let x_inc = if head_pos.0 > tail_pos.0 { 1 } else { -1 };
                tail_pos = (tail_pos.0 + x_inc, tail_pos.1 + inc);
            } else if (head_pos.1 - tail_pos.1).abs() == 1 {
                let y_inc = if head_pos.1 > tail_pos.1 { 1 } else { -1 };
                tail_pos = (tail_pos.0 + inc, tail_pos.1 + y_inc);
            }
            println!("Tail moved: {:?} ", tail_pos);

            visited.insert(tail_pos);
        }
    }
    println!("Tail visited {} at least once.", visited.len());
}
