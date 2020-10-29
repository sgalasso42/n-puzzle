use std::env;
use std::time::{Instant};

extern crate npuzzle;
use npuzzle::board::create::{snail_generate};
use npuzzle::board::check::{is_solvable};
use npuzzle::board::utils::{slot_pos};
use npuzzle::algo::graph::{resolve_puzzle, Dir};
use npuzzle::args::parser::{handle_args};

fn main() {
	let args: Vec<String> = env::args().collect();
	let (size, state) = handle_args(&args);

	println!("size: {:?}", size);
	println!("state: {:?}", state);
	
	let slot_pos = slot_pos(size, &state);
	println!("slot_pos: {}", slot_pos);

	let solvable: bool = is_solvable(size, state.clone());
	println!("solvable: {:?}", solvable);

	if !solvable {
		panic!("error: puzzle not solvable")
	}

	let target = snail_generate(size);
	println!("target: {:?}", target);

	let mut path: Vec<(Dir, Vec<i32>)> = Vec::new();
	path.push((Dir::None, state));

	eprintln!("-------");
	
	let start_time = Instant::now();
	resolve_puzzle(size, &mut path, &target);
	
	eprintln!("-------");
	
	let mut sequence = Vec::new();
	for node in path.iter() {
		if node.0 != Dir::None { sequence.push(node.0) }
	}

	println!("solution: {:?}", sequence);
	println!("moves number: {:?}", path.len() - 1);
	eprintln!("duration: {:?}s ({:?})\n", start_time.elapsed().as_secs(), start_time.elapsed());
	
	eprintln!("-------");
}