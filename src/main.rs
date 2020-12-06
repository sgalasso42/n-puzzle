use std::time::{Instant};

extern crate npuzzle;
use npuzzle::board::create::{snail_generate};
use npuzzle::board::check::{is_solvable};
use npuzzle::args::handle::{handle_args};
use npuzzle::args::parser::{Config};
use npuzzle::algo::graph::{resolve_puzzle, Dir};
use npuzzle::board::utils::{get_all_states};
use npuzzle::visual::render::{start_visual};

fn main() {
	let config = Config::new();
	let (size, state) = handle_args(&config);

	println!("First state: {:?}", state);
	println!("Size: {}", size);
	println!("Iterations: {}", config.iterations);
	println!("Heuristic: {:?}", config.heuristic);
	println!("Search type: {}", config.search_type);

	let solvable: bool = is_solvable(size, state.clone());
	println!("Solvable: {:?}", solvable);

	if !solvable {
		panic!("error: puzzle not solvable")
	}

	let target = snail_generate(size);
	println!("Target: {:?}", target);

	let mut path: Vec<(Dir, Vec<i8>)> = Vec::new();
	path.push((Dir::None, state.clone()));

	println!("-------");
	
	let start_time = Instant::now();
	let mut explored_nodes: u32 = 0;

	resolve_puzzle(size, &mut path, &target, &mut explored_nodes, &config);
	
	println!("-------");
	
	let mut sequence = Vec::new();
	for node in path.iter() {
		if node.0 != Dir::None { sequence.push(node.0.clone()) }
	}

	println!("Solution: {:?}", sequence);
	println!("Noves number: {:?}", path.len() - 1);
	println!("Explored nodes: {}", explored_nodes);
	// println!("possible nb of solvable states: {:?}", factorial((size * size) as u64) / 2);
	println!("Duration: {:?}s ({:?})", start_time.elapsed().as_secs(), start_time.elapsed());
	
	eprintln!("-------");

	if config.visual == true {
		let board_array = get_all_states(state.clone(), size, &sequence);
		start_visual(board_array, size, start_time.elapsed().as_secs().to_string(), config.heuristic.to_string());
	}
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