use std::env;
use std::fs;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;


struct Node {
	state: Vec<Vec<i32>>
}

impl Node {
	fn new(size: usize, values: Vec<i32>) -> Node {
		Node {
			state: (0..size).map(|_y| (0..size).map(|_x| values[(_y * size) + _x]).collect()).collect(),
		}
	}
}

// fn parse_input(contents: String) -> Vec<i64> {
// 	let mut input_line = String::new();
// 	contents.read_line(&mut input_line).unwrap();
// 	let size = input_line[0]; // to secure

// 	let values = Vec::new();
// 	for i in 0..size as usize {
// 			let mut input_line = String::new();
// 			contents.read_line(&mut input_line).unwrap();
// 			let inputs = input_line.split(" ").collect::<Vec<_>>();
// 			values.push(inputs[i]);
// 	}
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
	let mut values: Vec<i32> = load_file(&args); // parsing security + commentary #

	println!("{:?}", values);

	let mut start_node: Node = Node::new(3, values);

	println!("{:?}", start_node.state);
}