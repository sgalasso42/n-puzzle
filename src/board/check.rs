use crate::board::create::{snail_generate};

fn get_inversion_count(board: Vec<i8>, s: i8) -> i32 {
	let mut inv_count:i32 = 0;
	for i in 0..(s * s - 1) {
		for j in(i + 1)..(s * s) {
			if board[i as usize] != s * s && board[j as usize] != s * s && board[i as usize] > board[j as usize] {
				inv_count += 1;
			}
		}
	}
  return inv_count;
}

fn get_hole_x_from_bottom(board: Vec<i8>, s: i8) -> i8 {
	for i in 0..(s * s) {
		if board[i as usize] >= s * s {
			return s - i / s;
		}
	}
	return 0;
}

pub fn is_solvable(size: i8, board: Vec<i8>) -> bool {
	/*
	* Checks if the board is solvable
	* @return the true if it is
	*/
	let solved_board: Vec<i8> = snail_generate(size);
	let inv_board: i32 = get_inversion_count(board.clone(), size);
	let inv_solved: i32 = get_inversion_count(solved_board.clone(), size);
	
	if size % 2 != 0 {
		return inv_board % 2 == inv_solved % 2
	} else {
		let mut x_pos: i8 = get_hole_x_from_bottom(board.clone(), size);
		if size % 4 == 2 {
			x_pos -= 1;
		}
		if x_pos % 2 == 1 {
			return inv_board % 2 != inv_solved % 2;
		} else {
			return inv_board % 2 == inv_solved % 2;
		}
	}
}