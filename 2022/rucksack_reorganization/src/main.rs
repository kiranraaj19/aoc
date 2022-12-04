use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    compartment1: String,
    compartment2: String
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 == 0 {
            let (comp1,comp2) = s.split_at(s.len()/2);
            return Ok(Rucksack {
                compartment1: comp1.to_string(),
                compartment2: comp2.to_string()
            });
        } else {
            unreachable!();
        }
    }
}

impl Rucksack {
    fn common_type(&self) -> char {
        let set: HashSet<char> = HashSet::from_iter(self.compartment2.chars());
        let mut ans = '\0';
        self.compartment1.chars().for_each(|x| {
            if set.contains(&x) { ans = x; } 
        });

        return ans;
    }

    fn priority(x: char) -> i32 {
    let _priorities = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    
    _priorities.find(x).unwrap() as i32

    }
}

fn main() {
    let mut sum = 0;

    let _content = include_str!("input.prod")
        .lines()
        .for_each(|x| {
            sum +=  Rucksack::priority(x.parse::<Rucksack>().unwrap().common_type())+1       
        });

    println!("{sum}");
}
