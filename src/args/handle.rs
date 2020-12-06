use crate::args::parser::Config;
use crate::board::create::board_generate;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn load_file(file: &String) -> (i8, Vec<i8>) {
    let file = File::open(file).expect("error: file not found");
    let lines: Vec<_> = BufReader::new(file).lines().collect();
    let mut size: i8 = 0;
    let mut values: Vec<i8> = Vec::new();
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

pub fn handle_args(config: &Config) -> (i8, Vec<i8>) {
    if config.file.is_empty() {
        return board_generate(config.size as i8, config.iterations as i8, config.solvable);
    } else {
        return load_file(&config.file);
    }
}
