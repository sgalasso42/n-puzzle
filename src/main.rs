use std::time::{Instant};

extern crate npuzzle;
use npuzzle::board::create::{snail_generate};
use npuzzle::board::check::{is_solvable};
use npuzzle::board::utils::{slot_pos, factorial};
use npuzzle::algo::graph::{resolve_puzzle, Dir};
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
	path.push((Dir::None, state));

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
		let size: i32 = 3;
    	let board_array = [vec![3, 2, 6, 1, 4, 9, 8, 7, 5], vec![3, 2, 6, 1, 9, 4, 8, 7, 5], vec![3, 2, 6, 9, 1, 4, 8, 7, 5], vec![3, 2, 9, 6, 1, 4, 8, 7, 5]];
		graphics(&board_array, size);
	}
}