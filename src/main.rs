use std::env;
use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Copy, Clone)]
enum Dir {
	N, E, S, W
}

struct Node {
	state: Vec<i32>,
	// checked: bool
}

impl Node {
	fn new(values: Vec<i32>) -> Node {
		Node {
			state: values,
			// checked: false
		}
	}
}

/* ## FUNCTIONS ################################################################################ */

fn is_same(size: i32, state: &Vec<i32>, target: &Vec<i32>) -> bool {
	let matching = state.iter().zip(target).filter(|&(state, target)| state == target).count();
	return matching == (size * size) as usize;
}

fn apply_action(_state: &Vec<i32>, _dir: &Dir) -> Vec<i32> {
	// if dir == Dir::N {
	// 	if  {
	// 		state.swap(0, 2);
	// 		return Ok(state);
	// 	}
	// } else if dir == Dir::E {
	// 	if {
	// 		state.swap(0, 2);
	// 		return Ok(state);
	// 	}
	// } else if dir == Dir::S {
	// 	if {
	// 		state.swap(0, 2);
	// 		return Ok(state);
	// 	}
	// } else {
	// 	if {
	// 		state.swap(0, 2);
	// 		return Ok(state);
	// 	}
	// }
	return Vec::new();
}

fn get_neighbors(state: Vec<i32>) -> Vec<Node> {
	let directions = [Dir::N, Dir::E, Dir::S, Dir::W];
	let mut neighbors: Vec<Node> = Vec::new();
	for dir in directions.iter() {
		let result = apply_action(&state, dir);
		// if Ok(result) && !is_same(result, sequence.last())  {
			neighbors.push(Node::new(result));
		// }
	}
	return neighbors;
}

fn graph_search(size: i32, node: Node, target: &Vec<i32>, sequence: &mut Vec<Dir>) -> Vec<Dir> {
	if is_same(size, &node.state, target) {
		return sequence.to_vec();
	}
	let neighbors = get_neighbors(node.state);
	for neighbour in neighbors {
		graph_search(size, neighbour, &target, sequence);
	}
	sequence.pop();
	return sequence.to_vec();
}

fn snail(w: i32, h: i32, x: i32, y: i32) -> i32 {
	return if y != 0 {
		w + snail(h - 1, w, y - 1, w - x - 1)
	} else {
		x + 1
	};
}

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