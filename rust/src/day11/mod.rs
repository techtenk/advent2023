use std::collections::HashMap;

use crate::{helpers::get_input_lines, get_file_path};

pub fn run_part1() {
    let galaxy = parse_input();
    let stars_list = galaxy.get_stars_list();
    // calculate disances between all pairs
    let mut total_distance: u32 = 0;
    // bubble sort ish move to get all pairs
    for left in stars_list.iter().enumerate() {
        for right in stars_list.iter().enumerate() {
            if right.0 <= left.0 { continue };
            total_distance += right.1.0.abs_diff(left.1.0) as u32 + right.1.1.abs_diff(left.1.1) as u32;
        }
    }
    println!("Total Distance: {}", total_distance);
}

pub fn run_part2() {
    let mut galaxy = parse_input();
    galaxy.old = true;
    let stars_list = galaxy.get_stars_list();
    let mut total_distance: u64 = 0;
    // bubble sort ish move to get all pairs
    for left in stars_list.iter().enumerate() {
        for right in stars_list.iter().enumerate() {
            if right.0 <= left.0 { continue };
            total_distance += right.1.0.abs_diff(left.1.0) as u64 + right.1.1.abs_diff(left.1.1) as u64;
        }
    }
    println!("Total Distance: {}", total_distance);
}

struct GalaxyMap {
    orig_input: Vec<String>,
    old: bool
}

impl GalaxyMap {
    fn get_stars_list(&self) -> Vec<(usize, usize)> {
        let mut empty_rows = HashMap::new();
        let mut empty_cols = HashMap::new();

        for (row, line) in self.orig_input.iter().enumerate() {
            let row_empty = line.chars().fold(true, |acc, c| if c == '#' { return false } else { acc });
            empty_rows.insert(row, row_empty);
            for (col, c) in line.chars().enumerate() {
                let entry = empty_cols.entry(col).or_insert(if c == '#' { false } else { true });
                *entry = if c == '#' { false } else { *entry };
            }
        }
        // println!("Empty rows: {:?}", empty_rows.into_iter().filter(|item| item.1).collect::<Vec<(usize, bool)>>());
        // println!("Empty cols: {:?}", empty_cols.into_iter().filter(|item| item.1).collect::<Vec<(usize, bool)>>());
        let mut star_list = Vec::new();
        for (row, line) in self.orig_input.iter().enumerate() {
            let row_offset = GalaxyMap::calc_offset(&empty_rows, row, self.old);
            for (col, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
                let col_offset = GalaxyMap::calc_offset(&empty_cols, col, self.old);
                star_list.push((col + col_offset, row + row_offset));
            }
        }
        star_list
    }

    fn calc_offset(empty_list: &HashMap<usize, bool>, index: usize, old_galaxy: bool) -> usize {
        let factor = if old_galaxy { 999999 } else { 1 } as usize;
        
        empty_list.iter().filter(|item| *item.1 == true && *item.0 < index).count() * factor
    }
}

fn parse_input() -> GalaxyMap {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    GalaxyMap { orig_input: lines.map(|item| item.unwrap().to_string()).into_iter().collect(), old: false }
}