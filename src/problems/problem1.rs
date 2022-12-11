use crate::solution::Solution;
use std::{fs::File, io::Read};

pub struct Problem {
    pub name: String
}

impl Solution for Problem {
    
    type Input = Vec<u32>;
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

        data
            .trim()
            .split("\n\n")
            .map(
                |amount| amount.parse::<u32>().unwrap()
            ).collect()
    }

    fn solve1(&self, data: &Self::Input) -> Self::Output {
        let output = self.solve2(data);
        let (first, _) = output.split_once(',').unwrap();
        first.to_string()
    }

    fn solve2(&self, data: &Self::Input) -> Self::Output {
       let result = data.iter().fold(
           vec![0,0,0], |mut acc: Vec<u32>, candidate| {
                for slot in acc.iter_mut() {
                    if candidate > slot {
                        *slot = *candidate
                    }
                }
                acc
            }
        );

       format!("{}, {}, {}", result[0], result[1], result[2])
    }
}
