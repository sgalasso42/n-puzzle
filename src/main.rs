use std::time::{Instant};

extern crate npuzzle;
use npuzzle::board::create::{snail_generate};
use npuzzle::board::check::{is_solvable};
use npuzzle::board::utils::{slot_pos, factorial};
use npuzzle::algo::graph::{resolve_puzzle, Dir};
use npuzzle::args::handle::{handle_args};
use npuzzle::args::parser::{Config};

fn main() {
	let config = Config::new();
	println!("Config, {:?}", config);

	let (size, state) = handle_args(&config);

	println!("size: {:?}", size);
	println!("state: {:?}", state);
	
	let slot_pos = slot_pos(size, &state);
	println!("slot_pos: {}", slot_pos);

	let solvable: bool = is_solvable(size, state.clone());
	println!("solvable: {:?}", solvable);

	// if !solvable {
	// 	panic!("error: puzzle not solvable")
	// }

	let target = snail_generate(size);
	println!("target: {:?}", target);

	let mut path: Vec<(Dir, Vec<i32>)> = Vec::new();
	path.push((Dir::None, state));

	eprintln!("-------");
	
	let start_time = Instant::now();
	resolve_puzzle(size, &mut path, &target);
	// use crate::npuzzle::algo::heuristics::{linear_conflict};
	// linear_conflict(size, &(path.last().unwrap().1), &target);
	
	eprintln!("-------");
	
	let mut sequence = Vec::new();
	for node in path.iter() {
		if node.0 != Dir::None { sequence.push(node.0) }
	}

	println!("solution: {:?}", sequence);
	println!("moves number: {:?}", path.len() - 1);
	println!("possible nb of solvable states: {:?}", factorial((size * size) as u64) / 2);
	println!("duration: {:?}s ({:?})", start_time.elapsed().as_secs(), start_time.elapsed());
	
	eprintln!("-------");
}