use std::env;

extern crate npuzzle;
use npuzzle::board::create::*;
use npuzzle::board::check::*;
use npuzzle::board::utils::*;
use npuzzle::algo::graph::*;
use npuzzle::args::parser::*;
// To use only needed function and refato, see args/mod.rs
// use npuzzle::args::*;

fn main() {
	let args: Vec<String> = env::args().collect();

	let (size, state) = 
	if args.len() > 1 && args[1] == "--create" {
		board_generate(&args[2], 1000)
	} else {
		load_file(&args)
	};

	println!("[size]: {:?}", size);
	println!("[state]: {:?}", state);

	let is_solvable: bool = is_solvable(size, state.clone());
	println!("[is_solvable]: {:?}", is_solvable);

	if !is_solvable {
		panic!("error: puzzle not solvable")
	}

	let target = snail_generate(size);
	println!("[target]: {:?}", target);

	let slot_pos = slot_pos(size, &state);
	println!("slot_pos: {}", slot_pos);

	let mut best_sequence: Vec<Dir> = Vec::new();

	graph_search(size, state, &target, &mut Vec::new(), &mut best_sequence, slot_pos);

	println!("sequence: {:?}", best_sequence);
}