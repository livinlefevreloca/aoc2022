#![feature(array_chunks)]
use std::env::args;

use crate::problems::{
    problem1, problem2, problem3, problem4, problem5, problem6, problem7, problem8, problem9, problem10, problem11, problem12
};
use crate::solution::Solution;

mod solution;
mod problems;

fn main() {
    
    let mut arguments: Vec<String> = args().collect();

    if arguments.len() < 3 {
        arguments.push("false".to_string());
    }

    match arguments[1].as_str() {
        "1" => problem1::Problem { name: "problem1".to_string() }.get_output(arguments[2].as_str()),
        "2" => problem2::Problem { name: "problem2".to_string() }.get_output(arguments[2].as_str()),
        "3" => problem3::Problem { name: "problem3".to_string() }.get_output(arguments[2].as_str()),
        "4" => problem4::Problem { name: "problem4".to_string() }.get_output(arguments[2].as_str()),
        "5" => problem5::Problem { name: "problem5".to_string() }.get_output(arguments[2].as_str()),
        "6" => problem6::Problem { name: "problem6".to_string() }.get_output(arguments[2].as_str()),
        "7" => problem7::Problem { name: "problem7".to_string() }.get_output(arguments[2].as_str()),
        "8" => problem8::Problem { name: "problem8".to_string() }.get_output(arguments[2].as_str()),
        "9" => problem9::Problem { name: "problem9".to_string() }.get_output(arguments[2].as_str()),
        "10" => problem10::Problem { name: "problem10".to_string() }.get_output(arguments[2].as_str()),
        "11" => problem11::Problem { name: "problem11".to_string() }.get_output(arguments[2].as_str()),
        _ => eprintln!("Unknown problem number {}", arguments[1])
    }
}
