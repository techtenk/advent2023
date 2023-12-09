// pub const DAY_PART_1: u8 = 1;
// pub const DAY_PART_2: u8 = 2;
pub const DAY_ALL_PARTS: u8 = 3;
use crate::day1;
use crate::day2;
use crate::day3;

pub fn run(day: i32, parts: u8) {
    let run_part1 = parts & 1 > 0;
    let run_part2 = parts & 2 > 0;

    match day {
        1 => {
            if run_part1 {
                day1::run_part1();
            }
            if run_part2 {
                day1::run_part2();
            }
        },
        2 => {
            if run_part1 {
                day2::run_part1();
            }
            if run_part2 {
                day2::run_part2();
            }
        },
        3 => {
            if run_part1 {
                day3::run_part1();
            }
            if run_part2 {
                day3::run_part2();
            }
        }
        _ => {
            println!("Day {} not implemented", day);
        }
    }
}