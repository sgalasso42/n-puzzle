use rand::seq::SliceRandom;

pub fn create_board(size: &String) -> Vec<i32> {
	let s: i32 = size.parse().unwrap_or(0);

	let mut map: Vec<i32> = Vec::new();
	let mut i: i32 = 0;

	while i < s * s {
		map.push(-1);
		i += 1;
	}

	let mut cur: i32 = 1;
	let mut x: i32 = 0;
	let mut ix: i32 = 1;
	let mut y: i32 = 0;
	let mut iy: i32 = 0;

	loop {
		map[(x + y * s) as usize] = cur;
		if cur == 0 {
			break;
		}
		cur += 1;
		if x + ix == s || x + ix < 0 || (ix != 0 && map[(x + ix + y * s) as usize] != -1) {
			iy = ix;
			ix = 0;
		}
		else if y + iy == s || y + iy < 0 || (iy != 0 && map[(x + (y + iy) * s) as usize] != -1) {
			ix = -iy;
			iy = 0;
		}
		x += ix;
		y += iy;
		if cur == s * s {
			cur = 0
		}
	}
	
	i = 0;
	let mut idx: i32;
	let mut poss: Vec<i32> = Vec::new();
	let mut swi: &i32;
	
	while i < 10000 {
		idx = map.iter().position(|&x| x == 0).unwrap_or(0) as i32;
		poss.clear();
		
		if idx % s > 0 {
			poss.push(idx - 1);
		}
		if idx % s < s - 1 {
			poss.push(idx + 1);
		}
		if idx / s > 0 && idx - s >= 0 {
			poss.push(idx - s);
		}
		if idx / s < s - 1 {
			poss.push(idx + s);
		}

		swi = poss.choose(&mut rand::thread_rng()).unwrap_or(&-1);

		map[idx as usize] = map[*swi as usize];
		map[*swi as usize] = 0;
		i += 1
	}
	return map;
}

fn get_inversion_count(board:Vec<i32>, size:i32) -> i32 {
	let mut inv_count:i32 = 0;
	for i in 0..(size * size - 1) {
		for j in(i + 1)..(size * size) {
			if (board[i as usize] != size*size && board[j as usize] != size*size && board[i as usize] > board[j as usize]) {
				inv_count += 1;
			}
		}
	}
    return inv_count;
}

fn get_x_position(board:Vec<i32>, size:i32) -> i32 {
	let mut inv_count:i32 = 0;
	for i in 0..(size * size) {
		if (board[i as usize] == size * size) {
			return (size - (i) / size);
		}
	}
	return (0);
}

pub fn is_solvable(size: i32, board:Vec<i32>) -> bool {
	let inv_nb: i32 = get_inversion_count(board.clone(), size);
	println!("inv_nb: {:?}", inv_nb);
	if (size % 2 != 0) {
		return (inv_nb % 2 == 0);
	} else {
		let x_pos: i32 = get_x_position(board.clone(), size);
		println!("x_pos: {:?}", x_pos);
		if (x_pos % 2 != 0) {
			return (inv_nb % 2 == 0);
		} else {
			return (inv_nb % 2 != 0);
		}

	}
	return false;
}