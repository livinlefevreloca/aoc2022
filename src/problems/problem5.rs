use crate::solution::Solution;
use std::{fs::File, io::Read};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MOVE_RE: Regex = Regex::new(
        r"move (\d+) from (\d+) to (\d+)"
    ).unwrap();
}


pub struct Problem {
    pub name: String
}

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_stacks(lines: &Vec<String>) -> Vec<Vec<char>> {
    let last_line = lines.last().unwrap();
    let stacks_count = last_line.split_whitespace().count();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..stacks_count {
        stacks.push(Vec::new());
    }
    

    for line in lines[0..lines.len()-1].iter() {
        let mut current_stack = 0;
        let mut idx = 1;
        loop {
            let letter = line.chars().nth(idx).unwrap();
            if ! letter.is_whitespace() {
                stacks[current_stack].push(letter);
            }

            current_stack += 1;
            idx += 4;

            if current_stack == stacks_count {
                break
            }
        }
    }
    
    stacks
}

fn parse_instructions(lines: &[String]) -> Vec<Move> {
    
    lines.iter().map(|s| {
        let found = MOVE_RE.captures(s).unwrap();
        Move {
            amount: found.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            from: found.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
            to: found.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
        }
    }).collect()
}

fn do_move(stacks: &mut [Vec<char>], m: Move, multi: bool) {

    let to_move: std::vec::Drain<char> = stacks[m.from].drain(0..m.amount);
    
    let mut moved: Vec<char> = if ! multi {
        to_move.rev().collect()
    } else {
        to_move.collect()
    };

    moved.extend(stacks[m.to].clone());
    stacks[m.to] = moved;
}


impl Solution for Problem {
    
    type Input = (Vec<String>, Vec<String>);
    type Output = String;

    fn get_input_file(&self, test: &str) -> String {
        let file_name = if test == "true" {
            "test.txt"
        } else {
            "input.txt"
        };

        format!("inputs/{}/{}", self.name, file_name)
    }

    fn parse_input(&self, file_path: &str) -> Self::Input {
        let mut data = String::new();
        let mut file = File::open(file_path).unwrap();
        file.read_to_string(&mut data).unwrap();

        let (stacks_str, moves_str) =  data.split_once("\n\n").unwrap();
        (stacks_str.lines().map(|s| s.to_string()).collect(), moves_str.lines().map(|s| s.to_string()).collect())
        
    }

    fn solve1(&self, data: &Self::Input) -> Self::Output {
        let (stack_lines, move_lines) = data; 
        let moves = parse_instructions(move_lines);
        let mut stacks = parse_stacks(stack_lines);
        
        for m in moves {
            do_move(&mut stacks, m, false);
        }

        stacks.iter().map(|stack| stack[0]).collect()
    }

    fn solve2(&self, data: &Self::Input) -> Self::Output {
        let (stack_lines, move_lines) = data; 
        let moves = parse_instructions(move_lines);
        let mut stacks = parse_stacks(stack_lines);
        
        for m in moves {
            do_move(&mut stacks, m, true);
        }

        stacks.iter().map(|stack| stack[0]).collect()
    }
}
