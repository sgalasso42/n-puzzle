use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

pub fn load_file(args: &[String]) -> (i32, Vec<i32>) {
	if args.len() != 2 {
		panic!("error: bad args number")
	}
	let file = File::open(&args[1]).expect("error: file not found");
	let lines: Vec<_> = BufReader::new(file).lines().collect();
	let mut size = 0;
	let mut values: Vec<i32> = Vec::new();
	for (i, line) in lines.into_iter().enumerate() {
		for num in line.expect("error: bad format").split(" ") {
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