enum Element {
    Char(char),
    Vec(Vec<char>),
}
fn main() {
    let input = include_str!("../test.txt").trim();

    for pair in input.split("\n\n") {
        let pair_split = pair.split("\n").collect::<Vec<&str>>();

        let first: Vec<Element> = Vec::new();
        let first_item = pair_split[0].chars();
        let mut count_brackets = 0;

        for f_char in first_item {
            println!("{:?}", f_char);
        }
    }
}
