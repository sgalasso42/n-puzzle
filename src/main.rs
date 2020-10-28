use std::env;

extern crate npuzzle;
use npuzzle::board::create::{snail_generate};
use npuzzle::board::check::{is_solvable};
use npuzzle::board::utils::{slot_pos};
use npuzzle::algo::graph::{graph_search, Dir};
use npuzzle::args::parser::{handle_args};

fn main() {
	let args: Vec<String> = env::args().collect();
	let (size, state) = handle_args(&args);

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