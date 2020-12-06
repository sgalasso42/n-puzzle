use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Heuristic {
   Manhattan,
   Euclidian,
   Hamming,
   LinearConflict
}

impl fmt::Display for Heuristic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn heuristic(heuristic: &Heuristic, size: i8, state: &Vec<i8>, target: &Vec<i8>) -> u32 {
   match heuristic {
	  Heuristic::Manhattan => {
		return manhattan(size, state, target);
	  },
	  Heuristic::Euclidian => {
		return euclidian(size, state, target);
	  },
	  Heuristic::Hamming => {
		return hamming_distance(size, state, target);
	  }
	  Heuristic::LinearConflict => {
		return linear_conflict(size, state, target);
	  }
   }
}

fn manhattan(size: i8, state: &Vec<i8>, target: &Vec<i8>) -> u32 {
    let mut dist: u32 = 0;
    for i in 0..(state.len()){
        if state[i] != size * size {
            let destination_index: usize = target.iter().position(|&x| x == state[i]).unwrap_or(0);
            let x = (i as i32 % size as i32 - destination_index as i32 % size as i32).abs();
            let y = (i as i32 / size as i32 - destination_index as i32 / size as i32).abs();
            dist += (x + y) as u32;
        }
    }
    return dist;
}

fn euclidian(size: i8, state: &Vec<i8>, target: &Vec<i8>) -> u32 {
    let mut dist: f32 = 0.0;
    for i in 0..(state.len()) {
        if state[i] != size * size {
            let destination_index: usize = target.iter().position(|&x| x == state[i]).unwrap_or(0);
            let x = (i as i32 % size as i32 - destination_index as i32 % size as i32).pow(2);
            let y = (i as i32 / size as i32 - destination_index as i32 / size as i32).pow(2);
            dist += (x as f32 + y as f32).sqrt();
        }
    }
    return dist as u32;
}

fn hamming_distance(size: i8, state: &Vec<i8>, target: &Vec<i8>) -> u32 {
    let mut misplaced: u32 = 0;
    for i in 0..(size * size) {
        if state[i as usize] != size * size {
           if state[i as usize] != target[i as usize] {
                misplaced += 1;
            }
        }
    }
    return misplaced;
}

fn is_conflicting(tile_a: i8, target_a: i8, tile_b: i8, target_b: i8) -> bool {
    return (tile_b > tile_a && target_b < target_a) || (tile_a > tile_b && target_a < target_b);
}

fn find_conflicting_tiles_nb_for(size: i8, tile: usize, target: usize, line: &Vec<i8>, target_line: &Vec<i8>) -> i8 {
    let mut conflicts_nb: i8 = 0;
    if line[tile as usize] != size * size {
        for index in 0..size as usize {
            let target_b = target_line.iter().position(|&x| x == line[index as usize]);
            if target_b.is_some() {
                if line[index] != size * size && is_conflicting(tile as i8, target as i8, index as i8, target_b.unwrap() as i8) {
                    conflicts_nb += 1;
                }
            }
        }
    }
    return conflicts_nb;
}

fn line_extra_moves(size: i8, line: &Vec<i8>, target_line: Vec<i8>) -> i8 {
    let mut total_conflicting_tiles: i8 = 0;
    let mut conflicts_table: Vec<i8> = Vec::new();
    for tile_idx in 0..size as usize {
        let target = target_line.iter().position(|&x| x == line[tile_idx]);
        if target.is_some() {
            conflicts_table.push(find_conflicting_tiles_nb_for(size, tile_idx, target.unwrap(), &line, &target_line));
        } else {
            conflicts_table.push(0);
        }
    }
    while conflicts_table.iter().sum::<i8>() > 0 {
        let most_conflicting_tile: usize = conflicts_table.iter().position(|&x| x == *conflicts_table.iter().max().unwrap_or(&0)).unwrap_or(0);
        conflicts_table[most_conflicting_tile] = 0;
        for tile in 0..size as usize {
            let target = target_line.iter().position(|&x| x == line[tile]);
            let target_b = target_line.iter().position(|&x| x == line[most_conflicting_tile]);
            if target.is_some() && target_b.is_some() {
                if is_conflicting(tile as i8, target.unwrap_or(tile) as i8, most_conflicting_tile as i8, target_b.unwrap_or(most_conflicting_tile) as i8) {
                    conflicts_table[tile] -= 1;
                }
            }
        }
        total_conflicting_tiles += 1;
    }
    return total_conflicting_tiles;
}

fn linear_conflict(size: i8, state: &Vec<i8>, target: &Vec<i8>) -> u32 {
    let mut extra_moves: i8 = 0;
    for row_index in 0..size {
        let row: Vec<i8> = Vec::from(&state[((size * row_index) as usize)..((size * row_index + size) as usize)]);
        let target_row: Vec<i8> = Vec::from(&target[((size * row_index) as usize)..((size * row_index + size) as usize)]);
        extra_moves += line_extra_moves(size, &row, target_row);
    }
    for col_index in 0..size {
        let col: Vec<i8> = state.iter().cloned().enumerate().filter(|&(i, _)| i % size as usize == col_index as usize).map(|(_, e)| e).collect();
        let target_col: Vec<i8> = target.iter().cloned().enumerate().filter(|&(i, _)| i % size as usize == col_index as usize).map(|(_, e)| e).collect();
        extra_moves += line_extra_moves(size, &col, target_col);
    }
    return manhattan(size, state, target) + (2 * extra_moves) as u32;
}