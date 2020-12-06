// factorial product
pub fn factorial(num: u64) -> u64 {
	match num {
			0 | 1 => 1,
			_ => factorial(num - 1) * num,
	}
}

// convert position from single dimension to double dimensions array
pub fn fstod(index: i8, width: i8) -> (i8, i8) {
	return (index % width, index / width);
}

// convert position from double dimensions to single dimension array
pub fn fdtos(x: i8, y: i8, width: i8) -> i8 {
	return y * width + x;
}

// position du slot
pub fn slot_pos(size: i8, state: &Vec<i8>) -> i8 {
	return state.iter().position(|&x| x == size * size).unwrap_or(0) as i8;
}