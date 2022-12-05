#![feature(iter_next_chunk)]
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Rucksack {
    compartment1: String,
    compartment2: String,
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 == 0 {
            let (comp1, comp2) = s.split_at(s.len() / 2);
            return Ok(Rucksack {
                compartment1: comp1.to_string(),
                compartment2: comp2.to_string(),
            });
        } else {
            unreachable!();
        }
    }
}

impl Rucksack {
    fn common_type(&self) -> Vec<char> {
        let set: HashSet<char> = HashSet::from_iter(self.compartment2.chars());
        let mut ans = Vec::new();
        self.compartment1.chars().for_each(|x| {
            if set.contains(&x) {
                ans.push(x);
            }
        });

        return ans;
    }

    fn priority(x: char) -> i32 {
        let _priorities = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

        _priorities.find(x).unwrap() as i32
    }

    fn common_type_part_b(arr: [&str; 3]) -> char {
        let set1: HashSet<char> = HashSet::from_iter(arr[0].chars());
        let set2: HashSet<char> = HashSet::from_iter(arr[1].chars());
        
        let mut ans = '\0';
        arr[2].chars().for_each(|x| {
            if set1.contains(&x) && set2.contains(&x) {
                ans = x;
            }
        });

        ans
    }

}

fn main() {
    let (mut res1, mut res2) = (0,0);
    let input = include_str!("input.prod");
    
    // Part A
    let _content = input.lines().for_each(|x| {
        res1 += Rucksack::priority(x.parse::<Rucksack>().unwrap().common_type()[0]) + 1
    });

    // Part B
    let mut input = input.lines();
    while let Ok(x) = input.next_chunk::<3>() {
        res2 += Rucksack::priority(Rucksack::common_type_part_b(x)) + 1;
    }
        
    println!("PartA: {}", res1);
    println!("PartB: {}", res2);
}
