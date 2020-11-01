use crate::algo::graph::{Dir};

pub fn manhattan(size: i32, state: &Vec<i32>, target: &Vec<i32>) -> i32 {
    let mut dist: i32 = 0;
    // eprintln!("state {:?}", state);
    // eprintln!("target {:?}", target);

    for i in 0..(state.len()) {
        if (state[i] != size * size) {
            let destination_index: usize = target.iter().position(|&x| x == state[i]).unwrap_or(0);
            let x = (i as i32 % size - destination_index as i32 % size).abs();
            let y = (i as i32 / size - destination_index as i32 / size).abs();
            // eprintln!("index {:?}", i);
            // eprintln!("destination_index {:?}", destination_index);
            // eprintln!("x {:?}", x);
            // eprintln!("y {:?}", y);
            dist += (x + y);
            // eprintln!("dist: {:?}", dist);
        }
    }    
    return dist;
}