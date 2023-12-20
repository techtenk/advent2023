

use std::thread;

use scan_rules::{scanner::{Word, Everything}, ScanError};

use crate::{helpers::get_input_lines, get_file_path};


pub fn run_part1() {
    let (instructions, nodes) = parse_input();
    let mut current = Some(nodes.iter().find(|item| item.name == ['A'; 3]).unwrap());
    let mut found = false;
    let mut count: i32 = 0;
    while !found {
        for i in instructions.iter() {
            count += 1;
            match i {
                'L' => {current = nodes.iter().find(|item| item.name == current.unwrap().left)},
                'R' => {current = nodes.iter().find(|item| item.name == current.unwrap().right)},
                _ => panic!("Instruction wasn't L or R!")
            }
            if current.unwrap().name == ['Z'; 3] {
                found = true;
                break;
            }
        }
    }
    println!("Found ZZZ after {} instructions", count);
}

pub fn run_part2() {
    let (instructions, nodes) = parse_input();
    // threads because they are fun!
    let start_nodes: Vec<&Node> = nodes.iter().filter(|item| item.name[2] == 'A').collect();
    let intervals = thread::scope(|s| {
        let mut handles = Vec::new();
        for n in start_nodes.iter() {
            let i_list_len = instructions.iter().len();
            let mut i_list = instructions.iter().cycle(); // nice
            let n_list = &nodes;
            let handle = s.spawn(move || {
                // each thread will generate a list of 'steps' where the node ends in 'Z'. We arbitrarily check 10k at a time
                let mut valid_ends = Vec::new();
                let mut current = *n;
                for step in 1..1000*i_list_len {
                    let i = i_list.next().unwrap();
                    match i {
                        'L' => { current = n_list.iter().find(|item| item.name == current.left).unwrap() },
                        'R' => { current = n_list.iter().find(|item| item.name == current.right).unwrap() },
                        _ => panic!("Invalid instruction!")
                    }
                    if current.name[2] == 'Z' { valid_ends.push(step) };
                }
                valid_ends
            });
            handles.push(handle);
        }
        // wait for all the threads to return, compile the list of the next start nodes
        let mut intervals = Vec::new();
        let mut valid_ends_list = Vec::new();
        for h in handles {
            let ends = h.join().unwrap();
            valid_ends_list.push(ends);
        }

        // for ends in valid_ends_list.iter() {
        //     let mut ei = ends.iter();
        //     println!(" first three: [{:?}, {:?}, {:?}, {:?}", ei.next(), ei.next(), ei.next(), ei.next());
        // }
        
        // collect the intervals on each starting node
        // silly input is just REALLY simple cycle where there is only on 'Z'-ending node per loop
        // so all we have to do is take the 2nd point we found and subtract the first

        for list in valid_ends_list {
            let interval = list.get(1).unwrap() - list.get(0).unwrap();
            intervals.push(interval);
        }
        intervals
    });
    // intervals is now calculated from each start node
    println!("List of intervals {:?}", intervals);
    println!("At this point I just plugged the intervals into a LCM calculator and moved on");

}

struct Node {
    name: [char; 3],
    left: [char; 3],
    right: [char; 3]
}

impl TryFrom<String> for Node {
    fn try_from(value: String) -> Result<Self, ScanError> {
        let node = scan! { &value;
            (let name: Word<String>, "=", "(", let left: Word<String>, ",", let right: Word<String>, let _: Everything) => {
                let mut name_arr = ['0', '0', '0'];
                let mut left_arr = ['0', '0', '0'];
                let mut right_arr = ['0', '0', '0'];
                let name = name.chars().enumerate().fold(& mut name_arr, |acc, (i,c)| {acc[i] = c; acc});
                let left = left.chars().enumerate().fold(& mut left_arr, |acc, (i,c)| {acc[i] = c; acc});
                let right = right.chars().enumerate().fold(& mut right_arr, |acc, (i,c)| {acc[i] = c; acc});
                Node { name: *name, left: *left, right: *right }
            }
        };
        node
    }
    type Error = ScanError;
}

fn parse_input() -> (Vec<char>, Vec<Node>) {
    let mut buf = Vec::new();
    let mut lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    let instructions: Vec<char> = lines.next().unwrap().unwrap().chars().collect();

    let mut nodes: Vec<Node> = Vec::new();
    // burn empty line
    lines.next();
    for node_option in lines {
        let line = node_option.unwrap_or_else(|_| { panic!("Error parsing line!"); });
        match line.try_into() {
            Ok(node) => nodes.push(node),
            Err(err) => { println!("Could not convert line to Node: {:?}", err)}
        }
    }
    (instructions, nodes)
}