use std::{fs::File, io::Read};
use std::mem::replace;
use std::env::args;

fn main() {
    
    let arguments: Vec<String> = args().collect();
    
    match arguments[1].as_str() {
        "1" => problem1("inputs/problem1/input.txt"),
        "2" => problem2("inputs/problem2/input.txt"),
        _ => eprintln!("Unknown problem number {}", arguments[1])
    }

}


fn problem1(file_name: &str) {
    
    let mut data = String::new();
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut data).unwrap();

    let elves: Vec<&str> = data.trim().split("\n\n").collect();
    
    let mut top_three: Vec<u32> = vec![0,0,0];
    for elf in elves {
        let mut candidate: u32 = elf.split('\n').map(|amount| amount.parse::<u32>().unwrap()).sum();
        
        for slot in top_three.iter_mut() {
            if candidate > *slot {
                candidate = replace(slot, candidate);
            }
        }
    }

    println!("Largest total is: {}", top_three[0]);
    println!("Largest 3 totals summed are: {}", top_three.iter().sum::<u32>());
}

fn problem2(file_path: &str) {
    
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

    fn winner(me: &RPS, them: &RPS) -> i32 {
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
    
    fn get_value(play: &RPS) -> i32 {
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
    
    fn get_round_score(me_letter: &str, them_letter: &str) -> i32 {
        let me_play = get_play(me_letter);
        let them_play = get_play(them_letter);
        
        winner(&me_play, &them_play) + get_value(&me_play)
    }

    fn get_ideal_round_score(me_letter: &str, them_letter: &str) -> i32 {
        let them_play = get_play(them_letter);
        let me_required = get_required_result(me_letter);
        let me_play = get_required_play(&them_play, me_required);
        winner(&me_play, &them_play) + get_value(&me_play)
    }

    let mut file = File::open(file_path).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();
    
    let mut first_total = 0;
    let mut second_total = 0;
    for line in data.trim().split('\n') {
        let letters: Vec<&str> = line.split(' ').collect();
        first_total += get_round_score(letters[1], letters[0]);
        second_total += get_ideal_round_score(letters[1], letters[0]);
    }

    println!("First Total score is: {}", first_total);
    println!("Second Total score is: {}", second_total);
}

