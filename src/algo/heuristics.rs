use std::fmt;
use rulinalg::matrix::{Matrix, BaseMatrixMut, BaseMatrix};

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

pub fn heuristic(heuristic: &Heuristic, size: u16, state: &Vec<u16>, target: &Vec<u16>) -> u32 {
   return match heuristic {
	  Heuristic::Manhattan => manhattan(size, state, target),
	  Heuristic::Euclidian => euclidian(size, state, target),
	  Heuristic::Hamming => hamming_distance(size, state, target),
	  Heuristic::LinearConflict => linear_conflict(size, state, target)
   }
}

fn manhattan(size: u16, state: &Vec<u16>, target: &Vec<u16>) -> u32 {
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

fn euclidian(size: u16, state: &Vec<u16>, target: &Vec<u16>) -> u32 {
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

fn hamming_distance(size: u16, state: &Vec<u16>, target: &Vec<u16>) -> u32 {
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

fn find_conflicts(size: u16, line: &Vec<u16>, target_line: &Vec<u16>) -> Matrix<u16> {
    let mut conflicts_matrix: Matrix<u16> = Matrix::<u16>::zeros(size as usize, size as usize);
    for row in 0..(size as usize) {
        if line[row] != size * size {
            let target_a = target_line.iter().position(|&x| x == line[row]);
            if target_a.is_some() {
                for col in (row + 1)..(size as usize) {
                    if line[col] != size * size {
                        let target_b = target_line.iter().position(|&x| x == line[col]);
                        if target_b.is_some() && target_b < target_a {
                            conflicts_matrix.row_mut(row)[col] = 1;
                            conflicts_matrix.row_mut(col)[row] = 1;
                        }
                    }
                }
            }
        }
    }
    return conflicts_matrix;
}

fn line_extra_moves(size: u16, line: &Vec<u16>, target_line: Vec<u16>) -> u16 {
    let mut total_conflicting_tiles: u16 = 0;
    let mut conflicts_matrix: Matrix<u16> = find_conflicts(size, line, &target_line);
    while conflicts_matrix.sum() > 0 {
        let conflicts_table = conflicts_matrix.sum_cols();
        let most_conflicting_tile: usize = conflicts_table.iter().position(|val| val == conflicts_table.iter().max().unwrap()).unwrap(); // handle unwrap()
        *conflicts_matrix.row_mut(most_conflicting_tile) *= 0;
        *conflicts_matrix.col_mut(most_conflicting_tile) *= 0;
        total_conflicting_tiles += 1;
    }
    return total_conflicting_tiles;
}

fn linear_conflict(size: u16, state: &Vec<u16>, target: &Vec<u16>) -> u32 {
    let mut extra_moves: u16 = 0;
    for row_index in 0..size {
        let row: Vec<u16> = Vec::from(&state[((size * row_index) as usize)..((size * row_index + size) as usize)]);
        let target_row: Vec<u16> = Vec::from(&target[((size * row_index) as usize)..((size * row_index + size) as usize)]);
        extra_moves += line_extra_moves(size, &row, target_row);
    }
    for col_index in 0..size {
        let col: Vec<u16> = state.iter().cloned().enumerate().filter(|&(i, _)| i % size as usize == col_index as usize).map(|(_, e)| e).collect();
        let target_col: Vec<u16> = target.iter().cloned().enumerate().filter(|&(i, _)| i % size as usize == col_index as usize).map(|(_, e)| e).collect();
        extra_moves += line_extra_moves(size, &col, target_col);
    }
    let md = manhattan(size, state, target);
    return md + (2 * extra_moves) as u32;
}