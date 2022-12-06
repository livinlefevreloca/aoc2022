#![feature(array_chunks)]
use std::env::args;

use crate::problems::{problem1, problem2, problem3, problem4, problem5, problem6};
use crate::solution::Solution;

mod solution;
mod problems;

fn main() {
    
    let arguments: Vec<String> = args().collect();
    
    match arguments[1].as_str() {
        "1" => problem1::Problem { name: "problem1".to_string() }.get_output(false),
        "2" => problem2::Problem { name: "problem2".to_string() }.get_output(false),
        "3" => problem3::Problem { name: "problem3".to_string() }.get_output(false),
        "4" => problem4::Problem { name: "problem4".to_string() }.get_output(false),
        "5" => problem5::Problem { name: "problem5".to_string() }.get_output(false),
        "6" => problem6::Problem { name: "problem6".to_string() }.get_output(false),
        _ => eprintln!("Unknown problem number {}", arguments[1])
    }
}
