use crate::board::utils::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Dir {
	N, E, S, W, None
}

fn movement_value(dir: &Dir) -> (i32, i32) {
	return match dir {
		Dir::N => (0, -1),
		Dir::E => (1, 0),
		Dir::S => (0, 1),
		Dir::W => (-1, 0),
		Dir::None => (0, 0)
	}
}

// define new position
fn new_position(position: (i32, i32), movement_value: (i32, i32)) -> (i32, i32) {
	// eprintln!("[position]: {:?}", position);
	// eprintln!("[movement_value]: {:?}", movement_value);
	return (position.0 as i32 + movement_value.0, position.1 as i32 + movement_value.1);
}

// moving the slot to get the new state
fn apply_action(size: i32, state: &Vec<i32>, current_pos: (i32, i32), new_pos: (i32, i32)) -> Result<Vec<i32>, ()> {
	let mut new_state = state.clone();
	// eprintln!("--------------------");
	// eprintln!("[state]: {:?}", state);
	// eprintln!("[current_pos]: {:?}", current_pos);
	// eprintln!("[new_pos]: {:?}", new_pos);
	if (0..(size)).contains(&(new_pos.0)) && (0..(size)).contains(&(new_pos.1)) {
		let index_a = fdtos(current_pos.0, current_pos.1, size);
		let index_b = fdtos(new_pos.0, new_pos.1, size);
		new_state.swap(index_a as usize, index_b as usize);
		return Ok(new_state);
	}
	return Err(());
}

// find puzzle next possibilities
fn get_neighbors(size: i32, state: &Vec<i32>) -> Vec<(Dir, Vec<i32>)> {
	let sd_pos: usize = slot_pos(size, &state); // single dimension position
	// eprintln!("--------------------");
	// eprintln!("[sd_pos]: {:?}", sd_pos);
	let dd_pos: (i32, i32) = fstod(sd_pos as i32, size); // double dimension position
	// eprintln!("[dd_pos]: {:?}", dd_pos);
	let positions = [Dir::N, Dir::E, Dir::S, Dir::W];
	let mut neighbors: Vec<(Dir, Vec<i32>)> = Vec::new();
	for pos in positions.iter() {
		let new_state = apply_action(size, &state, dd_pos, new_position(dd_pos, movement_value(pos)));
		if new_state.is_ok() {
			neighbors.push((*pos, new_state.unwrap()));
		}
	}
	return neighbors;
}

// recursive graph search
fn graph_search(size: i32, path: &mut Vec<(Dir, Vec<i32>)>, target: &Vec<i32>, cost: i32, bound: i32) -> (bool, i32) {
	let node = path.last().unwrap();
	let new_cost = cost/* + h(node)*/;
	// eprintln!("********************");
	// eprintln!("[search node]: {:?}", node);
	if new_cost > bound { return (false, new_cost) }
	if node.1 == *target { return (true, new_cost) }
	// eprintln!("[neighbors]: {:?}", neighbors);
	let mut min: i32 = std::i32::MAX;
	for neighbour in get_neighbors(size, &node.1).iter() {
		if !path.contains(neighbour) {
			path.push(neighbour.clone());
			let res = graph_search(size, path, target, cost + 1, bound);
			if res.0 { return (true, min) }
			if res.1 < min { min = res.1 }
			path.pop();
		}
	}
	return (false, min);
}

// loop
pub fn resolve_puzzle(size: i32, path: &mut Vec<(Dir, Vec<i32>)>, target: &Vec<i32>) {
	let mut bound = /*h(state)*/ 1;
	loop {
		let res = graph_search(size, path, target, 0, bound);
		if res.0 { break }
		bound = res.1;
	}
}