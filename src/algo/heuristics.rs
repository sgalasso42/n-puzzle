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

fn is_conflicting(tile: i32, target: i32, index: i32, line: &Vec<i32>) -> bool {
    if tile < target {
        return index > tile && index <= target;
    }
    return index >= target && index < tile;
}

fn find_conflicting_tiles_nb_for(size: i32, tile: i32, target: i32, line: &Vec<i32>) -> i32 {
    let mut conflicts_nb: i32 = 0;
    if line[tile as usize] != size * size {
        for index in 0..size {
            // eprintln!("line[index]: {}", line[index as usize]);
            // eprintln!("is_conflicting: {} && {}", line[index as usize] != size * size, is_conflicting(tile, target, index, line));
            let mut is_conflicting: bool = line[index as usize] != size * size && is_conflicting(tile, target, index, line);
            // eprintln!("tl: {} | tg: {} | id: {} | conflict: {}", tile, target, index, is_conflicting);
            if is_conflicting {
                conflicts_nb += 1;
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
        conflicts_table.push(find_conflicting_tiles_nb_for(size, tile_idx, target.unwrap_or(tile_idx as usize) as i32, &line));
    }
    eprintln!("tbl: {:?}", conflicts_table);
    while conflicts_table.iter().sum::<i32>() > 0 {
        let most_conflicting_tile: usize = conflicts_table.iter().position(|&x| x == *conflicts_table.iter().max().unwrap_or(&0)).unwrap_or(0);
        eprintln!("mct: {}", most_conflicting_tile);
        conflicts_table[most_conflicting_tile] = 0;
        for tile in 0..size {
            let target = target_line.iter().position(|&x| x == line[tile as usize]);
            if target.is_some() {
                let unwrap = target.unwrap();
                // check for slot tile here before use is_conflicting()
                let is_conflicting = is_conflicting(tile, unwrap as i32, most_conflicting_tile as i32, line);
                if is_conflicting {
                    conflicts_table[tile as usize] -= 1;
                }
                eprintln!("tl: {} | tg: {} | mct: {} | conflict: {}", tile, unwrap, most_conflicting_tile, is_conflicting);
            } else {
                eprintln!("tl: {} | tg: NO | mct: {}", tile, most_conflicting_tile);
            }
        }
        total_conflicting_tiles += 1;
    }
    eprintln!("line total: {}", total_conflicting_tiles);
    return total_conflicting_tiles;
}

// https://cse.sc.edu/~mgv/csce580sp15/gradPres/HanssonMayerYung1992.pdf (page 8)
pub fn linear_conflict(size: i32, state: &Vec<i32>, target: &Vec<i32>) -> i32 {
    let mut extra_moves: i32 = 0;
    eprintln!("# ROWS ##################");
    for row_index in 0..size {
        eprintln!("------------------");
        let row: Vec<i32> = Vec::from(&state[((size * row_index) as usize)..((size * row_index + size) as usize)]);
        eprintln!("row : {:?}", row);
        let target_row: Vec<i32> = Vec::from(&target[((size * row_index) as usize)..((size * row_index + size) as usize)]);
        eprintln!("trow: {:?}", target_row);
        extra_moves += line_extra_moves(size, &row, target_row);
    }
    eprintln!("# COLS ##################");
    for col_index in 0..size {
        eprintln!("------------------");
        let col: Vec<i32> = state.iter().cloned().enumerate().filter(|&(i, _)| i % size as usize == col_index as usize).map(|(_, e)| e).collect();
        eprintln!("col : {:?}", col);
        let target_col: Vec<i32> = target.iter().cloned().enumerate().filter(|&(i, _)| i % size as usize == col_index as usize).map(|(_, e)| e).collect();
        eprintln!("tcol: {:?}", target_col);
        extra_moves += line_extra_moves(size, &col, target_col);
    }
    eprintln!("manhattan       : {}", manhattan(size, state, target));
    eprintln!("linear conflicts: {}", manhattan(size, state, target) + (2 * extra_moves));
    return manhattan(size, state, target) + (2 * extra_moves);
}