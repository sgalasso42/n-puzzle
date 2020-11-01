pub fn manhattan(size: i32, state: &Vec<i32>, target: &Vec<i32>) -> i32 {
    let mut dist: i32 = 0;
    for i in 0..(state.len()) {
        if (state[i] != size * size) {
            let destination_index: usize = target.iter().position(|&x| x == state[i]).unwrap_or(0);
            let x = (i as i32 % size - destination_index as i32 % size).abs();
            let y = (i as i32 / size - destination_index as i32 / size).abs();
            // eprintln!("x: {:?}", x);
	        // eprintln!("y: {:?}", y);
            dist += (x + y);
        }
    }
	// eprintln!("dist: {:?}", dist);
    return dist;
}

pub fn euclidian(size: i32, state: &Vec<i32>, target: &Vec<i32>) -> i32 {
    let mut dist: f32 = 0.0;
    for i in 0..(state.len()) {
        if (state[i] != size * size) {
            let destination_index: usize = target.iter().position(|&x| x == state[i]).unwrap_or(0);
            let x = (i as i32 % size - destination_index as i32 % size).pow(2);
            let y = (i as i32 / size - destination_index as i32 / size).pow(2);
            // let x: f32 = (i as f32 % size as f32 - destination_index as f32 % size as f32).powf(2.0);
            // let y: f32 = (i as f32 / size as f32 - destination_index as f32 / size as f32).powf(2.0);
	        // eprintln!("x: {:?}", x);
	        // eprintln!("y: {:?}", y);
            
            dist += (x as f32 + y as f32).sqrt();
        }
    }
	// eprintln!("dist: {:?}", dist);
    return dist as i32;
}