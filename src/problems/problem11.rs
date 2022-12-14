use crate::solution::Solution;
use std::{fs::File, io::Read, ops::Index};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref ITEMS_REGEX: Regex = Regex::new(r"Starting items: (.*?)$").unwrap();
}

lazy_static! {
    static ref ID_REGEX: Regex = Regex::new(r"Monkey (\d+):").unwrap();
}

lazy_static! {
    static ref TEST_REGEX: Regex = RegexBuilder::new(
        r"Test: divisible by (\d+)\n.*(\d+).*(\d+)"
        ).multi_line(true).dot_matches_new_line(true).build().unwrap();
}
    

pub struct Problem {
    pub name: String
}

pub struct Monkey {
    id: u64,
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> u64>,
    test_num: u64,
    handled: u64,
}

impl Monkey {

    fn new(id: u64, items: Vec<u64>, operation_str: String, test_tup: (u64, u64, u64)) -> Monkey {
        
        let op_num = operation_str.split_once(' ').unwrap();
        let handled = 0;
        let operation: Box<dyn Fn(u64) -> u64> = match op_num {
            ("old", "+") => Box::new(|n: u64| n + n),
            (num_str, "+") => { 
                let num = num_str.parse::<u64>().unwrap();
                Box::new(move |n: u64| n + num)
            },
            ("old", "*") => Box::new(|n: u64| n * n),
            (num_str, "*") =>  {
                let num = num_str.parse::<u64>().unwrap();
                Box::new(move |n: u64| n * num)
            },
            _ => unreachable!(),
        };
        
        Monkey { 
            id, 
            items, 
            operation ,
            test: Box::new(move |w| {
                if w % test_tup.0 == 0 {
                    test_tup.1
                } else {
                    test_tup.2
                }
            }),
            test_num: test_tup.0,
            handled,
        }
    }
    
}


impl Solution for Problem {
    
    type Input = String;
    type Output = u64;

    fn get_input_file(&self, test: &str) -> String {
        let file_name = if test == "true" {
            "test.txt"
        } else {
            "input.txt"
        };

        format!("inputs/{}/{}", self.name, file_name)
    }

    fn parse_input(&self, file_path: &str) -> Self::Input {
        let mut data: String = String::new();
        let mut file = File::open(file_path).unwrap();
        file.read_to_string(&mut data).unwrap();
       
        data
    }

    fn solve1(&self, data: &Self::Input) -> Self::Output {
        let mut monkeys: Vec<Monkey> = data.split("\n\n").map(|m| {
            let lines: Vec<&str> = m.split('\n').into_iter().collect();
            let id = ID_REGEX.captures(lines[0]).unwrap().index(1).parse::<u64>().unwrap();
            let items: Vec<u64> = ITEMS_REGEX.captures(lines[1]).unwrap()
                .index(1)
                .split(", ")
                .map(
                    |n| n.parse::<u64>().unwrap()
                ).collect();
            let operation_str = lines[2]
                .split(' ')
                .rev()
                .take(2)
                .collect::<Vec<&str>>()
                .join(" ");
            
            let slice = &lines[3..6].join("\n");
            let test_match = TEST_REGEX.captures(slice).unwrap();
            let test_parts: Vec<u64> = test_match.iter().skip(1).map(
                |s| s.unwrap().as_str().parse::<u64>().unwrap()
            ).collect();
            
            let test_tup: (u64, u64, u64) = (test_parts[0], test_parts[1], test_parts[2]); 

            Monkey::new(id, items, operation_str, test_tup)
        }).collect();
        
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let items: Vec<u64> = monkeys[i].items.to_vec();
                for item in items.iter().rev() {
                    let mut new_item = (*monkeys[i].operation)(*item);
                    new_item /= 3;
                    let dest: u64 = (*monkeys[i].test)(new_item);
                    monkeys[dest as usize].items.push(new_item);
                    monkeys[i].handled +=1;
                    monkeys[i].items.pop();
                }
            }
        }
        
        let mut handled = monkeys
            .iter()
            .map(|m| m.handled )
            .collect::<Vec<u64>>();

        handled.sort_by(|a, b| b.cmp(a));

        handled[0] * handled[1]

    }

    fn solve2(&self, data: &Self::Input) -> Self::Output {
        let mut monkeys: Vec<Monkey> = data.split("\n\n").map(|m| {
            let lines: Vec<&str> = m.split('\n').into_iter().collect();
            let id = ID_REGEX.captures(lines[0]).unwrap().index(1).parse::<u64>().unwrap();
            let items: Vec<u64> = ITEMS_REGEX.captures(lines[1]).unwrap()
                .index(1)
                .split(", ")
                .map(
                    |n| n.parse::<u64>().unwrap()
                ).collect();
            let operation_str = lines[2]
                .split(' ')
                .rev()
                .take(2)
                .collect::<Vec<&str>>()
                .join(" ");
            
            let slice = &lines[3..6].join("\n");
            let test_match = TEST_REGEX.captures(slice).unwrap();
            let test_parts: Vec<u64> = test_match.iter().skip(1).map(
                |s| s.unwrap().as_str().parse::<u64>().unwrap()
            ).collect();
            
            let test_tup: (u64, u64, u64) = (test_parts[0], test_parts[1], test_parts[2]); 

            Monkey::new(id, items, operation_str, test_tup)
        }).collect();
        
        let super_mod: u64 = monkeys.iter().fold(1, |acc, m| acc * m.test_num); 

        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                let items: Vec<u64> = monkeys[i].items.to_vec();
                for item in items.iter().rev() {
                    let mut new_item = (*monkeys[i].operation)(*item);
                    new_item %= super_mod;
                    let dest: u64 = (*monkeys[i].test)(new_item);
                    monkeys[dest as usize].items.push(new_item);
                    monkeys[i].handled +=1;
                    monkeys[i].items.pop();
                }
            }
        }
        
        let mut handled = monkeys
            .iter()
            .map(|m| m.handled )
            .collect::<Vec<u64>>();

        handled.sort_by(|a, b| b.cmp(a));
        handled[0] * handled[1]
    }
}
