use std::env;
use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

#[path = "board/create.rs"] mod create;
#[path = "board/check.rs"] mod check;

#[derive(Debug, Copy, Clone)]
enum Dir {
	N, E, S, W
}

// convert position from single dimension to double dimensions array
fn fstod(index: i32, width: i32) -> (i32, i32) {
	return (index % width, index / width);
}

// convert position from double dimensions to single dimension array
fn fdtos(x: i32, y: i32, width: i32) -> i32 {
	return y * width + x;
}

// cmp of two array
fn is_same(size: i32, state: &Vec<i32>, target: &Vec<i32>) -> bool {
	let matching = state.iter().zip(target).filter(|&(state, target)| state == target).count();
	return matching == (size * size) as usize;
}

// position du slot
fn slot_pos(size: i32, state: &Vec<i32>) -> usize {
	return state.iter().position(|&x| x == size * size).unwrap_or(0);
}

fn movement_value(dir: &Dir) -> (i32, i32) {
	return match dir {
		Dir::N => (0, -1),
		Dir::E => (1, 0),
		Dir::S => (0, 1),
		Dir::W => (-1, 0)
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
fn get_neighbors(size: i32, state: &Vec<i32>, parent: usize) -> Vec<(Dir, Vec<i32>)> {
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
			let unwrapped_new_state = new_state.unwrap();
			if slot_pos(size, &unwrapped_new_state) != parent {
				// eprintln!("[state]: {:?}", new_state);
				neighbors.push((*pos, unwrapped_new_state));
			}
		}
	}
	return neighbors;
}

// recursive graph search
fn graph_search(size: i32, state: Vec<i32>, target: &Vec<i32>, sequence: &mut Vec<Dir>, best_sequence: &mut Vec<Dir>, parent: usize) {
	eprintln!("********************");
	eprintln!("[search state]: {:?}", state);
	if is_same(size, &state, target) {
		*best_sequence = sequence.clone();
		eprintln!("[!] [solution]: {:?}", best_sequence);
	} else {
		let neighbors: Vec<(Dir, Vec<i32>)> = get_neighbors(size, &state, parent);
		eprintln!("[neighbors]: {:?}", neighbors);
		for neighbour in neighbors.iter() {
			sequence.push(neighbour.0);
			graph_search(size, neighbour.1.clone(), &target, sequence, best_sequence, slot_pos(size, &state)); // check si possible eviter clone
		}
	}
	sequence.pop();
}

// give snail value for a given index
fn snail(w: i32, h: i32, x: i32, y: i32) -> i32 {
	return if y != 0 {
		w + snail(h - 1, w, y - 1, w - x - 1)
	} else {
		x + 1
	};
}

// generate an snake organized array of a given size
fn snail_generate(size: i32) -> Vec<i32> {
	let mut target: Vec<i32> = Vec::new();
	for y in 0..size {
		for x in 0..size {
			target.push(snail(size, size, x, y));
		}
	}
	return target;
}

fn load_file(args: &[String]) -> (i32, Vec<i32>) {
	if args.len() != 2 {
		panic!("error: bad args number")
	}
	let file = File::open(&args[1]).expect("error: file not found");
	let lines: Vec<_> = BufReader::new(file).lines().collect();
	let mut size = 0;
	let mut values: Vec<i32> = Vec::new();
	for (i, line) in lines.into_iter().enumerate() {
		for num in line.expect("error: bad format").split(" ") {
			if i == 0 {
				size = num.parse().expect("error: bad character")
			} else {
				let mut val = num.parse().expect("error: bad character");
				if val == 0 {
					val = size * size
				}
				values.push(val)
			}
		}
	}
	// if size * size != value.len => error
	// handle comments #
	return (size, values);
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let (size, values) = 
	if args.len() > 1 && args[1] == "--create" {
		create::create_game_board(&args[2], 1000)
	} else {
		load_file(&args)
	};

	println!("values: {:?}", values);
	println!("size: {:?}", size);

	let is_solvable: bool = check::is_solvable(size, values.clone());
	println!("is_solvable: {:?}", is_solvable);

	if !is_solvable {
		panic!("error: puzzle not solvable")
	}

	let root: Node = Node::new(values);
	println!("root: {:?}", root.state);

	let target = snail_generate(size);
	println!("target: {:?}", target);

	let slot_pos = slot_pos(size, &state);
	println!("slot_pos: {}", slot_pos);

	let mut best_sequence: Vec<Dir> = Vec::new();

	graph_search(size, state, &target, &mut Vec::new(), &mut best_sequence, slot_pos);

	println!("sequence: {:?}", best_sequence);
}