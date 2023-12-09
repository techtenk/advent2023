use core::num;
use std::{collections::HashSet, hash::Hasher};
use std::hash::Hash;

use crate::{helpers::get_input_lines, get_file_path};
#[derive(Clone, Copy)]
struct SchemaNumber {
    positions: [(usize, usize); 3],
    number: i32
}
#[derive(Clone, Copy)]
struct SchemaSymbol {
    line: usize,
    column: usize,
    symbol: char
}
pub fn run_part1() {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    let (numbers, symbols) = read_schematic(lines);
    // now iterate over the numbers and check which are part numbers, add them up
    let mut total = 0;
    // println!("Parts:");
    for n in numbers {
        if is_part_number(&n, &symbols) {
            total += n.number;
            // print!("{}, ", n.number);
        } else {
            // print!("{}, ", n.number);
        }
    }
    // println!("");
    println!("Total: {}", total);
}

fn is_part_number(number: &SchemaNumber, symbol_list: &Vec<SchemaSymbol>) -> bool {
    // we will massage the data to be able to do a simple intersect. If it's not empty, we know it's a part number
    let mut adjacent_points: HashSet<(usize, usize)> = HashSet::new();
    let mut symbol_points: HashSet<(usize, usize)> = HashSet::new();
    for (i, p) in number.positions.iter().enumerate() {
        // check if positions are valid, probably should have used an Option instead
        if i > 0 && number.number < 10 {
            break;
        }
        if i > 1 && number.number < 100 {
            break;
        }
        if matches!(p, (1.., 1..)) {
            adjacent_points.insert((p.0 - 1, p.1 - 1));
        }
        if matches!(p, (1.., 0..)) {
            adjacent_points.insert((p.0 - 1, p.1));
            adjacent_points.insert((p.0 - 1, p.1 + 1));
        }
        if matches!(p, (0.., 1..)) {
            adjacent_points.insert((p.0, p.1 - 1));
            adjacent_points.insert((p.0 + 1, p.1 - 1));
        }
        adjacent_points.insert((p.0, p.1 + 1));
        adjacent_points.insert((p.0 + 1, p.1));
        adjacent_points.insert((p.0 + 1, p.1 + 1));
    }
    for s in symbol_list {
        symbol_points.insert((s.line, s.column));
    }
    adjacent_points.intersection(&symbol_points).any(|_| true) // if there any, return true, an empty iterator returns false
}

fn read_schematic(lines: std::io::Lines<std::io::BufReader<&[u8]>>) -> (Vec<SchemaNumber>, Vec<SchemaSymbol>) {
    let mut numbers_found: Vec<SchemaNumber> = Vec::new();
    let mut symbols_found: Vec<SchemaSymbol> = Vec::new();
    for (line_no, line) in lines.enumerate() {
        let mut running_number: String = "".to_string();
        let mut positions_list: Vec<(usize, usize)> = Vec::new(); 
        for (col_no, c) in line.unwrap().chars().enumerate() {
            match c {
                '0'..='9' => {
                    running_number = format!("{running_number}{c}");
                    positions_list.push((line_no, col_no));
                },
                _ => {
                    if !matches!(c, '.') {
                        // found a symbol
                        symbols_found.push(SchemaSymbol { line: line_no, column: col_no, symbol: c });
                    }
                    check_for_number(&running_number, &positions_list, &mut numbers_found);
                    
                    running_number = "".to_string();
                    positions_list.clear();
                }
            }
        }
        check_for_number(&running_number, &positions_list, &mut numbers_found)
    }

    (numbers_found, symbols_found)
}

fn check_for_number(running_number: &String, positions_list: &Vec<(usize, usize)>, numbers_found: &mut Vec<SchemaNumber>) {
    if let Ok(n) = running_number.parse::<i32>() {
        let mut positions = [(0 as usize, 0 as usize); 3];
        for i in 0..running_number.len() {
            if i == 3 {
                println!("should not happen");
            }
            positions[i] = positions_list.get(i).unwrap().to_owned();
        }
        numbers_found.push(SchemaNumber { number: n, positions: positions });
    }
}

struct Gear {
    parts: [SchemaNumber; 2]
}

impl Gear {
    pub fn get_ratio(&self) -> i64 {
        self.parts[0].number as i64 * self.parts[1].number as i64 
    }
}

pub fn run_part2() {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    let (numbers, symbols) = read_schematic(lines);

    let mut gears: Vec<Gear> = Vec::new();
    // find the gears by walking through the * symbols and checking for overlapping part numbers.
    // 2 numbers makes it a gear, add to vector
    for g in symbols.iter().filter(|item| item.symbol == '*') {
        let mut parts: Vec<SchemaNumber> = Vec::new();
        for part in numbers.iter() {
            if is_part_number(&part, &vec![*g]) {
                parts.push(*part);
            }
        }
        let mut parts_iter = parts.iter();
        if let Some(part1) = parts_iter.next() {
            if let Some(part2) = parts_iter.next() {
                gears.push(Gear { parts: [*part1, *part2] });
            }
        }
    }
    // now sum the gear ratios
    let gears_total = gears.iter().fold(0, |acc, item| {acc + item.get_ratio()});
    println!("Total Gear Ratio: {}", gears_total);

}