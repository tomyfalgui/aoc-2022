use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = ();
    fn from_str(input: &str) -> Result<Choice, Self::Err> {
        match input {
            "X" | "A" => Ok(Choice::Rock),
            "Y" | "B" => Ok(Choice::Paper),
            "Z" | "C" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}
// could have done
// map each line into (A, B)
// pattern matched onto (A, B) => SCISSORS + DRAW and so on
// https://github.com/gbegerow/advent-of-code/blob/main/aoc_2022_02/src/lib.rs

fn main() {
    let input = include_str!("../input.txt");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut total_score = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.trim_end().split(' ').collect();

        let opponent = Choice::from_str(split[0]).unwrap();
        let player = Choice::from_str(split[1]).unwrap();

        let score = check_decision(&opponent, &player);
        total_score += score;
        match player {
            Choice::Rock => total_score += 1,
            Choice::Paper => total_score += 2,
            Choice::Scissors => total_score += 3,
        }
    }
    println!("{}", total_score);
}

fn part_two(input: &str) {
    let mut total_score = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.trim_end().split(' ').collect();

        let opponent = Choice::from_str(split[0]).unwrap();
        let player_input = split[1];

        let (player_choice, points) = determine_required(&opponent, &player_input);
        total_score += points;
        match player_choice {
            Choice::Rock => total_score += 1,
            Choice::Paper => total_score += 2,
            Choice::Scissors => total_score += 3,
        }
    }
    println!("{}", total_score);
}

fn determine_required(opponent: &Choice, player_input: &str) -> (Choice, usize) {
    if player_input == "X" {
        // lose
        match opponent {
            Choice::Rock => (Choice::Scissors, 0),
            Choice::Paper => (Choice::Rock, 0),
            Choice::Scissors => (Choice::Paper, 0),
        }
    } else if player_input == "Y" {
        // draw
        match opponent {
            Choice::Rock => (Choice::Rock, 3),
            Choice::Paper => (Choice::Paper, 3),
            Choice::Scissors => (Choice::Scissors, 3),
        }
    } else {
        // win
        match opponent {
            Choice::Rock => (Choice::Paper, 6),
            Choice::Paper => (Choice::Scissors, 6),
            Choice::Scissors => (Choice::Rock, 6),
        }
    }
}

fn check_decision(opponent: &Choice, player: &Choice) -> usize {
    if let Choice::Rock = player {
        if let Choice::Rock = opponent {
            3
        } else if let Choice::Paper = opponent {
            0
        } else {
            6
        }
    } else if let Choice::Paper = player {
        if let Choice::Rock = opponent {
            6
        } else if let Choice::Paper = opponent {
            3
        } else {
            0
        }
    } else {
        if let Choice::Rock = opponent {
            0
        } else if let Choice::Paper = opponent {
            6
        } else {
            3
        }
    }
}
