use crate::board::utils::*;

pub fn manhattan(size: i32, state: &Vec<i32>, target: &Vec<i32>) -> i32 {
    let mut dist: i32 = 0;
    for i in 0..(state.len()) {
        if state[i] != size * size {
            let destination_index: usize = target.iter().position(|&x| x == state[i]).unwrap_or(0);
            let x = (i as i32 % size - destination_index as i32 % size).abs();
            let y = (i as i32 / size - destination_index as i32 / size).abs();
            // eprintln!("x: {:?}", x);
	        // eprintln!("y: {:?}", y);
            dist += x + y;
        }
    }
	// eprintln!("dist: {:?}", dist);
    return dist;
}

pub fn euclidian(size: i32, state: &Vec<i32>, target: &Vec<i32>) -> i32 {
    let mut dist: f32 = 0.0;
    for i in 0..(state.len()) {
        if state[i] != size * size {
            let destination_index: usize = target.iter().position(|&x| x == state[i]).unwrap_or(0);
            let x = (i as i32 % size - destination_index as i32 % size).pow(2);
            let y = (i as i32 / size - destination_index as i32 / size).pow(2);
            // let x: f32 = (i as f32 % size as f32 - destination_index as f32 % size as f32).powf(2.0);
            // let y: f32 = (i as f32 / size as f32 - destination_index as f32 / size as f32).powf(2.0);
	        // eprintln!("x: {:?}", x);
	        // eprintln!("y: {:?}", y);
            
            dist += (x as f32 + y as f32).sqrt();
        }
    }
	// eprintln!("dist: {:?}", dist);
    return dist as i32;
}

fn is_conflicting(tile_a: i32, target_a: i32, tile_b: i32, target_b: i32) -> bool {
    return (tile_b > tile_a && target_b < target_a) || (tile_a > tile_b && target_a < target_b);
}

fn find_conflicting_tiles_nb_for(size: i32, tile: i32, target: i32, line: &Vec<i32>, target_line: &Vec<i32>) -> i32 {
    let mut conflicts_nb: i32 = 0;
    if line[tile as usize] != size * size {
        for index in 0..size {
            // eprintln!("line[index]: {}", line[index as usize]);
            // eprintln!("is_conflicting: {} && {}", line[index as usize] != size * size, is_conflicting(tile, target, index, line));
            let target_b = target_line.iter().position(|&x| x == line[index as usize]);
            if target_b.is_some() {
                // eprintln!("tl: {} | tg: {} | id: {} | conflict: {}", tile, target, index, is_conflicting);
                if line[index as usize] != size * size && is_conflicting(tile, target, index, target_b.unwrap() as i32) {
                    conflicts_nb += 1;
                }
            }
        }
    }
    return conflicts_nb;
}

fn line_extra_moves(size: i32, line: &Vec<i32>, target_line: Vec<i32>) -> i32 {
    let mut total_conflicting_tiles: i32 = 0;
    let mut conflicts_table: Vec<i32> = Vec::new();
    for tile_idx in 0..size {
        let target = target_line.iter().position(|&x| x == line[tile_idx as usize]);
        if target.is_some() {
            conflicts_table.push(find_conflicting_tiles_nb_for(size, tile_idx, target.unwrap() as i32, &line, &target_line));
        } else {
            conflicts_table.push(0);
        }
    }
    while conflicts_table.iter().sum::<i32>() > 0 {
        // eprintln!("tbl: {:?}", conflicts_table);
        let most_conflicting_tile: usize = conflicts_table.iter().position(|&x| x == *conflicts_table.iter().max().unwrap_or(&0)).unwrap_or(0);
        // eprintln!("mct: {}", most_conflicting_tile);
        conflicts_table[most_conflicting_tile] = 0;
        for tile in 0..size {
            let target = target_line.iter().position(|&x| x == line[tile as usize]);
            let target_b = target_line.iter().position(|&x| x == line[most_conflicting_tile as usize]);
            if target.is_some() && target_b.is_some() {
                if is_conflicting(tile, target.unwrap_or(tile as usize) as i32, most_conflicting_tile as i32, target_b.unwrap_or(most_conflicting_tile) as i32) {
                    conflicts_table[tile as usize] -= 1;
                }
            }
        }
        total_conflicting_tiles += 1;
    }
    // eprintln!("line total: {}", total_conflicting_tiles);
    return total_conflicting_tiles;
}

// https://cse.sc.edu/~mgv/csce580sp15/gradPres/HanssonMayerYung1992.pdf (page 8)
pub fn linear_conflict(size: i32, state: &Vec<i32>, target: &Vec<i32>) -> i32 {
    let mut extra_moves: i32 = 0;
    // eprintln!("# ROWS ##################");
    for row_index in 0..size {
        // eprintln!("------------------");
        let row: Vec<i32> = Vec::from(&state[((size * row_index) as usize)..((size * row_index + size) as usize)]);
        // eprintln!("row : {:?}", row);
        let target_row: Vec<i32> = Vec::from(&target[((size * row_index) as usize)..((size * row_index + size) as usize)]);
        // eprintln!("trow: {:?}", target_row);
        extra_moves += line_extra_moves(size, &row, target_row);
    }
    // eprintln!("# COLS ##################");
    for col_index in 0..size {
        // eprintln!("------------------");
        let col: Vec<i32> = state.iter().cloned().enumerate().filter(|&(i, _)| i % size as usize == col_index as usize).map(|(_, e)| e).collect();
        // eprintln!("col : {:?}", col);
        let target_col: Vec<i32> = target.iter().cloned().enumerate().filter(|&(i, _)| i % size as usize == col_index as usize).map(|(_, e)| e).collect();
        // eprintln!("tcol: {:?}", target_col);
        extra_moves += line_extra_moves(size, &col, target_col);
    }
    // eprintln!("manhattan       : {}", manhattan(size, state, target));
    // eprintln!("linear conflicts: {}", manhattan(size, state, target) + (2 * extra_moves));
    return manhattan(size, state, target) + (2 * extra_moves);
}