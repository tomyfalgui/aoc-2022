fn main() {
    let input = include_str!("../test.txt").trim();

    let init_vec = input
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let init_vec_length = init_vec.len();

    let mut final_vec = init_vec.clone();

    for i in 0..init_vec_length {
        let to_move = init_vec.remove(i);
        println!("{}", to_move);
        let final_index = calculate_index(init_vec_length as isize, to_move, i as isize);

        println!("moving {} to {} ", to_move, final_index);

        init_vec.insert(final_index, to_move);
        println!("{:?}", init_vec);
    }

    println!("{:?}", init_vec);
}

fn calculate_index(length: isize, to_move: isize, i: isize) -> usize {
    if i + to_move > length - 1 {
        ((i + to_move) % (length - 1)) as usize
    } else if i + to_move < 0 {
        (length - (i + to_move)) as usize
    } else {
        (i + to_move) as usize
    }
}
