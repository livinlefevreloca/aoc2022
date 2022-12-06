use crate::solution::Solution;
use std::{fs::File, io::Read};

pub struct Problem {
    pub name: String
}

#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum RequiredResult {
    Win,
    Lose,
    Draw,
}

fn winner(me: &RPS, them: &RPS) -> u32 {
    match (me, them)  {
        (RPS::Rock, RPS::Paper) => 0,
        (RPS::Rock, RPS::Rock) => 3,
        (RPS::Rock, RPS::Scissors) => 6,
        (RPS::Paper, RPS::Scissors) => 0,
        (RPS::Paper, RPS::Paper) => 3,
        (RPS::Paper, RPS::Rock) => 6,
        (RPS::Scissors, RPS::Rock) => 0,
        (RPS::Scissors, RPS::Scissors) => 3,
        (RPS::Scissors, RPS::Paper) => 6,
    }
}

fn get_play(letter: &str) -> RPS {
    match letter {
        "A" => RPS::Rock,
        "X" => RPS::Rock,
        "B" => RPS::Paper,
        "Y" => RPS::Paper,
        "C" => RPS::Scissors,
        "Z" => RPS::Scissors,
        _ => unreachable!(),
    }
}

fn get_required_result(letter: &str) -> RequiredResult {
    match letter {
        "X" => RequiredResult::Lose,
        "Y" => RequiredResult::Draw,
        "Z" => RequiredResult::Win,
        _ => unreachable!(),
    }
}

fn get_value(play: &RPS) -> u32 {
    match play {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn get_required_play(them_play: &RPS, required_result: RequiredResult) -> RPS {
    match (them_play, required_result) {
        (RPS::Rock, RequiredResult::Win) => RPS::Paper,
        (RPS::Rock, RequiredResult::Lose) => RPS::Scissors,
        (RPS::Rock, RequiredResult::Draw) => RPS::Rock,
        (RPS::Paper, RequiredResult::Draw) => RPS::Paper,
        (RPS::Paper, RequiredResult::Win) => RPS::Scissors,
        (RPS::Paper, RequiredResult::Lose) => RPS::Rock,
        (RPS::Scissors, RequiredResult::Lose) => RPS::Paper,
        (RPS::Scissors, RequiredResult::Draw) => RPS::Scissors,
        (RPS::Scissors, RequiredResult::Win) => RPS::Rock,
    }
}

fn get_round_score(me_letter: &str, them_letter: &str) -> u32 {
    let me_play = get_play(me_letter);
    let them_play = get_play(them_letter);
    
    winner(&me_play, &them_play) + get_value(&me_play)
}

fn get_ideal_round_score(me_letter: &str, them_letter: &str) -> u32 {
    let them_play = get_play(them_letter);
    let me_required = get_required_result(me_letter);
    let me_play = get_required_play(&them_play, me_required);
    winner(&me_play, &them_play) + get_value(&me_play)
}

impl Solution for Problem {
    
    type Input = Vec<String>;
    type Output = u32;

    fn get_input_file(&self, test: bool) -> String {
        let file_name = if test {
            "test.txt"
        } else {
            "input.txt"
        };

        format!("inputs/{}/{}", self.name, file_name)
    }

    fn parse_input(&self, file_path: &str) -> Self::Input {
        let mut file = File::open(file_path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        data.trim().split('\n').map(|s| s.to_string()).collect()
    }

    fn solve1(&self, data: &Self::Input) -> Self::Output {
        let mut total: u32 = 0;
        for line in data {
            let (first, second) = line.split_once(' ').unwrap();
            total += get_round_score(second, first);
        }

        total
    }

    fn solve2(&self, data: &Self::Input) -> Self::Output {
        let mut total: u32 = 0;
        for line in data {
            let (first, second) = line.split_once(' ').unwrap();
            total += get_ideal_round_score(first, second);
        }

        total
    }
}
