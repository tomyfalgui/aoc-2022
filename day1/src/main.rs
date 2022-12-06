use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Cant read file");
    let split: Vec<&str> = contents.split('\n').map.collect();

    println!("{:?}", split);
}
