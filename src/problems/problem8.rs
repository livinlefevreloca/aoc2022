use crate::solution::Solution;
use std::{fs::File, io::Read};


pub struct Problem {
    pub name: String
}

fn is_visible((i, j): (usize, usize), arr: &[Vec<u32>], m: usize, n: usize) -> u32 {

    let top_max = (0..i).map(|idx| arr[idx][j]).max().unwrap();
    let bottom_max = (i+1..n).map(|idx| arr[idx][j]).max().unwrap();
    let left_max = (0..j).map(|idx| arr[i][idx]).max().unwrap();
    let right_max = (j+1..m).map(|idx| arr[i][idx]).max().unwrap();
    let height = arr[i][j];
    
    if height > top_max || height > bottom_max || height > left_max || height > right_max {1} else {0}
}

fn get_score((i,j): (usize, usize), arr: &[Vec<u32>], m: usize, n: usize) -> usize {
    let height = arr[i][j];
    let mut top_dist = (0..i).rev().take_while(|idx| arr[*idx][j] < height).count();
    let mut bottom_dist = (i+1..n).take_while(|idx| arr[*idx][j] < height).count();
    let mut left_dist = (0..j).rev().take_while(|idx| arr[i][*idx] < height).count();
    let mut right_dist = (j+1..m).take_while(|idx| arr[i][*idx] < height).count();
    
    top_dist =  if top_dist < i { top_dist + 1 } else { top_dist };
    bottom_dist =  if bottom_dist < n-i-1 { bottom_dist + 1 } else { bottom_dist };
    left_dist =  if left_dist < j { left_dist + 1 } else { left_dist };
    right_dist =  if right_dist < m-j-1 { right_dist + 1 } else { right_dist };
     
    top_dist * bottom_dist * left_dist * right_dist
}

fn perimeter(m: usize, n: usize) -> usize {
    2*(m-2) + 2*n
}


impl Solution for Problem {
    
    type Input = Vec<Vec<u32>>;
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
        let mut data: String = String::new();
        let mut file = File::open(file_path).unwrap();
        file.read_to_string(&mut data).unwrap();

        data.trim().split('\n').map(
            |l| l.trim().chars().map(
                |c| String::from(c).parse::<u32>().unwrap()
            ).collect::<Vec<u32>>()
        ).collect()
    }

    fn solve1(&self, data: &Self::Input) -> Self::Output {
        let m = data.len();
        let n = data[0].len();

        let perim = perimeter(m, n) as u32; 
        
        let inner = (1..m-1).map(|i| {
            (1..m-1).map(|j| is_visible((i, j), data, m, n)).sum::<u32>()
        }).sum::<u32>();

        println!("perim: {}, inner: {}", perim, inner);
        inner + perim
    }

    fn solve2(&self, data: &Self::Input) -> Self::Output {
        let m = data.len();
        let n = data[0].len();
        (1..n-1).flat_map(|i| {
            (1..m-1).map(|j| get_score((i,j), data, m, n)).collect::<Vec<usize>>()
        }).max().unwrap() as u32
    }
}
