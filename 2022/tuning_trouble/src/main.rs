use std::collections::HashSet;

fn main() {
    let part_a = String::from(include_str!("input.prod"));
    
    let (mut i,mut j) = (0,0);
    let mut set = HashSet::new();
    
    while j <= part_a.len() {
        // Just change 4 to 14 for Part B
        if j-i >= 4 {
            println!("Characters Processed are: {}",i+4);
            break;
        }

        while set.contains(&part_a.chars().nth(j)) {
            set.remove(&part_a.chars().nth(i));
            i+=1;
        } 
        
        set.insert(part_a.chars().nth(j));
        j+=1;
        
    }
}
