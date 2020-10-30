use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;
use crate::board::create::{board_generate};
use clap::{Arg, App};

fn load_file(args: &[String]) -> (i32, Vec<i32>) {
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

pub fn handle_args(args: &Vec<String>) -> (i32, Vec<i32>) {
	let (size, state) = if args.len() > 1 && args[1] == "--create" {
		board_generate(&args[2], 1000)
	} else {
		load_file(&args)
	};

	return (size, state);
}

pub fn test_args() {
	let matches = App::new("npuzzle")
        .version("0.1.0")
        .author("Simon Galasso <simon.galasso@hotmail.fr>, Nicolas Vienot <nvienot@gmail.com>")
        .about("Solving taquins!")
        .arg(Arg::with_name("file")
			.short("f")
			.long("file")
			.value_name("file")
			.takes_value(true)
			.help("Path to the file to read from"))
        .arg(Arg::with_name("size")
			.short("n")
			.long("size")
			.value_name("nb")
			.takes_value(true)
			.help("The size of the puzzle"))
		.arg(Arg::with_name("iterations")
			.short("i")
			.long("iterations")
			.value_name("nb")
			.takes_value(true)
			.help("The number of iterations"))
		.arg(Arg::with_name("heuristic")
			.short("c")
			.long("heuristic")
			.value_name("name")
			.takes_value(true)
			.help("The heuristic you want to choose from"))
		.arg(Arg::with_name("solvable")
			.short("s")
			.long("solvable")
			.takes_value(false)
			.help("Generates a solvable puzzle"))
		.arg(Arg::with_name("unsolvable")
			.short("u")
			.long("unsolvable")
			.takes_value(false)
			.help("Generates an unsolvable puzzle"))
		.arg(Arg::with_name("visual")
			.short("w")
			.long("visual")
			.takes_value(false)
			.help("Make a visualisation of the result"))
        .get_matches();

    // let myfile = matches.value_of("file").unwrap_or("input.txt");
    // println!("The file passed is: {}", myfile);

    // let num_str = matches.value_of("num");
    // match num_str {
    //     None => println!("No idea what your favorite number is."),
    //     Some(s) => {
    //         match s.parse::<i32>() {
    //             Ok(n) => println!("Your favorite number must be {}.", n + 5),
    //             Err(_) => println!("That's not a number! {}", s),
    //         }
    //     }
    // }
}