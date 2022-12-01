use std::{fs::File, io::Read};
use std::mem::replace;

fn main() {
    problem1("inputs/problem1/input.txt")
}


fn problem1(file_name: &str) {
    
    let mut data = String::new();
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut data).unwrap();

    let elves: Vec<&str> = data.trim().split("\n\n").collect();
    
    let mut max: u32 = 0;
    let mut top_three: Vec<u32> = vec![0,0,0];
    for elf in elves {
        let total: u32 = elf.split('\n').map(|amount| amount.parse::<u32>().unwrap()).sum();
        if total > max {
            max = total
        }
        
        let mut candidate = total;
        for slot in top_three.iter_mut() {
            if candidate > *slot {
                candidate = replace(slot, candidate);
            }
        }
    }

    println!("Largest total is: {}", max);
    println!("Largest 3 totals summed are: {}", top_three.iter().sum::<u32>());


}
