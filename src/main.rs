use std::env;
use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
enum Dir {
	N, E, S, W
}

struct Node {
	state: Vec<i32>
}

impl Node {
	fn new(size: usize, values: Vec<i32>) -> Node {
		Node {
			state: values,
		}
	}
}

/* ## FUNCTIONS ################################################################################ */

fn isCompleted(state: Vec<i32>) -> bool {
	return true;
}

fn get_neighbors() {
	
}

fn graph_search(node: Node, sequence: Vec<Dir>) -> Vec<Dir> {
	if isCompleted(node.state) {
		return sequence;
	}
	// let neighbors = get_neighbors();
	// for neighbour in neighbors {
	// 	graph_search(node, sequence);
	// }
	// sequence.pop();
	return sequence;
}

fn snail(w: i32, h: i32, x: i32, y: i32) -> i32 {
	return if y != 0 { w + snail(h - 1, w, y - 1, w - x - 1) } else { x + 1 };
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
	if (args.len() != 2) { panic!("error: bad args number") }
	let file = File::open(&args[1]).expect("error: file not found");
	let lines: Vec<_> = BufReader::new(file).lines().collect();
	let mut size = 0;
	let mut values: Vec<i32> = Vec::new();
	for (i, line) in lines.into_iter().enumerate() {
		for num in line.expect("error: bad format").split(" ") {
			if i == 0 { size = num.parse().expect("error: bad character") }
			else { values.push(num.parse().expect("error: bad character")) }
		}
	}
	// if size * size != value.len => error
	// handle commentary #
	return (size, values);
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let (size, values) = load_file(&args);

	let mut root: Node = Node::new(3, values);
	println!("root: {:?}", root.state);

	let target = snail_generate(size);
	println!("target: {:?}", target);

	let mut sequence: Vec<Dir> = graph_search(root, Vec::new());
	println!("sequence: {:?}", sequence);
}