use crate::solution::Solution;
use std::{fs::File, io::Read};
use lazy_static::lazy_static;
use regex::Regex;


lazy_static! {
    static ref NUMS_RE: Regex = Regex::new(r"\d+").unwrap();
}


pub struct Problem {
    pub name: String
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
        let mut overlaps: u32 = 0;
        for line in data {
            let nums: Vec<u32> = NUMS_RE.captures_iter(line).map(
                |n| n.get(0).unwrap().as_str().parse::<u32>().unwrap()
            ).collect();
            
            if
                (nums[0] <= nums[2] && nums[3] <= nums[1]) || 
                (nums[2] <= nums[0] && nums[1] <= nums[3])
            {
                overlaps += 1;
            }
        }

        overlaps
    }

    fn solve2(&self, data: &Self::Input) -> Self::Output {
        let mut overlaps: u32 = 0;
        for line in data {
            let nums: Vec<u32> = NUMS_RE.captures_iter(line).map(
                |n| n.get(0).unwrap().as_str().parse::<u32>().unwrap()
            ).collect();
            
            if
                nums[1] >= nums[2] && nums[1] <= nums[3] || 
                nums[3] >= nums[0] && nums[3] <= nums[1]
            {
                overlaps += 1;
            }
        }

        overlaps
    }
}
