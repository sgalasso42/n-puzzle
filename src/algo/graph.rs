use crate::board::utils::*;
use crate::algo::heuristics::{Heuristic, heuristic};

#[derive(Debug, Clone, PartialEq)]
pub enum Dir {
	N, E, S, W, None
}

impl Dir {
	pub fn value(&self) -> (i8, i8) {
		match *self {
			Dir::N => (0, -1),
			Dir::E => (1, 0),
			Dir::S => (0, 1),
			Dir::W => (-1, 0),
			Dir::None => (0, 0)
		}
	}
}

pub fn new_position(position: (i8, i8), dir: (i8, i8)) -> (i8, i8) {
	return (position.0 + dir.0, position.1 + dir.1);
}

pub fn apply_action(size: i8, state: &Vec<i8>, current_pos: (i8, i8), new_pos: (i8, i8)) -> Result<Vec<i8>, ()> {
	let mut new_state: Vec<i8> = state.clone();
	if (0..(size)).contains(&(new_pos.0)) && (0..(size)).contains(&(new_pos.1)) {
		let index_a: i8 = fdtos(current_pos.0, current_pos.1, size);
		let index_b: i8 = fdtos(new_pos.0, new_pos.1, size);
		new_state.swap(index_a as usize, index_b as usize);
		return Ok(new_state);
	}
	return Err(());
}

fn get_neighbors(size: i8, state: &Vec<i8>) -> Vec<(Dir, Vec<i8>)> {
	let sd_pos: i8 = slot_pos(size, &state);
	let dd_pos: (i8, i8) = fstod(sd_pos, size);
	let positions = [Dir::N, Dir::E, Dir::S, Dir::W];
	let mut neighbors: Vec<(Dir, Vec<i8>)> = Vec::new();
	for pos in positions.iter() {
		match apply_action(size, &state, dd_pos, new_position(dd_pos, pos.value())) {
			Ok(new_state) => neighbors.push((pos.clone(), new_state)),
			Err(()) => {}
		}
	}
	return neighbors;
}

fn graph_search(size: i8, path: &mut Vec<(Dir, Vec<i8>)>, target: &Vec<i8>, cost: u32, bound: u32, explored_nodes: &mut u32) -> (bool, u32) {
	*explored_nodes += 1;
	let node = path.last().expect("Error: The path is empty");
	let new_cost: u32 = cost + heuristic(Heuristic::Manhattan, size, &node.1, target);
	if new_cost > bound {
		return (false, new_cost);
	}
	else if node.1 == *target {
		return (true, new_cost);
	}
	let mut min: u32 = std::u32::MAX;
	for neighbour in get_neighbors(size, &node.1).iter() {
		if !path.contains(neighbour) {
			path.push(neighbour.clone());
			match graph_search(size, path, target, cost + 1, bound, explored_nodes) {
				(res, _) if res => return (true, min),
				(_, val) if val < min => min = val,
				(_, _) => {}
			}
			path.pop();
		}
	}
	return (false, min);
}

pub fn resolve_puzzle(size: i8, path: &mut Vec<(Dir, Vec<i8>)>, target: &Vec<i8>, explored_nodes: &mut u32) {
	let node = path.last().expect("Error: The path has not been initialized");
	let mut bound = heuristic(Heuristic::Manhattan, size, &node.1, target);
	eprintln!("bound: {}", bound);
	loop {
		match graph_search(size, path, target, 0, bound, explored_nodes) {
			res if res.0 => break,
			res => bound = res.1
		}
		eprintln!("new bound: {}", bound);
	}
}