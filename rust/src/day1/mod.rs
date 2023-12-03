use core::num;

use crate::{get_file_path, helpers::{self, get_input_lines}};
use regex::Regex;
use regex::CaptureMatches;

pub fn run_part1() {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    let mut numbers = Vec::new();
    // find the first and last numbers in each line
    for line in lines {
        let number = find_first_last(line.unwrap(), Part::ONE);
        numbers.push(number.parse::<i32>().unwrap());
    }
    println!("Total: {}", numbers.iter().sum::<i32>());
}

enum Part {
    ONE,
    TWO
}

fn find_first_last(line: String, part: Part) -> String {
    // get part 1 or 2 Regex
    let regex: Regex = match part {
        Part::ONE => Regex::new("[0-9]").unwrap(),
        _ => Regex::new("[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap()
    };
    let reverse_regex = match part {
        Part::ONE => Regex::new("[0-9]").unwrap(),
        _ => Regex::new("enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|[0-9]").unwrap()
    };
    let mut matches = regex.find_iter(&line);
    let rev_line = line.chars().rev().collect::<String>();
    let mut rev_matches = reverse_regex.find_iter(&rev_line);
    let mut first = matches.next().unwrap().as_str();
    let mut last = rev_matches.next().unwrap().as_str();
    first = match first {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => first
    };
    last = match last {
        "eno" => "1",
        "owt" => "2",
        "eerht" => "3",
        "ruof" => "4",
        "evif" => "5",
        "xis" => "6",
        "neves" => "7",
        "thgie" => "8",
        "enin" => "9",
        _ => last
    };
    return format!("{first}{last}");
}

pub fn run_part2() {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    let mut numbers = Vec::new();
    // find the first and last numbers in each line
    for line in lines {
        let line_string = line.unwrap();
        print!("{}", line_string);
        let number = find_first_last(line_string, Part::TWO);
        numbers.push(number.parse::<i32>().unwrap());
        // println!("\n{number}");
    }
    println!("Total: {}", numbers.iter().sum::<i32>());
}