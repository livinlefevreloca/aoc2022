use crate::solution::Solution;
use std::{fs::File, io::Read};
use std::collections::HashMap;


pub struct Problem {
    pub name: String
}

#[derive(Debug)]
pub enum Op {
    Add(i32),
    Noop
}


impl Solution for Problem {
    
    type Input = Vec<Op>;
    type Output = i32;

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

        data.trim().split('\n').map(|line| {
            match line {
                "noop" => Op::Noop,
                add => Op::Add(
                    add.split_once(' ').unwrap().1
                        .parse::<i32>().unwrap()
                ),
            }
        }).collect()
    }

    fn solve1(&self, data: &Self::Input) -> Self::Output {
        let mut counter: usize = 0;
        let mut x: i32 = 1;
        let mut results: HashMap<usize, i32> = HashMap::new();
        let mut current_op: Option<&Op> = None;
        let mut int_idx = 0;

        loop {
            
            let exec = match current_op {
                Some(op) => op,
                None => &data[int_idx as usize],
            };

            counter += 1;
            
            match exec {
                Op::Noop => {
                    int_idx += 1;
                    results.insert(counter, x);
                },
                Op::Add(amount) => {
                    results.insert(counter, x);
                    if current_op.is_some() {
                        x += amount;
                        int_idx += 1;
                        current_op = None;
                    } else {
                        current_op = Some(exec);
                    }
                }
            }

            if int_idx == data.len() {break};
        }

        results.iter().filter(
             |(c, _)| vec![20, 60, 100, 140, 180, 220].contains(c)
         ).map(|(c, x)| x * *c as i32).sum()
    }

    // fn solve1(&self, data: &Self::Input) -> Self::Output {
    //     let mut counter: usize = 0;
    //     let mut x: i32 = 1;
    //     let mut results: HashMap<usize, i32> = HashMap::new();

    //     for int in data {
    //         match int.split_once(' ') {
    //             None => {
    //                 counter +=1;
    //                 results.insert(counter, x);
    //             },
    //             Some(("addx", num)) => {
    //                 counter += 1;
    //                 results.insert(counter, x);
    //                 counter += 1;
    //                 results.insert(counter, x);
    //                 x += num.parse::<i32>().unwrap();
    //             },
    //             Some((_, _)) => unreachable!(),
    //         }

    //     }
    //     
    //     results.iter().filter(
    //         |(c, _)| vec![20, 60, 100, 140, 180, 220].contains(c)
    //     ).map(|(c, x)| x * *c as i32).sum()
    // }

    fn solve2(&self, data: &Self::Input) -> Self::Output {
        let mut counter: usize = 0;
        let mut x: i32 = 1;
        let mut current_op: Option<&Op> = None;
        let mut int_idx = 0;
        let mut result: Vec<String> = Vec::new();
        let mut current_row = String::with_capacity(40);

        loop {
            
            let exec = match current_op {
                Some(op) => op,
                None => &data[int_idx as usize],
            };

            
            if (counter % 40) as i32 >= x - 1 && (counter % 40) as i32 <= x + 1 {
                current_row.push('#');
            } else {
                current_row.push('.');
            }

            counter += 1;

            match exec {
                Op::Noop => {
                    int_idx += 1;
                },
                Op::Add(amount) => {
                    if current_op.is_some() {
                        x += amount;
                        int_idx += 1;
                        current_op = None;
                    } else {
                        current_op = Some(exec);
                    }
                }
            }

            
            if counter % 40 == 0 {
                result.push(current_row);
                current_row = String::with_capacity(40);
            }

            if int_idx == data.len() {break};
        }

        println!("{}", result.join("\n"));

        0
        
    }
}
