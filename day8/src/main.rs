fn main() {
    let input = include_str!("../input.txt").trim();

    let tree = parse_input(input);
    let tree_length = tree.len();

    let mut tree_product = 0;

    for (row, tree_line) in tree.iter().enumerate() {
        if row == 0 || row == (tree_length - 1) {
            continue;
        }
        let tree_line_length = tree_line.len();
        for (column, _curr_tree) in tree_line.iter().enumerate() {
            if column == 0 || column == (tree_line_length - 1) {
                continue;
            }
            let top = get_all_top(&tree, row, column);
            let bottom = get_all_down(&tree, row, column);
            let right = get_all_right(&tree, row, column);
            let left = get_all_left(&tree, row, column);

            println!(
                r#"
                     row {row}: column {column} of value {_curr_tree}

                     has:
                        top: {top}
                        bottom: {bottom}
                        right: {right}
                        left: {left}

                     "#
            );

            let new_prod = top * bottom * right * left;
            if new_prod > tree_product {
                tree_product = new_prod;
            }
        }
    }

    println!("Best product is {}", tree_product);
}
fn get_all_down(tree_lines: &Vec<Vec<u64>>, tree_row: usize, tree_column: usize) -> usize {
    let row_length = tree_lines.len();

    let mut trees = 0;
    for x in (tree_row + 1)..row_length {
        if tree_lines[x][tree_column] >= tree_lines[tree_row][tree_column] {
            trees += 1;
            break;
        } else {
            trees += 1;
        }
    }

    trees
}
fn get_all_right(tree_lines: &Vec<Vec<u64>>, tree_row: usize, tree_column: usize) -> usize {
    let column_length = tree_lines[0].len();

    let mut trees = 0;
    for x in (tree_column + 1)..column_length {
        if tree_lines[tree_row][x] >= tree_lines[tree_row][tree_column] {
            trees += 1;
            break;
        } else {
            trees += 1;
        }
    }

    trees
}
fn get_all_left(tree_lines: &Vec<Vec<u64>>, tree_row: usize, tree_column: usize) -> usize {
    let mut trees = 0;
    for x in (0..tree_column).rev() {
        if tree_lines[tree_row][x] >= tree_lines[tree_row][tree_column] {
            trees += 1;
            break;
        } else {
            trees += 1;
        }
    }

    trees
}

fn get_all_top(tree_lines: &Vec<Vec<u64>>, tree_row: usize, tree_column: usize) -> usize {
    let mut trees = 0;
    for x in (0..tree_row).rev() {
        if tree_lines[x][tree_column] >= tree_lines[tree_row][tree_column] {
            trees += 1;
            break;
        } else {
            trees += 1;
        }
    }

    trees
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut vec: Vec<Vec<u64>> = Vec::new();

    for line in input.lines() {
        let mut inner_vec: Vec<u64> = Vec::new();
        line.chars()
            .for_each(|x| inner_vec.push(x.to_digit(10).unwrap().into()));

        vec.push(inner_vec);
    }

    vec
}
