#[path = "create.rs"] mod create;

fn get_inversion_count(board:Vec<i32>, size:i32) -> i32 {
	let mut inv_count:i32 = 0;
	for i in 0..(size * size - 1) {
		for j in(i + 1)..(size * size) {
			if board[i as usize] != size * size && board[j as usize] != size * size && board[i as usize] > board[j as usize] {
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

fn get_hole_positions(board:Vec<i32>, size:i32) -> (i32, i32) {
	for i in 0..(size * size) {
		if board[i as usize] >= size * size || board[i as usize] <= 0 {
			return (i % size, i / size)
		}
	}
	return (0, 0);
}

pub fn is_solvable(size: i32, board:Vec<i32>) -> bool {
	let solved_board: Vec<i32> = create::create_solved_board(size);
	let mut inv_board: i32 = get_inversion_count(board.clone(), size);
	let mut inv_solved: i32 = get_inversion_count(solved_board.clone(), size);
	println!("inv_board: {:?}", inv_board);
	println!("inv_solved: {:?}", inv_solved);

	// if size % 2 == 0 {
	// 	// let (x_board/, y_board): (i32, i32) = get_hole_positions(board.clone(), size);
	// 	// let (x_solved, y_solved): (i32, i32) = get_hole_positions(solved_board.clone(), size);
	// 	// println!("x_board: {:?} y_board: {:?}", x_board, y_board);
	// 	// println!("x_solved: {:?} y_solved: {:?}", x_solved, y_solved);
	// 	inv_board += get_hole_x(board.clone(), size);
	// 	inv_solved += get_hole_x(solved_board.clone(), size);
	// 	println!("inv_board: {:?}", inv_board);
	// 	println!("inv_solved: {:?}", inv_solved);
	// }
		// let (x_init, y_init) = self.initial_state.free_coordinates();
		// let (x_goal, y_goal) = self.goal_state.free_coordinates();
		// nb_inversion_start +=
		// 	self.initial_state.index_out_of_xy(x_init, y_init) / self.size;
		// nb_inversion_goal +=
		// self.goal_state.index_out_of_xy(x_goal, y_goal) / self.size;
		
	// return inv_board % 2 == inv_solved % 2
	
	if size % 2 == 1 {
		if inv_board % 2 == inv_solved % 2 {
			return true;
		}
	} else {
		let mut x_pos: i32 = get_hole_x_from_bottom(board.clone(), size);
		if size % 4 == 2 {
			x_pos -= 1;
		}
		println!("x_pos: {:?}", x_pos);

		if x_pos % 2 == 1 {
			if inv_board % 2 != inv_solved % 2 {
				return true;
			}
		} else {
			if inv_board % 2 == inv_solved % 2 {
				return true;
			}
		}
	}
	
	// if size % 2 != 0 {
	// 	return inv_board % 2 == inv_solved % 2;
	// } else {
	// 	let mut x_pos: i32 = get_hole_x_from_bottom(board.clone(), size);
	// 	if size % 4 == 2 {
	// 		x_pos -= 1;
	// 	}
	// 	if x_pos % 2 != 0 {
	// 		return inv_board % 2 != inv_solved % 2;
	// 	} else {
	// 		return inv_board % 2 == inv_solved % 2;
	// 	}
	// }
	return false;
}