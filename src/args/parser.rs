use clap::{App, Arg};
use std::ffi::OsString;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub file: String,
    pub size: i8,
    pub iterations: i32,
    pub heuristic: String,
    pub search_type: String,
    pub solvable: bool,
    pub visual: bool,
}

impl Config {
    pub fn new() -> Self {
        Self::new_from(std::env::args_os().into_iter()).unwrap_or_else(|e| e.exit())
    }

    pub fn new_from<I, T>(args: I) -> Result<Self, clap::Error>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let app = App::new("npuzzle")
            .version("0.1.0")
            .author("Simon Galasso <simon.galasso@hotmail.fr>, Nicolas Vienot <nvienot@gmail.com>")
            .about("Solving taquins!");

        let file_option = Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("file")
            .takes_value(true)
            .help("Path to the file to read from");

        let size_option = Arg::with_name("size")
            .short("n")
            .long("size")
            .value_name("nb")
            .takes_value(true)
            .help("The size of the puzzle");

        let iterations_option = Arg::with_name("iterations")
            .short("i")
            .long("iterations")
            .value_name("nb")
            .takes_value(true)
            .help("The number of iterations");

        let heuristic_option = Arg::with_name("heuristic")
            .short("c")
            .long("heuristic")
            .value_name("name")
            .takes_value(true)
            .help("Heuristic selection, choose from 'manhattan', 'euclidian', 'hamming' and 'conflict'");

        let type_option = Arg::with_name("type")
            .short("t")
            .long("type")
            .value_name("type")
            .takes_value(true)
            .help("Alternative g(x) and f(x), choose from 'greedy' and 'uniform'");

        let solvable_option = Arg::with_name("solvable")
            .short("s")
            .long("solvable")
            .takes_value(false)
            .help("Generates a solvable puzzle");

        let unsolvable_option = Arg::with_name("unsolvable")
            .short("u")
            .long("unsolvable")
            .takes_value(false)
            .help("Generates an unsolvable puzzle");

        let visual_option = Arg::with_name("visual")
            .short("v")
            .long("visual")
            .takes_value(false)
            .help("Make a visualisation of the result");

        let app = app
            .arg(file_option)
            .arg(size_option)
            .arg(iterations_option)
            .arg(heuristic_option)
            .arg(type_option)
            .arg(solvable_option)
            .arg(unsolvable_option)
            .arg(visual_option);

        let matches = app.get_matches_from_safe(args)?;

        let file: String = matches.value_of("file").unwrap_or("").to_string();

        let mut size: i32 = matches.value_of("size").unwrap_or("3").parse().unwrap_or(3);
        if !(2..101).contains(&size) {
            size = 3;
        }
        let mut iterations: i32 = matches
            .value_of("iterations")
            .unwrap_or("100")
            .parse()
            .unwrap_or(100);
        if !(0..1000000).contains(&iterations) {
            iterations = 100;
        }

        let heuristic: String = match matches.value_of("heuristic").unwrap_or("manhattan") {
            "manhattan" => "manhattan".to_string(),
            "euclidian" => "euclidian".to_string(),
            "hamming" => "hamming".to_string(),
            "conflict" => "conflict".to_string(),
            _ => "manhattan".to_string(),
        };

        let search_type: String = match matches.value_of("type").unwrap_or("normal") {
            "greedy" => "greedy".to_string(),
            "uniform" => "uniform".to_string(),
            _ => "normal".to_string(),
        };

        let solvable: bool = matches.is_present("solvable")
            || (!matches.is_present("unsolvable") && !matches.is_present("solvable"));
        let visual: bool = matches.is_present("visual") && size < 15;

        Ok(Config {
            file: file,
            size: size as i8,
            iterations: iterations,
            heuristic: heuristic,
            search_type: search_type,
            solvable: solvable,
            visual: visual,
        })
    }
}
