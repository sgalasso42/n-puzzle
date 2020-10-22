#[path = "create.rs"] mod create;

fn get_inversion_count(board:Vec<i32>, size:i32) -> i32 {
	let mut inv_count:i32 = 0;
	for i in 0..(size * size - 1) {
		for j in(i + 1)..(size * size) {
			if board[i as usize] != size * size && board[j as usize] != size * size && board[i as usize] != 0 && board[j as usize] != 0 && board[i as usize] > board[j as usize] {
				inv_count += 1;
			}
		}
	}
    return inv_count;
}

fn get_hole_x_from_bottom(board:Vec<i32>, size:i32) -> i32 {
	for i in 0..(size * size) {
		if board[i as usize] >= size * size || board[i as usize] <= 0 {
			return size - i / size;
		}
	}
	return 0;
}

fn get_hole_x(board:Vec<i32>, size:i32) -> i32 {
	for i in 0..(size * size) {
		if board[i as usize] >= size * size || board[i as usize] <= 0 {
			return i / size;
		}
	}
	return 0;
}

fn get_hole_index(board:Vec<i32>, size:i32) -> i32 {
	for i in 0..(size * size) {
		if board[i as usize] >= size * size || board[i as usize] <= 0 {
			return i;
		}
	}
	return 0;
}

fn get_hole_positions(board:Vec<i32>, size:i32) -> (i32, i32) {
	for i in 0..(size * size) {
		if board[i as usize] >= size * size || board[i as usize] <= 0 {
			return (i % size, i / size)
		}
	}
	return (0, 0);
}

fn rewrite_max_value(board:Vec<i32>, size: i32) -> Vec<i32> {
	/*
	* Rewrites empty tile value to 0 if >= size * size
	* @return the new generated board
	*/
	let mut new_board: Vec<i32> = Vec::new();
	for i in 0..(size * size) {
		if board[i as usize] >= size * size {
			new_board.push(0 as i32);
		} else {
			new_board.push(board[i as usize]);
		}
	}
	return new_board;
}

pub fn is_solvable(size: i32, board:Vec<i32>) -> bool {
	/*
	* Checks if the board is solvable
	* @return the true if it is
	*/
	let solved_board: Vec<i32> = create::create_solved_board(size);
	// Check and rewrite empty tile -> needs to be 0
	let my_board: Vec<i32> = rewrite_max_value(board.clone(), size);
	// Get nb of inversions for board and result
	let inv_board: i32 = get_inversion_count(my_board.clone(), size);
	let inv_solved: i32 = get_inversion_count(solved_board.clone(), size);
	
	if size % 2 != 0 {
		return inv_board % 2 == inv_solved % 2
	} else {
		// Get row from bottom for the empty tile
		let mut x_pos: i32 = get_hole_x_from_bottom(my_board.clone(), size);
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