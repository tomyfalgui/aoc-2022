fn main() {
    let input = include_str!("../test.txt").trim();

    let rows = parse_input(input);
    let start_pos = find_letter(&rows, 'S');
    let end_pos = find_letter(&rows, 'E');
    let num_rows = convert_rows(rows);

    println!("{:?}", num_rows);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

fn find_letter(input: &Vec<Vec<char>>, letter: char, skip: (u64, u64)) -> (u64, u64) {
    let mut pos = (0, 0);
    let input_len = input.len();

    'outer: for i in 0..input_len {
        let column_len = input[i].len();
        for j in 0..column_len {
            if letter == input[i][j] {
                pos = (i as u64, j as u64);
                break 'outer;
            }
        }
    }

    pos
}

fn find_path_count(input: &Vec<Vec<u64>>, start: (u64, u64), end: (u64, u64)) -> Vec<u64> {
    // start at start_pos
    // CONSTRAINTS: row < input.len() and column < input.len()
    // since input has m x n dimensions
    // look down left or right or up
    // after looking at these directions
    // log each possible step
    // check if one of these steps == end
    let max_y = input.len();
    let max_x = input[0].len();

    let down_pos = (start.0, start.1 + 1);
    let up_pos = (start.0, start.1 - 1);
    let left_pos = (start.0 - 1, start.1);
    let right_pos = (start.0 + 1, start.1);

    if is_valid_step(down_pos, max_x as u64, max_y as u64) {}

    vec![1, 2, 3]
}

fn is_valid_step(next: (u64, u64), max_x: u64, max_y: u64) -> bool {
    next.0 < max_x && next.1 < max_y && next.0 > 0 && next.1 > 0
}

fn convert_rows(input: Vec<Vec<char>>) -> Vec<Vec<u64>> {
    input
        .into_iter()
        .map(|r| r.into_iter().map(|i| i as u64).collect::<Vec<u64>>())
        .collect()
}
