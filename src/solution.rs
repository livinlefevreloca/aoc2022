use std::fmt::Display;

pub trait Solution {
    
    type Input;
    type Output: Display;
    
    fn parse_input(&self, file_path: &str) -> Self::Input;
    fn solve1(&self, data: &Self::Input) -> Self::Output;
    fn solve2(&self, data: &Self::Input) -> Self::Output;
    fn get_input_file(&self, test: &str) -> String;

    fn get_output(&self, test: &str) {
        
        let file_path = self.get_input_file(test);
        let data = self.parse_input(&file_path);
        println!("Solution to part 1: {}", self.solve1(&data));
        println!("Solution to part 2: {}", self.solve2(&data));
    }

}
