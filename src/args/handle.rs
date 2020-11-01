use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;
use crate::board::create::{board_generate};
use crate::args::parser::{Config};

fn load_file(file: &String) -> (i32, Vec<i32>) {
	// if args.len() != 2 {
	// 	panic!("error: bad args number")
	// }
	let file = File::open(file).expect("error: file not found");
	let lines: Vec<_> = BufReader::new(file).lines().collect();
	let mut size = 0;
	let mut values: Vec<i32> = Vec::new();
	for (i, line) in lines.into_iter().enumerate() {
		for num in line.expect("error: bad format").split_whitespace() {
			if i == 0 {
				size = num.parse().expect("error: bad character")
			} else {
				let mut val = num.parse().expect("error: bad character");
				if val == 0 {
					val = size * size
				}
				values.push(val)
			}
		}
	}
	// if size * size != value.len => error
	// handle comments #
	return (size, values);
}

pub fn handle_args(config: &Config) -> (i32, Vec<i32>) {
	if (config.file != "") {
		return load_file(&config.file)
	} else {
		return board_generate(config.size, config.iterations)
	}
}