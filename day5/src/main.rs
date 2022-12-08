#[derive(Debug)]
struct InstructionSet {
    from: usize,
    to: usize,
    count: usize,
}

impl InstructionSet {
    fn from_vec(vec: Vec<&str>) -> Self {
        Self {
            from: vec[3].to_string().parse().unwrap(),
            count: vec[1].to_string().parse().unwrap(),
            to: vec[5].to_string().parse().unwrap(),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let (mut stacks, instructions) = parse_input(input);

    for instruction in instructions {
        let InstructionSet { from, to, count } = instruction;
        let from_index = from - 1;
        let to_index = to - 1;

        let from_vec = stacks.get_mut(from_index).unwrap();
        let mut taken_vec: Vec<_> = from_vec.drain(0..count).collect();
        taken_vec.reverse();

        let to_vec = stacks.get_mut(to_index).unwrap();
        for item in taken_vec {
            to_vec.insert(0, item);
        }
        // for _i in 0..count {
        //     let from_vec = stacks.get_mut(from_index).unwrap();
        //     //let from_value = from_vec.remove(0); part 1
        //     // part 2
        //     let from_value = from_vec.pop().unwrap();
        //
        //     let to_vec = stacks.get_mut(to_index).unwrap();
        //     //to_vec.insert(0, from_value); part 1
        //     to_vec.push(from_value);
        // }
    }

    for stack in stacks {
        println!("{}", stack[0]);
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<InstructionSet>) {
    let mut stack_of_stacks: Vec<Vec<char>> = Vec::new();
    let mut instruction_sets: Vec<InstructionSet> = Vec::new();

    let input_split = input.split("\n\n").collect::<Vec<&str>>();
    let stack = input_split[0].split('\n').collect::<Vec<&str>>();
    let stack_length = stack.len();
    let stack_lines = &stack[..stack_length - 1];

    for stack_line in stack_lines.to_owned() {
        let mut counter = 0;
        for item in stack_line.split(' ') {
            if item == "" {
                counter += 1;
                if counter % 4 == 0 {
                    if let None = stack_of_stacks.get((counter / 4) - 1) {
                        let new_char_vec: Vec<char> = Vec::new();
                        stack_of_stacks.push(new_char_vec);
                    }
                }
            } else if item != "" {
                counter += 4;
                let split_word = item.chars().collect::<Vec<char>>()[1];
                if counter % 4 == 0 {
                    match stack_of_stacks.get_mut((counter / 4) - 1) {
                        Some(val) => val.push(split_word),
                        None => {
                            let mut new_char_vec: Vec<char> = Vec::new();
                            new_char_vec.push(split_word);

                            stack_of_stacks.push(new_char_vec);
                        }
                    }
                }
            }
        }
    }

    let instruction_lines = input_split[1].split('\n').collect::<Vec<&str>>();
    for instruction_set in instruction_lines {
        if instruction_set.len() == 0 {
            continue;
        }
        let space_split = instruction_set.split(' ').collect::<Vec<&str>>();
        let instruction = InstructionSet::from_vec(space_split);
        instruction_sets.push(instruction);
    }

    (stack_of_stacks, instruction_sets)
}
