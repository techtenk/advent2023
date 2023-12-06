use crate::{helpers::get_input_lines, get_file_path};

struct SchemaNumber {
    line: usize,
    column: usize,
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

    for (line_no, line) in lines.enumerate() {
        let mut running_number: Option<&str> = None;
        for (col_no, c) in line.unwrap().chars().enumerate() {
            match c {
                '0'..='9' => println!("Hey"),
                _ => println!("Heyo")
            }
        }
    }
}