use std::str::FromStr;
use std::iter;
use std::cmp::min;

#[derive(Debug)]
struct Stacks {
    stacks: Vec<String>
}

#[derive(Debug)]
struct Move {
    query: Vec<i32>
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut query: Vec<i32> = Vec::new();
        s.split(" ").for_each(|x| {
            if let Ok(y) = x.parse::<i32>(){
                query.push(y);
            }
        });

        Ok(Move {
            query  
        })
    }
}

impl Stacks {
    fn move_query(&mut self, q: Move){
        let remove_string: &mut String  = self.stacks.get_mut(q.query[1] as usize - 1).unwrap();
        let remove_string: String = remove_string.drain(0..q.query[0] as usize).collect();
        
        // Dont Use rev() in Part B
        self.stacks[q.query[2] as usize - 1 ] = remove_string.chars().rev().collect::<String>().trim().to_owned() + self.stacks[q.query[2] as usize - 1].trim_start();

    }

    }

fn main() {
    let input = include_str!("input.prod");

    // Part A
    let mut stacks: Vec<String> = iter::repeat(String::new()).take(9).collect();
    let mut stack_count = 0;
    let _part_a = input.lines()
                        .take(8)
                        .for_each(|x| {
                            // let mut curr = String::new();
                            for i in 1..x.len() {
                                if (i-1)%4 == 0 {
                                    // curr.push(x.chars().nth(i).unwrap());
                                    stacks[stack_count].push(x.chars().nth(i).unwrap());
                                    stack_count+=1;
                                }
                            } 
                            stack_count = 0;
                            // println!("{}",curr);
                        });

    let mut stacks: Stacks = Stacks { stacks: stacks.iter().map(|x| x.trim().to_string()).collect::<Vec<String>>() };
    println!("{:?}", stacks);
    let _part_a = input.lines().skip(10)
                                .for_each(|x| {
                                    //println!("{:?}",x.parse::<Move>().unwrap());
                                    println!("{:?}",x.parse::<Move>().unwrap());
                                    stacks.move_query(x.parse::<Move>().unwrap());
                                    println!("{:?}",stacks);
                                });

    stacks.stacks.iter().for_each(|x| print!("{}",x.chars().nth(0).unwrap()));
}
