use crate::algo::graph::{apply_action, new_position, Dir};

// Factorial product
pub fn factorial(num: u64) -> u64 {
	match num {
		0 | 1 => 1,
		_ => match factorial(num - 1).checked_mul(num) {
            Some(x) => x,
            None => return 0,
        },
	}
}

// Convert position from single dimension to double dimensions array
pub fn fstod(index: i8, width: i8) -> (i8, i8) {
	return (index % width, index / width);
}

// Convert position from double dimensions to single dimension array
pub fn fdtos(x: i8, y: i8, width: i8) -> i8 {
	return y * width + x;
}

// Slot position
pub fn slot_pos(size: i8, state: &Vec<i8>) -> i8 {
	return state.iter().position(|&x| x == size * size).unwrap_or(0) as i8;
}

// Get all the states as a Vector from the sequence
pub fn get_all_states(state: Vec<i8>, size: i8, sequence: &Vec<Dir>) -> Vec<Vec<i8>> {
	let mut state_updated: Vec<i8> = state.clone();
	let mut board_array: Vec<Vec<i8>> = Vec::new();
	board_array.push(state.clone());
	for pos in sequence.iter() {
		let sd_pos: i8 = slot_pos(size, &state_updated);
		let dd_pos: (i8, i8) = fstod(sd_pos, size);
		let new_state: Vec<i8> = apply_action(size, &state_updated, dd_pos, new_position(dd_pos, pos.value())).unwrap();
		board_array.push(new_state.clone());
		state_updated = new_state.clone();
	}
	return board_array;
}