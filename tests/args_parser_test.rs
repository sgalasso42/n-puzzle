#[cfg(test)]
mod tests {
    extern crate npuzzle;
    use npuzzle::args::parser::*;

    #[test]
    fn test_no_args() {
        assert_eq!(
            Config::new_from(["exename"].iter()).unwrap(),
            Config { file: "".to_string(), size: 3, iterations: 1000, heuristic: "test".to_string(), solvable: false, unsolvable: false, visual: false }
        );
    }

    fn test_complete_name() {
        assert_eq!(
            Config::new_from(["exename", "--file", "Hello"].iter()).unwrap(),
            Config { file: "Hello".to_string(), size: 3, iterations: 1000, heuristic: "test".to_string(), solvable: false, unsolvable: false, visual: false }
        );
    }

    fn test_short_name() {
        assert_eq!(
            Config::new_from(["exename", "-f", "Hello"].iter()).unwrap(),
            Config { file: "Hello".to_string(), size: 3, iterations: 1000, heuristic: "test".to_string(), solvable: false, unsolvable: false, visual: false }
        );
    }
}
