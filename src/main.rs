use std::time::{Instant};

extern crate npuzzle;
use npuzzle::board::create::{snail_generate};
use npuzzle::board::check::{is_solvable};
use npuzzle::board::utils::{slot_pos, factorial};
use npuzzle::algo::graph::{resolve_puzzle, Dir, get_full_array};
use npuzzle::args::handle::{handle_args};
use npuzzle::args::parser::{Config};
use npuzzle::graphics::init::{graphics};

fn main() {
	let config = Config::new();
	let (size, state) = handle_args(&config);

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
	path.push((Dir::None, state.clone()));

	eprintln!("-------");
	
	let start_time = Instant::now();
	let mut explored_nodes: i32 = 0;
	resolve_puzzle(size, &mut path, &target, &mut explored_nodes);
	// use crate::npuzzle::algo::heuristics::{linear_conflict};
	// linear_conflict(size, &(path.last().unwrap().1), &target);
	
	eprintln!("-------");
	
	let mut sequence = Vec::new();
	for node in path.iter() {
		if node.0 != Dir::None { sequence.push(node.0) }
	}

	println!("solution: {:?}", sequence);
	println!("moves number: {:?}", path.len() - 1);
	println!("explored nodes: {}", explored_nodes);
	println!("possible nb of solvable states: {:?}", factorial((size * size) as u64) / 2);
	println!("duration: {:?}s ({:?})", start_time.elapsed().as_secs(), start_time.elapsed());
	
	eprintln!("-------");

	if config.visual == true {
		let board_array = get_full_array(state.clone(), size, &sequence);
		graphics(&board_array, size, start_time.elapsed().as_secs().to_string());
	}
}