use std::collections::HashMap;

#[derive(Debug,Clone)]
struct Dir {
    name: String,
    content: HashMap<String,Item>
}

#[derive(Debug,Clone)]
struct File {
    size: u64,
}

#[derive(Debug,Clone)]
enum Item {
    Dir(Dir),
    File(File)
}

impl Dir {
    
    fn new(name: String) -> Dir {
        Dir {
            name,
            content: HashMap::new()
        }
    }

    fn find_parent(&self, d: &Dir) -> Option<&Dir> {
        
        if self.content.len() == 0 { return None; }

        if self.content.contains_key(&d.name) {
            return Some(self);
        } else {
            return None;
        }

    }
}

fn main(){
    let part_a = include_str!("input.test");
    
    let mut iter = part_a.lines().skip(1);

    let mut root = Dir {
        name: "/".to_string(),
        content: HashMap::new()
    };

    let curr_dir = &mut root.clone();
    
    for mut i in iter.clone() {
        if i.contains("$") {
            // It is a command
            match i.split(" ").nth(1).unwrap() {
                "cd" => {
                    match i.split(" ").last().unwrap() {
                        ".." => curr_dir = root.find_parent(&curr_dir).unwrap(),
                        "/" => curr_dir = &mut root,
                        _ => {
                               if let Item::Dir(c) = &curr_dir.content[i.split(" ").last().unwrap()] {
                                   curr_dir = c;
                               }
                        }
                    }
                },
                _ => {
                    i = iter.next().unwrap();
                    while i.contains("cd") == false && i.contains("ls") == false  {
                        if i.contains("dir") {
                            let name = i.split(" ").last().unwrap();
                            let val = curr_dir.content.entry(name.to_string()).or_insert(Item::Dir(Dir::new(name.to_string())));
                        }
                    }
                }
            }
        } 
    }
}

