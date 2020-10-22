#[cfg(test)]
mod tests {
    extern crate npuzzle;
    use npuzzle::board::check::*;

    #[test]
    fn solvable_1() {
        let vec = vec![3, 2, 6, 1, 4, 0, 8, 7, 5];
        let is_solvable: bool = is_solvable(3, vec);
        assert_eq!(is_solvable, true);
    }

    #[test]
    fn solvable_2() {
        let vec = vec![1, 8, 2, 0, 4, 3, 7, 6, 5];
        let is_solvable: bool = is_solvable(3, vec);
        assert_eq!(is_solvable, false);
    }

    #[test]
    fn solvable_3() {
        let vec = vec![3, 9, 1, 15, 14, 11, 4, 6, 13, 0, 10, 12, 2, 7, 8, 5];
        let is_solvable: bool = is_solvable(4, vec);
        assert_eq!(is_solvable, false);
    }
}
