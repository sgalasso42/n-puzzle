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

fn target(w: i32, h: i32, x: i32, y: i32) -> i32 {
	return if y != 0 { w + target(h - 1, w, y - 1, w - x - 1) } else { x + 1 };
}

// fn isCompleted() {

// }

// fn get_neighbors() {
	
// }

// fn graph_search(node: Node, sequence: Vec<Dir>) -> Vec<Dir> {
// 	if isCompleted(node.state) {
// 		return sequence;	
// 	}
// 	let neighbors = get_neighbors();
// 	for neighbour in neighbors {
// 		graph_search(node, sequence);
// 	}
// 	sequence.pop();
// 	return sequence;
// }

fn load_file(args: &[String]) -> Vec<i32> {
	// to secure
	let file_name = &args[1];
	let file = File::open(file_name).expect("file not found!");
	let reader = BufReader::new(file);
	let mut values: Vec<i32> = Vec::new();
	for line in reader.lines() {
		for num in line.unwrap().split(" ") {
			values.push(num.parse().unwrap());
		}
	}
	return values;
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let values: Vec<i32> = load_file(&args); // parsing security + commentary #

	println!("values: {:?}", values);

	let mut root: Node = Node::new(3, values);

	println!("root: {:?}", root.state);

	// let mut sequence: Vec<Dir> = graph_search(root, Vec::new());

	// println!("actions: {:?}", sequence);

	let size = 3;

	for i in 0..size {
		for j in 0..size {
			print!("{} ", target(size, size, j, i));
		}
		print!("\n");
	}

	// println("target: {:?}", target);
}