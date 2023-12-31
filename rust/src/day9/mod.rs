use std::ops::{Add, Sub};

use crate::{helpers::get_input_lines, get_file_path};

pub fn run_part1() {
    let histories = parse_input();
    let mut total = 0;
    for hist in histories {
        total += hist.get_prediction();
    }
    println!("Total: {}", total);
}

pub fn run_part2() {
 let histories: Vec<ValueHistory> = parse_input();
 let mut total = 0;
 for hist in histories {
    total += hist.get_prediction_reverse();
}
println!("Total: {}", total);
}

struct ValueHistory {
    history: Vec<i32>
}

impl ValueHistory {
    fn get_prediction(&self) -> i32 {
        // println!("\nNew value history:");
        Self::recurse_get_prediction(&self.history, false)
    }

    fn get_prediction_reverse(&self) -> i32 {
        // println!("\nNew value history:");
        Self::recurse_get_prediction(&self.history, true)
    }

    fn recurse_get_prediction(list: &Vec<i32>, reverse: bool) -> i32 {
        // println!("{:?}", list);
        // base case
        if list.iter().all(|item| *item == 0) {
            return 0;
        }

        // create a new list of the difference of each pair
        let mut last = None;
        let mut diff_list = Vec::new();
        for num in list {
            if last.is_some() {
                diff_list.push(num - last.unwrap());
            }
            last = Some(num);
        }

        let (first_num, add_or_sub): (i32, fn(i32, i32) -> <i32 as Add<i32>>::Output) = match reverse {
            true => (*list.iter().next().unwrap(), i32::sub),
            false => (*list.iter().last().unwrap(), i32::add)
        };

        add_or_sub(first_num, Self::recurse_get_prediction(&diff_list, reverse))
    }
}

impl Into<ValueHistory> for String {
    fn into(self) -> ValueHistory {
        let history = scan! { &self;
            ([let i: i32]*) => i
        }.unwrap();
        
        ValueHistory { history }
    }
}

fn parse_input() -> Vec<ValueHistory> {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    lines.map(|line| {
        line.unwrap().into()
    }).collect()
}