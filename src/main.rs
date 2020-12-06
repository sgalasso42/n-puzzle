use std::time::{Instant};

extern crate npuzzle;
use npuzzle::board::create::{snail_generate};
use npuzzle::board::check::{is_solvable};
use npuzzle::args::handle::{handle_args};
use npuzzle::args::parser::{Config};
use npuzzle::algo::graph::*;
use npuzzle::board::utils::*;

pub fn get_full_array(state: Vec<i8>, size: i8, sequence: &Vec<Dir>) -> Vec<Vec<i8>> {
	let mut state_updated: Vec<i8> = state.clone();
	let mut board_array: Vec<Vec<i8>> = Vec::new();
	board_array.push(state.clone());
	for pos in sequence.iter() {
		let sd_pos: i8 = slot_pos(size, &state_updated);
		let dd_pos: (i8, i8) = fstod(sd_pos, size);
		let new_state: Vec<i8> = apply_action(size, &state_updated, dd_pos, new_position(dd_pos, pos.value())).unwrap();
		board_array.push(new_state.clone());
		state_updated = new_state.clone();
	}
	return board_array;
}

fn main() {
	let config = Config::new();
	// println!("config: {:?}", config);
	let (size, state) = handle_args(&config);

	println!("size: {:?}", size);
	println!("state: {:?}", state);
	println!("iterations: {:?}", config.iterations);
	
	let slot_pos: i8 = slot_pos(size, &state);
	println!("slot_pos: {}", slot_pos);

	let solvable: bool = is_solvable(size, state.clone());
	println!("solvable: {:?}", solvable);

	if !solvable {
		panic!("error: puzzle not solvable")
	}

	let target = snail_generate(size);
	println!("target: {:?}", target);

	let mut path: Vec<(Dir, Vec<i8>)> = Vec::new();
	path.push((Dir::None, state.clone()));

	eprintln!("-------");
	
	let start_time = Instant::now();
	let mut explored_nodes: u32 = 0;
	resolve_puzzle(size, &mut path, &target, &mut explored_nodes);
	// use crate::npuzzle::algo::heuristics::{linear_conflict};
	// linear_conflict(size, &(path.last().unwrap().1), &target);
	
	eprintln!("-------");
	
	let mut sequence = Vec::new();
	for node in path.iter() {
		if node.0 != Dir::None { sequence.push(node.0.clone()) }
	}

	println!("solution: {:?}", sequence);
	println!("moves number: {:?}", path.len() - 1);
	println!("explored nodes: {}", explored_nodes);
	// println!("possible nb of solvable states: {:?}", factorial((size * size) as u64) / 2);
	println!("duration: {:?}s ({:?})", start_time.elapsed().as_secs(), start_time.elapsed());
	
	eprintln!("-------");

	// if config.visual == true {
	// 	let board_array = get_full_array(state.clone(), size, &sequence);
	// 	start_visual(&board_array, size, start_time.elapsed().as_secs().to_string(), config.heuristic);
	// }
}

// [!] Attention au parsing :
// size < std::i8::MAX
// explored_nodes < std::u32::MAX
// cell content < std::io::MAX

// TODO :
// Utilisation des bonnes structures de data (pas que i32 et f64)
// Multithreading
// Sécu
// virer les eprint