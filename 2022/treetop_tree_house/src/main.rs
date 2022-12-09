use std::cmp::max;

fn main() {
    let input = include_str!("input.prod");
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    
    // Converting the input to a matrix of i32
    input.lines().for_each(|x| {
        matrix.push(x.chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>());
    });

    // Keeps a count of Invisible trees, if there is a tree taller or equal to on
    // (left,right,top,bottom) then it is an invisible tree
    let mut count = 0;
    // We will keep iterating and finding the maximum scenic score for each tree
    let mut max_scenic = i32::MIN;

    for i in 1..matrix.len() {
        for j in 1..matrix.len() {
            
            let (mut left,mut right,mut top,mut bottom) = (0,0,0,0);
            let (mut left_scenic, mut right_scenic, mut top_scenic, mut bottom_scenic) = (0,0,0,0);
            let curr = matrix[i][j];
            // Check left 
            for p in (0..j).rev() {
                if matrix[i][p] < curr { left_scenic+=1; }
                else {
                    left_scenic+=1;
                    left = 1;
                    break;
                }
            }
            // Check Right 
            for p in j+1..matrix.len() {
                if matrix[i][p] < curr {right_scenic+=1;}
                else {
                    right_scenic+=1;
                    right = 1;
                    break;
                }
            }
            // Check Top
            for p in (0..i).rev() {
                if matrix[p][j] < curr { top_scenic += 1;}
                else {
                    top_scenic+=1;
                    top = 1;
                    break;
                }
            }
            // Check Bottom 
            for p in i+1..matrix.len() {
                if matrix[p][j] < curr {bottom_scenic += 1;}
                else {
                    bottom_scenic+=1;
                    bottom = 1;
                    break;
                }
            }
            
            match (left,right,top,bottom) {
                (1,1,1,1) => count+=1,
                _ => count += 0
            }
            // println!("{} {} -> {} {} {} {}",i,j, top_scenic, bottom_scenic, left_scenic, right_scenic);
            // Check if it is maximum scenic score
            max_scenic = max(max_scenic,top_scenic*bottom_scenic*left_scenic*right_scenic);
        }
    }

    println!("Number of visible trees: {}",matrix.len().pow(2) - count);
    println!("Maximum scenic score is: {}",max_scenic);
}
