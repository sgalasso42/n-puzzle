use rand::seq::SliceRandom;

// give snail value for a given index
fn snail(w: i32, h: i32, x: i32, y: i32) -> i32 {
	return if y != 0 {
		w + snail(h - 1, w, y - 1, w - x - 1)
	} else {
		x + 1
	};
}

// generate a snake organized array of a given size
pub fn snail_generate(size: i32) -> Vec<i32> {
	let mut target: Vec<i32> = Vec::new();
	for y in 0..size {
		for x in 0..size {
			target.push(snail(size, size, x, y));
		}
	}
	return target;
}

// generate a game board array of a given size
pub fn board_generate(s: i32, iterations: i32) -> (i32, Vec<i32>) {
	/*
	* Creates a game board
	* @return the generated game board
	*/
	// let s: i32 = size.parse().unwrap_or(0);
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