use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SearchType {
    Normal,
    Greedy,
    Uniform,
}

impl fmt::Display for SearchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// pub fn search_type(search_type: &SearchType, size: i8, state: &Vec<i8>, target: &Vec<i8>) -> u32 {
//    match search_type {
// 	  SearchType::Normal => {
// 		return manhattan(size, state, target);
// 	  },
// 	  SearchType::Greedy => {
// 		return euclidian(size, state, target);
// 	  },
// 	  SearchType::Uniform => {
// 		return hamming_distance(size, state, target);
// 	  }
//    }
// }
