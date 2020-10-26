use std::env;
use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Copy, Clone)]
enum Dir {
	N, E, S, W
}

#[derive(Debug)]
struct Node {
	state: Vec<i32>,
}

impl Node {
	fn new(values: Vec<i32>) -> Node {
		Node {
			state: values,
		}
	}
}

// convert position from single dimension to double dimensions array
fn fstod(index: usize, width: usize) -> (usize, usize) {
	return (index % width, index / width);
}

// convert position from double dimensions to single dimension array
fn fdtos(x: usize, y: usize, width: usize) -> usize {
	return y * width + x;
}

// cmp of two array
fn is_same(size: i32, state: &Vec<i32>, target: &Vec<i32>) -> bool {
	let matching = state.iter().zip(target).filter(|&(state, target)| state == target).count();
	return matching == (size * size) as usize;
}

// define new position
fn new_position(dir: &Dir) -> (i32, i32) {
	return match dir {
		Dir::N => (0, -1),
		Dir::E => (1, 0),
		Dir::S => (0, 1),
		Dir::W => (-1, 0)
	}
}

// moving the slot to get the new state
fn apply_action(size: i32, state: &mut Vec<i32>, current_pos: (usize, usize), new_pos: (i32, i32)) -> Result<Vec<i32>, ()> {
	let index_a = fdtos(current_pos.0, current_pos.1, size as usize);
	let index_b = fdtos(new_pos.0 as usize, new_pos.1 as usize, size as usize);
	if (0..(size * size)).contains(&(index_a as i32)) && (0..(size * size)).contains(&(index_b as i32)) {
		state.swap(index_a, index_b);
		return Ok(state.clone());
	}
	return Err(());
}

// find puzzle next possibilities
fn get_neighbors(size: i32, mut state: Vec<i32>) -> Vec<Node> {
	let sd_pos = state.iter().position(|&x| x == 0).unwrap_or(0); // single dimension position
	let dd_pos = fstod(sd_pos, size as usize); // double dimension position
	let positions = [Dir::N, Dir::E, Dir::S, Dir::W];
	let mut neighbors: Vec<Node> = Vec::new();
	for pos in positions.iter() {
		let new_state = apply_action(size, &mut state, dd_pos, new_position(pos));
		if new_state.is_ok() {
			neighbors.push(Node::new(new_state.unwrap()));
		}
	}
	return neighbors;
}

// recursive graph search
fn graph_search(size: i32, node: Node, target: &Vec<i32>, sequence: &mut Vec<Dir>) -> Vec<Dir> {
	if is_same(size, &node.state, target) {
		return sequence.to_vec();
	}
	let neighbors = get_neighbors(size, node.state);
	for neighbour in neighbors {
		graph_search(size, neighbour, &target, sequence);
	}
	sequence.pop();
	return sequence.to_vec();
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
	let (size, values) = load_file(&args);

	let root: Node = Node::new(values);
	println!("root: {:?}", root.state);

	let target = snail_generate(size);
	println!("target: {:?}", target);

	let sequence: Vec<Dir> = graph_search(size, root, &target, &mut Vec::new());
	println!("sequence: {:?}", sequence);
}