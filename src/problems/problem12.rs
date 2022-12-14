use crate::solution::Solution;
use std::{fs::File, io::Read};


pub struct Problem {
    pub name: String
}

static START: char = 'S';
static END: char = 'E';

fn find_start_and_end(map: &[Vec<char>]) -> [(usize, usize); 2] {
    
    let mut start_end = [(0, 0); 2];

    for (i, row) in map.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == START {
                start_end[0].0 = i;
                start_end[0].1 = j;
            } else if *ch == END {
                start_end[1].0 = i;
                start_end[1].1 = j;
            }
        }
    }

    start_end
}

fn get_possible_directions(map: &[Vec<char>], (i, j): (usize, usize)) -> Vec<(usize, usize)> {

    let place_char: char = map[i][j];
    let mut options: Vec<(usize, usize)> = Vec::with_capacity(4);

    for (ip, jp) in &[(i, j-1), (i-1, j), (i, j+1), (i+1, j)] {
        if (place_char as i8 - map[*ip][*jp] as i8).abs() < 1 {
            options.push((*ip, *jp))
        }
    }
    
    options
}


impl Solution for Problem {
    
    type Input = Vec<Vec<char>>;
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

        data.split('\n').map(|line| line.chars().collect::<Vec<char>>()).collect()
    }

    fn solve1(&self, data: &Self::Input) -> Self::Output {
        let [(s_x, s_y), (e_x, e_y)] = find_start_and_end(data);
        let current = (s_x, s_y);
        let options = get_possible_directions(data, current);
        


        todo!()
    }

    fn solve2(&self, data: &Self::Input) -> Self::Output {
        todo!()
    }
}
