use std::str::FromStr;
use std::iter;
use std::collections::VecDeque;

#[derive(Debug)]
enum Instruction {
    No,
    Add(Add)
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split(" ").collect();
        match s[0] {
            "addx" => {
                Ok(Instruction::Add(Add {
                    value: s[1].to_string().parse::<i32>().unwrap()
                }))
            },
            _ => {
                Ok(Instruction::No)
            }
        }
    }
}

#[derive(Debug)]
struct Add {
    value: i32
}

#[derive(Debug)]
struct Register {
    x: i32
}

impl Register {
    fn new() -> Register {
        Register { x: 1 }
    }
}

#[derive(Debug)]
struct Time(i32);

impl Time {
    fn new() -> Time {
        Time(0)
    }
    fn from(y: i32) -> Time {
        Time(y)
    } 
    fn tick(&mut self, step: i32) {
        self.0 += step;
    }
}

fn main() {
    let input = include_str!("input2.test");

    let mut curr_time = Time::new();
    let mut register = Register::new();
    let mut signal_strength = 0;
    
    let mut signal_steps = iter::repeat(1).take(100).enumerate().map(|(i,_x)| (19+(i)*40) as i32).collect::<VecDeque<i32>>();

    input.lines().enumerate().for_each(|x| {
        if curr_time.0 >= signal_steps[0] {
            signal_strength += (curr_time.0 + 1)*register.x;
            println!("Register : {} Time: {} Signal_Strength : {}", register.x, curr_time.0, signal_strength);
            signal_steps.pop_front();
        }
        
        let instruction = x.1.parse::<Instruction>().unwrap();

        match instruction {
            Instruction::Add(Add {value: y}) => { 
                register.x += y;
                curr_time.tick(2);
            },
            _ => curr_time.tick(1)
        }
        
        curr_time.tick(1);
    });
    
    println!("{:?}",register);
}

