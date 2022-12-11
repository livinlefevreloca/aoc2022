use crate::solution::Solution;
use std::{fs::File, io::Read};


pub struct Problem {
    pub name: String
}


#[derive(Debug)]
pub struct Node {
    name: String,
    size: usize
}


impl Solution for Problem {
    
    type Input = Vec<Node>;
    type Output = usize;

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

        let mut active: Vec<Node> = Vec::new();
        let mut complete: Vec<Node> = Vec::new();
        
        let mut current_node: Option<Node> = None;

        for line in data.lines() {
            if line.starts_with("$ cd") {

                let name = line.split_whitespace().last().unwrap();
                let size: usize = 0;
                
                if name == ".." {
                    complete.push(current_node.take().unwrap());
                    current_node = active.pop();
                } else {
                    if current_node.is_some() {
                        active.push(current_node.take().unwrap());
                    }
                    current_node = Some(Node{ name: name.to_string(), size })
                }
            } else if line.starts_with("$ ls") || line.starts_with("dir") {
                continue;
            } else {
                let (size_str, _) = line.split_once(' ').unwrap();
                
                let size: usize = size_str.parse::<usize>().unwrap();
                current_node = current_node.map(|mut node| {
                        node.size += size;
                        node
                    }
                );
                active.iter_mut().for_each(|node| node.size += size);
            }
        }
        
        complete.extend(active);
        complete
    }

    fn solve1(&self, data: &Self::Input) -> Self::Output {
        data.iter().filter(|node| node.size <= 100000).map(|node| node.size).sum()
    }

    fn solve2(&self, data: &Self::Input) -> Self::Output {
        let total_space = 70000000;
        let update_space = 30000000;
        let user_space = data.iter().max_by_key(|node| node.size).unwrap().size;
        let unused = total_space - user_space;
        let required = update_space - unused;
        data.iter().filter(|node| node.size >= required).min_by_key(|node| node.size).unwrap().size
    }
}
