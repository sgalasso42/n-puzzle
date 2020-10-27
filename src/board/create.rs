use rand::seq::SliceRandom;

// give snail value for a given index
fn snail(w: i32, h: i32, x: i32, y: i32) -> i32 {
	return if y != 0 {
		w + snail(h - 1, w, y - 1, w - x - 1)
	} else {
		x + 1
	};
}

// generate an snake organized array of a given size
pub fn snail_generate(size: i32) -> Vec<i32> {
	let mut target: Vec<i32> = Vec::new();
	for y in 0..size {
		for x in 0..size {
			target.push(snail(size, size, x, y));
		}
	}
	return target;
}

// pub fn create_solved_board(s: i32) -> Vec<i32> {
// 	/*
// 	* SHOULD use snail_generate
// 	* Creates a solved board
// 	* @return the generated solved board
// 	*/
// 	let mut board: Vec<i32> = Vec::new();
// 	let mut i: i32 = 0;

// 	while i < s * s {
// 		board.push(-1);
// 		i += 1;
// 	}

// 	let mut cur: i32 = 1;
// 	let mut x: i32 = 0;
// 	let mut ix: i32 = 1;
// 	let mut y: i32 = 0;
// 	let mut iy: i32 = 0;

// 	loop {
// 		board[(x + y * s) as usize] = cur;
// 		if cur == s * s {
// 			break;
// 		}
// 		cur += 1;
// 		if x + ix == s || x + ix < 0 || (ix != 0 && board[(x + ix + y * s) as usize] != -1) {
// 			iy = ix;
// 			ix = 0;
// 		}
// 		else if y + iy == s || y + iy < 0 || (iy != 0 && board[(x + (y + iy) * s) as usize] != -1) {
// 			ix = -iy;
// 			iy = 0;
// 		}
// 		x += ix;
// 		y += iy;
// 		// if cur == s * s {
// 		// 	cur = 0
// 		// }
// 	}
// 	println!("solved_board: {:?}", board);

// 	return board;
// }

pub fn board_generate(size: &String, iterations: i32) -> (i32, Vec<i32>) {
	/*
	* Creates a game board
	* @return the generated game board
	*/
	let s: i32 = size.parse().unwrap_or(0);
	let mut board: Vec<i32> = snail_generate(s);
	
	let mut i: i32 = 0;
	let mut pos: i32;
	let mut choices: Vec<i32> = Vec::new();
	let mut switch_index: &i32;
	
	while i < iterations {
		pos = board.iter().position(|&x| x == s * s).unwrap_or(0) as i32;
		choices.clear();
		
		if pos % s > 0 {
			choices.push(pos - 1);
		}
		if pos % s < s - 1 {
			choices.push(pos + 1);
		}
		if pos / s > 0 && pos - s >= 0 {
			choices.push(pos - s);
		}
		if pos / s < s - 1 {
			choices.push(pos + s);
		}

		switch_index = choices.choose(&mut rand::thread_rng()).unwrap_or(&-1);

		board[pos as usize] = board[*switch_index as usize];
		board[*switch_index as usize] = s * s;
		i += 1
	}

	return (s, board);
}

pub fn create_fake_board(board:Vec<i32>, size:i32) -> Vec<i32> {
	/*
	* Creates a solved board as in the traditional game
	* @return the generated solved board
	*/
	let mut fake_board:Vec<i32> = Vec::new();
	for i in 0..(size * size) {
		fake_board.push(i + 1 as i32);
	}
	println!("fake_board: {:?}", fake_board);

	return board;
}