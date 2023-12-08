use crate::{helpers::get_input_lines, get_file_path};

struct SchemaNumber {
    positions: [(usize, usize); 3],
    number: i32
}
struct SchemaSymbol {
    line: usize,
    column: usize,
    symbol: char
}
pub fn run_part1() {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    let mut numbers_found: Vec<SchemaNumber> = Vec::new();
    for (line_no, line) in lines.enumerate() {
        let mut running_number: String = "".to_string();
        let mut positions_list: Vec<(usize, usize)> = Vec::new(); 
        for (col_no, c) in line.unwrap().chars().enumerate() {
            match c {
                '0'..='9' => {
                    running_number = format!("{running_number}{c}");
                    positions_list.push((line_no, col_no));
                },
                '.'.. => {
                    let n = running_number.parse::<i32>().unwrap();
                    let mut positions = [(0 as usize, 0 as usize); 3];
                    for i in 0..3 {
                        positions[i] = positions_list.get(i).unwrap().to_owned();
                    }
                    numbers_found.push(SchemaNumber { number: n, positions: positions });
                    running_number = "".to_string();
                    positions_list.clear();
                },
                _ => println!("Heyo")
            }
        }
    }
}