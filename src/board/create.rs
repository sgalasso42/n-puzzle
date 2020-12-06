use rand::seq::SliceRandom;

// give snail value for a given index
fn snail(w: i8, h: i8, x: i8, y: i8) -> i8 {
	return if y != 0 {
		w + snail(h - 1, w, y - 1, w - x - 1)
	} else {
		x + 1
	};
}

// generate a snake organized array of a given size
pub fn snail_generate(size: i8) -> Vec<i8> {
	let mut target: Vec<i8> = Vec::new();
	for y in 0..size {
		for x in 0..size {
			target.push(snail(size, size, x, y));
		}
	}
	return target;
}

// generate a game board array of a given size
pub fn board_generate(s: i8, iterations: i32, solvable: bool) -> (i8, Vec<i8>) {
	/*
	* Creates a game board
	* @return the generated game board
	*/
	let mut board: Vec<i8> = snail_generate(s as i8);
	
	let mut i: i32 = 0;
	let mut pos: i8;
	let mut choices: Vec<i8> = Vec::new();
	let mut switch_index: &i8;
	
	while i < iterations {
		pos = board.iter().position(|&x| x == s * s).unwrap_or(0) as i8;
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

	if !solvable {
		if board[0] == s * s || board[1] == s * s {
			let tmp: i8 = board[(s * s) as usize - 1];
			board[(s * s) as usize - 1] = board[(s * s) as usize - 2];
			board[(s * s) as usize - 2] = tmp;
		}
		else {
			let tmp: i8 = board[0];
			board[0] = board[1];
			board[1] = tmp;
		}
	}
	return (s as i8, board);
}