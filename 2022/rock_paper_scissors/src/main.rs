use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let wins = HashMap::from([("A","Y"),("B","Z"),("C","X")]);
    let draws = HashMap::from([("A","X"),("B","Y"),("C","Z")]);
    let loss = HashMap::from([("A","Z"),("B","X"),("C","Y")]);
    let scores = HashMap::from([("X",1),("Y",2),("Z",3)]);

    let mut score = 0;
    
    // Part 1
    let _input = input.split('\n').filter(|x| *x != "").inspect(|x| {
        let curr = x.split(' ').collect::<Vec<&str>>();
        if curr[1] == *wins.get(curr[0]).unwrap(){
            score += 6 + scores[curr[1]];
        } else if curr[1] == *draws.get(curr[0]).unwrap() {
            score += 3 + scores[curr[1]];
        } else {
            score += scores[curr[1]];
        }
    }).collect::<Vec<&str>>();

    println!("Total Score is in part 1 is: {}", score);

    // Part 2
    let mut score = 0;

    let _input = input.split('\n').filter(|x| *x != "").inspect(|x| {
        let curr = x.split(' ').collect::<Vec<&str>>();
        if curr[1] == "X" {
            //Loss
           score += scores[loss[curr[0]]];
        } else if curr[1] == "Y" {
            // Draw
            score += 3 + scores[draws[curr[0]]];
        } else {
            // Win
            score += 6 + scores[wins[curr[0]]];
        }
    }).collect::<Vec<&str>>();

    println!("Total Score is in part 2 is: {}", score);
}
