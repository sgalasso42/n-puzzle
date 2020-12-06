pub fn factorial(num: u64) -> u64 {
	match num {
			0 | 1 => 1,
			_ => factorial(num - 1) * num,
	}
}

pub fn fstod(index: i8, width: i8) -> (i8, i8) {
	return (index % width, index / width);
}

pub fn fdtos(x: i8, y: i8, width: i8) -> i8 {
	return y * width + x;
}

pub fn slot_pos(size: i8, state: &Vec<i8>) -> i8 {
	return state.iter().position(|&x| x == size * size).expect("Error: No slot found in state") as i8;
}