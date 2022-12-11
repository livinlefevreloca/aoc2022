use crate::solution::Solution;
use std::{fs::File, io::Read};
use std::collections::HashSet;


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


pub struct Problem {
    pub name: String
}


impl Solution for Problem {
    
    type Input = Vec<String>;
    type Output = u32;

    fn get_input_file(&self, test: &str) -> String {
        let file_name = if test == "true" {
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
        data.iter().fold(0, |acc, line| {
            let (first, last) = line.split_at(line.len() / 2);
            let first_set: HashSet<char> =  HashSet::from_iter(first.chars());
            let last_set =  HashSet::from_iter(last.chars());
            let item: char = *first_set.intersection(&last_set).collect::<Vec<&char>>()[0];
            let priority = get_priorty(item);
            acc + priority
        })

    }

    fn solve2(&self, data: &Self::Input) -> Self::Output {
        data.array_chunks::<3>().fold(
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
        )
    }
}
