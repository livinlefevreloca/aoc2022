#![feature(iter_array_chunks)]
use std::num;
use std::{fs::File, io::Read};
use std::mem::replace;
use std::env::args;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    
    let arguments: Vec<String> = args().collect();
    
    match arguments[1].as_str() {
        "1" => problem1("inputs/problem1/input.txt"),
        "2" => problem2("inputs/problem2/input.txt"),
        "3" => problem3("inputs/problem3/input.txt"),
        "4" => problem4("inputs/problem4/input.txt"),
        _ => eprintln!("Unknown problem number {}", arguments[1])
    }

}


fn problem1(file_name: &str) {
    
    let mut data = String::new();
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut data).unwrap();

    let top_three = data
        .trim()
        .split("\n\n")
        .fold(vec![0,0,0], |mut acc, val|{
            let mut candidate = val.split('\n').map(|amount| amount.parse::<u32>().unwrap()).sum();
            for slot in acc.iter_mut() {
                if candidate > *slot {
                    candidate = replace(slot, candidate);
                }
            }
            acc
        });

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

fn problem3(file_path: &str) {
    
    let mut file = File::open(file_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    fn get_priorty(item: char) -> u32 {
        match item {
            item if item.is_uppercase() => {
                let value: u32 = item.into();
                value - 38
            },
            item if item.is_lowercase() => {
                let value: u32 =  item.into();
                value - 96
            },
            _ => unreachable!(),
        }
    }

    let total = data
        .trim()
        .split('\n')
        .fold(0, |acc, line| {
            let (first, last) = line.split_at(line.len() / 2);
            let first_set: HashSet<char> =  HashSet::from_iter(first.chars());
            let last_set =  HashSet::from_iter(last.chars());
            let item: char = *first_set.intersection(&last_set).collect::<Vec<&char>>()[0];
            let priority = get_priorty(item);
            acc + priority
        });

    let group_total = data.trim().split('\n').array_chunks::<3>().fold(
        0,
        |acc, groups| {
            let letter: char = groups
                .into_iter()
                .map(|s| HashSet::from_iter(s.chars()))
                .reduce(
                    |acc: HashSet<char, _>, group: HashSet<char, _>| {
                            group.intersection(&acc).copied().collect::<HashSet<char>>()
                    }
                ).unwrap().into_iter().collect::<Vec<char>>()[0];
            
            acc + get_priorty(letter)
        }
    );


    println!("Total priority is: {}", total);
    println!("Group total priorities is: {}", group_total);
}

fn problem4(file_path: &str) {
    let mut data = String::new();
    let mut file = File::open(file_path).unwrap();
    file.read_to_string(&mut data).unwrap();

    let nums_re = Regex::new(r"\d+").unwrap();
    
    let mut full_overlaps: u32 = 0;
    let mut overlaps: u32 = 0;
    for line in data.trim().split('\n') {

        let nums: Vec<u32> = nums_re.captures_iter(line).map(
            |n| n.get(0).unwrap().as_str().parse::<u32>().unwrap()
        ).collect();
        
        if
            (nums[0] <= nums[2] && nums[3] <= nums[1]) || 
            (nums[2] <= nums[0] && nums[1] <= nums[3])
        {
            full_overlaps += 1;
            overlaps += 1;
        } else if 
            nums[1] >= nums[2] && nums[1] <= nums[3] || 
            nums[3] >= nums[0] && nums[3] <= nums[1] {
            overlaps += 1;
        }
    }

    println!("Found {} full overlaps", full_overlaps);
    println!("Found {} overlaps", overlaps);
        
}
