// factorial product
pub fn factorial(num: u64) -> u64 {
	match num {
			0 | 1 => 1,
			_ => factorial(num - 1) * num,
	}
}

// convert position from single dimension to double dimensions array
pub fn fstod(index: i32, width: i32) -> (i32, i32) {
	return (index % width, index / width);
}

// convert position from double dimensions to single dimension array
pub fn fdtos(x: i32, y: i32, width: i32) -> i32 {
	return y * width + x;
}

// cmp of two array
pub fn is_same(size: i32, state: &Vec<i32>, target: &Vec<i32>) -> bool {
	let matching = state.iter().zip(target).filter(|&(state, target)| state == target).count();
	return matching == (size * size) as usize;
}

// position du slot
pub fn slot_pos(size: i32, state: &Vec<i32>) -> usize {
	return state.iter().position(|&x| x == size * size).unwrap_or(0);
}