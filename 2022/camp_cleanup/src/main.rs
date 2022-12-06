use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [start,end] = s.split('-').collect::<Vec<&str>>()[..] else {todo!()};

        return Ok(Range {
            start: start.parse::<i32>().unwrap(),
            end:  end.parse::<i32>().unwrap()
        });
    }
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        let Range {start: x1,end: x2} = self;
        let Range {start: y1,end: y2} = other;

        (y1 >= x1 && y2 <= x2 )  || ( x1 >= y1 && x2 <= y2 )
    }

    fn has_common(&self, other: &Self) -> bool {
        for i in other.start..other.end+1 {
            if i >= self.start && i <= self.end {
                return true;
            }
        }
        false
    }
}

fn main() {
    let input = include_str!("input.prod");

    // Part A
    let mut count = 0;
    let _part_a = input.lines().for_each(|x| {
        let [elf1, elf2] = &x.split(',').map(|x| x.parse::<Range>().unwrap()).collect::<Vec<Range>>()[..] else {todo!()};
        if elf1.contains(&elf2) {
            count+=1;
        } 
    });
    println!("Count: {}", count);

    // Part B
    count = 0;
    let _part_b = input.lines().for_each(|x| {
        let [elf1, elf2] = &x.split(',').map(|x| x.parse::<Range>().unwrap()).collect::<Vec<Range>>()[..] else {todo!()};
        if elf1.has_common(elf2) {
            count+=1;
        }
    });
    println!("Count: {}",count);
}
