use crate::solution::Solution;
use std::{fs::File, io::Read};
use std:: collections::HashSet;


pub struct Problem {
    pub name: String
}

#[derive(Debug, Clone)]
struct Pos {
    x: i32,
    y: i32
}


impl Solution for Problem {
    
    type Input = Vec<(String, i32)>;
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
        let mut data: String = String::new();
        let mut file = File::open(file_path).unwrap();
        file.read_to_string(&mut data).unwrap();

        data.trim().split('\n').map(|line| {
            let (direction, number): (&str, &str) = line.split_once(' ').unwrap();
            let parsed = number.parse::<i32>().unwrap();
            (direction.to_string(), parsed)
        }).collect()
    }

    fn solve1(&self, data: &Self::Input) -> Self::Output {
        let (mut t, mut h): (Pos, Pos) = (Pos { x: 0, y: 0 }, Pos { x: 0, y: 0 }); 
        let mut positions: HashSet<(i32, i32)> = HashSet::new();

        for (dir, movement) in data {
            for _ in 0..*movement {
                let h_new = match dir.as_ref() {
                    "U" => Pos { x : h.x, y: h.y + 1 },
                    "D" => Pos { x: h.x, y: h.y - 1 },
                    "L" => Pos { x: h.x - 1, y: h.y },
                    "R" => Pos { x: h.x + 1, y: h.y },
                    _ => unreachable!(),
                };
                
                let (x_dist, y_dist): (i32, i32) = ((h_new.x - t.x).abs(), (h_new.y -t.y).abs());
                
                let prev_h = h;
                h = h_new;

                if  x_dist > 1 || y_dist > 1 {
                    t = prev_h;
                    positions.insert((t.x, t.y));
                }
                
            }
        }

        positions.len() as u32 + 1
    }

    fn solve2(&self, data: &Self::Input) -> Self::Output {
        let mut rope:  Vec<Pos> = (0..10).map(|_| Pos {x: 0, y: 0} ).collect();
        let mut positions: HashSet<(i32, i32)> = HashSet::new();

        for (dir, movement) in data {
            for idx in 0..*movement {
                let h_new = match dir.as_ref() {
                    "U" => Pos { x : rope[0].x, y: rope[0].y + 1 },
                    "D" => Pos { x: rope[0].x, y: rope[0].y - 1 },
                    "L" => Pos { x: rope[0].x - 1, y: rope[0].y },
                    "R" => Pos { x: rope[0].x + 1, y: rope[0].y },
                    _ => unreachable!(),
                };
                rope[0] = h_new;

                println!("dir: {}, movement: {}, idx: {}", dir, movement, idx);
                for i in 0..(rope.len() - 1) {
                    let last = &rope[i].clone();
                    let current = rope[i+1].clone();

                    

                    if last.x != current.x && last.y != current.y && ((last.x - current.x).abs() > 1 || (last.y - current.y).abs() > 1) {
                        let change = Pos { 
                            x: ((last.x - current.x) / (last.x - current.x).abs()),
                            y: ((last.y - current.y) / (last.y -current.y).abs()),
                        };
                        println!("i: {}, last: {:?}, current: {:?}, change: {:?}", i, last, current, change);
                        rope[i+1] = Pos { x: current.x + change.x, y: current.y + change.y };
                    } else if (last.x - current.x).abs() > 1 {
                        let change = (last.x - current.x) / (last.x - current.x).abs();
                        println!("i: {}, last: {:?}, current: {:?}, change: {:?}", i, last, current, change);
                        rope[i+1] = Pos { x: current.x + change, y: current.y };
                    } else if (last.y - current.y).abs() > 1 {
                        let change = (last.y - current.y) / (last.y -current.y).abs();
                        println!("i: {}, last: {:?}, current: {:?}, change: {:?}", i, last, current, change);
                        rope[i+1] = Pos { x: current.x, y: current.y + change };
                    } else {
                        break
                    }
                } 
                positions.insert(rope.last().map(|pos|(pos.x, pos.y)).unwrap());
                println!("rope: {:?}\n\n", rope.iter().enumerate().collect::<Vec<(usize, &Pos)>>());
            }    
        }
        positions.len() as u32
    }
}
