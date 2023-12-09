
use crate::{helpers::get_input_lines, get_file_path};

pub fn run_part1() {
    let mut buf = Vec::new();
    let mut lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    let times_line = lines.next().unwrap().unwrap();
    let distance_line = lines.next().unwrap().unwrap();

    let times = parse_line(times_line);
    let distances = parse_line(distance_line);
    let mut ways_to_win: Vec<i32> = Vec::new();
    
    for (race, race_time) in times.iter().enumerate() {
        let mut wins = 0;
        for t in 1..*race_time {
            let distance = (race_time - t) * t;
            if distance > *distances.get(race).unwrap() {
                wins += 1;
            }
        }
        ways_to_win.push(wins);
    }
    let moe = ways_to_win.into_iter().reduce(|acc, item| return acc * item).unwrap();
    println!("Margin of Error: {}", moe);
}

fn parse_line(line: String) -> Vec<i32> {
    let vec = line.split(" ").filter(|item| *item != "" && *item != "Time:" && *item != "Distance:").collect::<Vec<&str>>();
    return vec.into_iter().map(|item| item.parse::<i32>().unwrap()).collect();
}

fn parse_line_part2(line: String) -> Vec<i64> {
    let vec = line.chars().filter(|item| item.is_numeric()).collect::<Vec<char>>();
    let num_string = vec.into_iter().collect::<String>();
    return vec![num_string.parse::<i64>().unwrap()];
}

pub fn run_part2() {
    let mut buf = Vec::new();
    let mut lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    let times_line = lines.next().unwrap().unwrap();
    let distance_line = lines.next().unwrap().unwrap();

    let times = parse_line_part2(times_line);
    let distances = parse_line_part2(distance_line);
    let mut ways_to_win: Vec<i32> = Vec::new();
    
    for (race, race_time) in times.iter().enumerate() {
        let mut wins = 0;
        for t in 1..*race_time {
            let distance = (race_time - t) * t;
            if distance > *distances.get(race).unwrap() {
                wins += 1;
            }
        }
        ways_to_win.push(wins);
    }
    let moe = ways_to_win.into_iter().reduce(|acc, item| return acc * item).unwrap();
    println!("Margin of Error: {}", moe);
}