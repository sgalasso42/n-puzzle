use std::thread;
use crate::board::utils::*;
use crate::algo::heuristics::*;
use std::collections::VecDeque;

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

pub fn get_full_array(state: Vec<i32>, size: i32, sequence: &Vec<Dir>) -> Vec<Vec<i32>> {
	let mut state_updated: Vec<i32> = state.clone();
	let mut board_array: Vec<Vec<i32>> = Vec::new();
	board_array.push(state.clone());
	for pos in sequence.iter() {
		let sd_pos: usize = slot_pos(size, &state_updated);
		let dd_pos: (i32, i32) = fstod(sd_pos as i32, size);
		let new_state = apply_action(size, &state_updated, dd_pos, new_position(dd_pos, movement_value(pos))).unwrap();
		board_array.push(new_state.clone());
		state_updated = new_state.clone();
	}
	return board_array;
}

// Define new position
fn new_position(position: (i32, i32), movement_value: (i32, i32)) -> (i32, i32) {
	// eprintln!("[position]: {:?}", position);
	// eprintln!("[movement_value]: {:?}", movement_value);
	return (position.0 as i32 + movement_value.0, position.1 as i32 + movement_value.1);
}

// Moving the slot to get the new state
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

// Find puzzle next possibilities
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

// IDA* / Recursive graph search
fn graph_search(size: i32, path: &mut Vec<(Dir, Vec<i32>)>, target: &Vec<i32>, cost: i32, bound: i32, explored_nodes: &mut i32) -> (bool, i32) {
	*explored_nodes += 1;
	let node = path.last().unwrap();
	let new_cost = cost + linear_conflict(size, &node.1, target);
	
	// eprintln!("[search node]: {:?}", node);
	if new_cost > bound { return (false, new_cost) }
	else if node.1 == *target { return (true, new_cost) }
	// eprintln!("[neighbors]: {:?}", neighbors);
	let mut min: i32 = std::i32::MAX;
	for neighbour in get_neighbors(size, &node.1).iter() {
		if !path.contains(neighbour) {
			path.push(neighbour.clone());
			let res = graph_search(size, path, target, cost + 1, bound, explored_nodes);
			if res.0 { return (true, min) }
			else if res.1 < min { min = res.1 }
			path.pop();
		}
	}
	return (false, min);
}

// BFS / Find threads start position
fn get_start_pos(size: i32, path: &mut Vec<(Dir, Vec<i32>)>, target: &Vec<i32>, cost: i32, bound: i32, explored_nodes: &mut i32) -> VecDeque<Vec<i32>> {
	let root = path.last().unwrap();
	let mut closed: VecDeque<Vec<i32>> = VecDeque::new();
	let mut opened: VecDeque<Vec<i32>> = VecDeque::new();
	opened.push_front(root.1);
	while !opened.is_empty() {
		let node: Vec<i32> = opened.pop_back().unwrap();
		// if node.0 == *target { return (true, 0) } // not necessary to return 0
		for neighbour in get_neighbors(size, &node).iter() { // false not necessary
			if !closed.contains(&neighbour.1) {
				closed.push_back(neighbour.1.clone());
				opened.push_front(neighbour.1.clone());
			}
		}
		if opened.len() >= 8 { break ; }
	}
	return opened;
}

// Loop
pub fn resolve_puzzle(size: i32, main_path: &mut Vec<(Dir, Vec<i32>)>, target: &Vec<i32>, explored_nodes: &mut i32) {
	let node = main_path.last().unwrap();
	let mut bound = linear_conflict(size, &node.1, target);

	eprintln!("bound: {}", bound);
	loop {
		let mut results: Vec<(bool, i32)> = Vec::new();
		let handles = Vec::new();
		for neighbour in get_start_pos(size, &mut main_path.clone(), target, 0, bound, explored_nodes) {
			handles.push(thread::spawn(|| {
				let mut path: Vec<(Dir, Vec<i32>)> = main_path.clone();
				path.push((Dir::None, neighbour.clone())); // changer none par valeur réelle
				results.push(graph_search(size, &mut path, target, 0, bound, explored_nodes));
			}));
		}
		for handle in handles.iter() {
			handle.join().unwrap();
		}
		if results.iter().any(|res| res.0 == true) {
			break;
		}
		bound = results.iter().min_by_key(|res| res.1);
		eprintln!("new bound: {}", bound);
	}
}

// retourner des Option au lieu de tuples (permet aussi de ne pas retourner de score en cas de solution)
// BFS gerer cas ou on trouve la solution
// Concatener les valeurs du BFS avec celles de l'IDA*