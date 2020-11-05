use std::cmp;
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

fn is_conflicting(row: &Vec<i32>, current: usize, target: usize, tile: usize) -> bool {
    if current < target {
        // eprintln!("c: {} | tg: {} | tl: {} | res: {}", current, target, tile, tile > current && tile <= target && row[tile] != -1);
        return tile > current && tile <= target && row[tile] != -1;
    }
    // eprintln!("c: {} | tg: {} | tl: {} | res: {}", current, target, tile, tile >= target && tile < current && row[tile] != -1);
    return tile >= target && tile < current && row[tile] != -1;
}

fn find_conflicting_tiles_nb_for(size: i32, tile_index: i32, target: usize, row: &Vec<i32>) -> i32 {
    let mut conflicts_nb: i32 = 0;
    for x in 0..size {
        eprintln!("tl: {} | tg: {} | id: {} | conflict: {}", tile_index, target, x, is_conflicting(row, tile_index as usize, target, x as usize));
        if is_conflicting(row, tile_index as usize, target, x as usize) {
            conflicts_nb += 1;
        }
    }
    return conflicts_nb;
}

fn line_extra_moves(size: i32, row_index: i32, state: &Vec<i32>, target: &Vec<i32>) -> i32 {
    let mut row: Vec<i32> = Vec::from(&state[((size * row_index) as usize)..((size * row_index + size) as usize)]);
    eprintln!("------------------");
    eprintln!("row: {:?}", row);
    let mut total_conflicting_tiles: i32 = 0;
    let mut conflicts_table: Vec<i32> = Vec::new();
    for tile_index in 0..size {
        let target: usize = fstod(target.iter().position(|&x| x == state[fdtos(tile_index, row_index, size) as usize]).unwrap() as i32, size).0 as usize;
        conflicts_table.push(find_conflicting_tiles_nb_for(size, tile_index, target, &row));
    }
    eprintln!("tbl: {:?}", conflicts_table);
    while (conflicts_table.iter().sum::<i32>() > 0) {
        let most_conflicting_tile: usize = conflicts_table.iter().position(|&x| x == *conflicts_table.iter().max().unwrap_or(&0)).unwrap_or(0);
        eprintln!("mct: {}", most_conflicting_tile);
        conflicts_table[most_conflicting_tile] = 0;
        let target: usize = fstod(target.iter().position(|&x| x == state[fdtos(most_conflicting_tile as i32, row_index, size) as usize]).unwrap() as i32, size).0 as usize;
        for tile in 0..find_conflicting_tiles_nb_for(size, most_conflicting_tile as i32, target, &row) {
            conflicts_table[tile as usize] -= 1;
        }
        total_conflicting_tiles += 1;
    }
    eprintln!("row total: {}", total_conflicting_tiles);
    return total_conflicting_tiles;
}

pub fn linear_conflict(size: i32, state: &Vec<i32>, target: &Vec<i32>) -> i32 {
    let mut extra_moves: i32 = 0;
    for row_index in 0..size {
        extra_moves += line_extra_moves(size, row_index, state, target);
    }
    // for col_index in 0..size {
    //     extra_moves += line_extra_moves(size, row_index, state, target);
    // }
    return manhattan(size, state, target) + (2 * extra_moves);
}