use std::collections::HashSet;

fn main() {
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
