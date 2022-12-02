use min_max_heap::MinMaxHeap;

fn main(){
    let content = include_str!("input.txt");
    
    let mut sum = 0;
    let mut pq = MinMaxHeap::new();

    let _elfs: Vec<&str> = content.split('\n').inspect(|x| {
    if let Ok(x) = x.parse::<i32>() {
        sum+=x;
    } else {
        // Add it to priority queue
        if pq.len() < 3 {
            pq.push(sum);
        } else {
            if sum > *pq.peek_min().unwrap() {
                pq.pop_min();
                pq.push(sum);
            } 
        }
        sum=0;
    }
    }).collect();
    
    println!("Top 3 Calories: {:?}", pq);
    println!("Total calories: {}", pq.iter().sum::<i32>());

}
