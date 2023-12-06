use std::env;

mod run_day;
mod helpers;
mod day1;
mod day2;
mod day3;

fn help() {
    println!("usage:
aoc2023 --help
    Print this help message
aoc2023 run [day]
    day can be:
      - a positive integer to run a single day
      - 'all' to run all days
      - latest (default) to run just the day with the highest index value
Examples:
    aoc2023 run 3
    aoc2023 run latest
    aoc2023 run all
      ");
}

fn main() {
    // check command line args, print usage if incorrect or has --help 
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            // a single parameter can be --help or run
            if !args[1].eq("run") {
                help();
            } else {
                println!("Run latest");
            }
        },
        3 => {
            if args[1].eq("run") {
                match args[2].as_str() {
                    "latest" => {
                        println!("Run latest");
                    },
                    "all"  => {
                        println!("Run all");
                    },
                    _ => {
                        // try to parse it as an integer, if it's not an integer, 0 won't match a day
                        let num = args[2].parse::<i32>().unwrap_or(0);
                        println!("Running day {}", num);
                        run_day::run(num, run_day::DAY_ALL_PARTS);
                    }
                }
            } else {
                help();
            }
        }
        _ => {
            help();
        }
    }
}
