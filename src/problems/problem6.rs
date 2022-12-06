use crate::solution::Solution;
use std::{fs::File, io::Read};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref MOVE_RE: Regex = Regex::new(
        r"move (\d+) from (\d+) to (\d+)"
    ).unwrap();
}


pub struct Problem {
    pub name: String
}


impl Solution for Problem {
    
    type Input = String;
    type Output = i32;

    fn get_input_file(&self, test: bool) -> String {
        let file_name = if test {
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
        data
    }

    fn solve1(&self, data: &Self::Input) -> Self::Output {
        let unique_count = 4;
        let mut prev: Vec<char> = Vec::with_capacity(unique_count);
        
        for (idx, ch) in data.chars().enumerate() {
            prev.push(ch);
            if prev.len() == unique_count {
                let set: HashSet<&char> = prev.iter().collect();
                if set.len() == unique_count {
                   return idx as i32 + 1 
                }

                prev = prev.drain(1..unique_count).collect();
            }
        }
        -1
    }

    fn solve2(&self, data: &Self::Input) -> Self::Output {
        let unique_count = 14;
        let mut prev: Vec<char> = Vec::with_capacity(unique_count);
        
        for (idx, ch) in data.chars().enumerate() {
            prev.push(ch);
            if prev.len() == unique_count {
                let set: HashSet<&char> = prev.iter().collect();
                if set.len() == unique_count {
                   return idx as i32 + 1 
                }

                prev = prev.drain(1..unique_count).collect();
            }
        }
        -1
    }
}
