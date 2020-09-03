use std::env;
use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
enum Dir {
	N, E, S, W
}

struct Node {
	state: Vec<i32>,
	checked: bool
}

impl Node {
	fn new(size: usize, values: Vec<i32>) -> Node {
		Node {
			state: values,
			checked: false
		}
	}
}

/* ## FUNCTIONS ################################################################################ */

fn is_same(size: i32, state: Vec<i32>, target: Vec<i32>) -> bool {
	let matching = state.iter().zip(&target).filter(|&(state, target)| state == target).count();
	return matching == (size * size) as usize;
}

fn apply_action(state: Vec<i32>, dir: Dir) -> Vec<i32> {
	// apply move relative to dir
}

fn get_neighbors() {
	let mut neighbors: Vec<Node> = Vec::new();
	for dir in Dir {
		let result = apply_action(state, dir);
		if Ok(result) && !is_same(result, sequence.last())  {
			neighbors.push(result);
		}
	}
}

fn graph_search(size: i32, node: Node, target: Vec<i32>, sequence: Vec<Dir>) -> Vec<Dir> {
	if is_same(size, node.state, target) {
		return sequence;
	}
	let neighbors = get_neighbors();
	for neighbour in neighbors {
		graph_search(size, node, target, sequence);
	}
	sequence.pop();
	return sequence;
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
	if (args.len() != 2) {
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
	// handle commentary #
	return (size, values);
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let (size, values) = load_file(&args);

	let mut root: Node = Node::new(size as usize, values);
	println!("root: {:?}", root.state);

	let target = snail_generate(size);
	println!("target: {:?}", target);

	let mut sequence: Vec<Dir> = graph_search(size, root, target, Vec::new());
	println!("sequence: {:?}", sequence);
}