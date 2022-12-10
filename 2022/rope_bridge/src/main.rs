use std::ops;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug,Clone,Copy)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Debug,Clone,Copy)]
struct Head(Position);

#[derive(Debug,Clone,Copy)]
struct Tail(Position);

struct Rope{
    head: Head,
    tail: Tail,
    visited: HashSet<(i32,i32)>
}

#[derive(Debug)]
struct Query(Position);

impl Position {
    fn from((x,y): (i32,i32)) -> Position {
        Position {
            x,
            y
        }
    }
}

impl ops::Add for Position {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Position::from((self.x + _rhs.x, self.y + _rhs.y))
    }
}

impl FromStr for Query {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x: Vec<&str> = s.split(" ").collect();
        let magnitude = x[1].parse::<i32>().unwrap();
        return match x[0] {
            "R" => Ok(Query(Position {x:1*magnitude,y:0})),
            "L" => Ok(Query(Position {x:-1*magnitude,y:0})),
            "U" => Ok(Query(Position {x:0,y:1*magnitude})),
            "D" => Ok(Query(Position {x:0,y:-1*magnitude})),
            _ => unreachable!()
        };
    }
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Head(Position {x:0,y:0}),
            tail: Tail(Position {x:0, y:0}),
            visited: HashSet::new()
        }
    }

    fn event(&mut self, q: Query) {
        // New position of head
        self.head.0 = self.head.0 + q.0;
        println!("Head: {:?}\n Tail: {:?}",self.head.0,self.tail.0);
        
        Self::adjust(self);
    }

    fn adjust(&mut self){
        let (h_x, h_y) = (self.head.0.x, self.head.0.y);
        let (t_x,t_y) = (self.tail.0.x,self.tail.0.y);
        if (((h_x-t_x).pow(2) + (h_y-t_y).pow(2)) as f32).sqrt() < 2.0 {
            return;
        } else {
            println!("{:?}", Direction::vector(&self));
            // X Direction
            match Direction::vector(&self).0 {
                Direction::Pos => self.tail.0.x = self.tail.0.x + 1,
                Direction::Neg => self.tail.0.x = self.tail.0.x - 1,
                _ => ()
            }

            //Y Direction
            match Direction::vector(&self).1 {
                Direction::Pos => self.tail.0.y = self.tail.0.y + 1,
                Direction::Neg => self.tail.0.y = self.tail.0.y - 1,
                _ => ()
            }

            println!("({} {})",self.tail.0.x, self.tail.0.y);
            self.visited.insert((self.tail.0.x, self.tail.0.y));
            Self::adjust(self);
        }
        
        
    }
}

#[derive(Debug)]
enum Direction {
    Pos = 1,
    Zero = 0,
    Neg = -1
}

impl Direction {
    fn vector(r: &Rope) -> (Direction, Direction) {

        let x_dir;
        let y_dir;
        
       let Rope { head: Head(Position{x:h_x,y:h_y}),tail: Tail(Position{x:t_x,y:t_y}),.. } = r; 
        
        match (h_x - t_x).signum() as i32{
            1 => x_dir = Direction::Pos,
            0 => x_dir = Direction::Zero,
            _ => x_dir = Direction::Neg
        }

        match (h_y - t_y).signum() as i32{
            1 => y_dir = Direction::Pos,
            0 => y_dir = Direction::Zero,
            _ => y_dir = Direction::Neg
        }
        
        (x_dir, y_dir)
    }
}

fn main() {
    let input = include_str!("input.prod");

    let mut rope = Rope::new();
    rope.visited.insert((0,0));

    input.lines().for_each(|x| {
        let query = x.parse::<Query>().unwrap();

        rope.event(query);
    });
    
    println!("{:?}", rope.visited.len());
}
