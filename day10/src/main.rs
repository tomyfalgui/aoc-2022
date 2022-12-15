fn main() {
    let input = include_str!("../input.txt").trim();

    let mut cycles = 0;
    let mut register = 1;
    // let mut strengths: Vec<i64> = Vec::new();
    let mut pixels: Vec<Vec<char>> = Vec::new();
    for _i in 0..6 {
        let mut new_vec_char = Vec::new();
        for _j in 0..40 {
            new_vec_char.push('.');
        }

        pixels.push(new_vec_char);
    }
    for line in input.lines() {
        let command = line.split(' ').collect::<Vec<&str>>();
        let name = command[0];
        let mut value: Option<i64> = None;

        if name == "addx" {
            value = Some(command[1].parse().unwrap());
        }

        let mut current_cycles = 0;
        loop {
            let mut must_break = false;
            let parent_array_index: i64 = cycles / 40;
            let real_index: i64 = cycles - (parent_array_index * 39) - parent_array_index;
            if parent_array_index == 6 {
                break;
            }
            if name == "noop" {
                if current_cycles == 1 {
                    must_break = true;
                }
            } else if name == "addx" {
                let inner = value.unwrap();

                if current_cycles == 2 {
                    register += inner;
                    must_break = true;
                }
            }

            // check for out of bounds
            if register + 2 > real_index && register - 2 < real_index {
                pixels[parent_array_index as usize][real_index as usize] = '#';
            } else {
                pixels[parent_array_index as usize][real_index as usize] = '.';
            }

            if must_break {
                break;
            }
            current_cycles += 1;
            cycles += 1;
        }
    }

    // println!("{:?}", strengths);
    // println!("Sum of power cycles is {}", strengths.iter().sum::<i64>());
    for line in pixels {
        let s: String = line.into_iter().collect();
        println!("{s}");
    }
}
