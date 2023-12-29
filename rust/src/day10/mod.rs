use std::{collections::HashMap, sync::Mutex, ops::DerefMut};

use crate::{helpers::get_input_lines, get_file_path};

static mut loop_nodes: Vec<(usize, usize)> = Vec::new();
// NOW HERE IS A PATTERN YOU CAN'T UNDERSTAND, HOW I COULD JUST SINGLETON (KILL A MAN)
pub fn get_loop_nodes() -> Mutex<&'static mut Vec<(usize, usize)>> {
    unsafe {
        return Mutex::new(&mut loop_nodes);
    }
}


pub fn run_part1() {
    let grid =  {
        // let mut fullgrid = parse_input();
        // // find the loop, then null out (replace with '.' the rest of the nodes)
        // let start = fullgrid.iter().find(|(pos, item)| **item == 'S').unwrap();
        // let mut visited: Vec<(usize, usize)> = Vec::new();
        // let mut pending = Vec::new();
        // pending.push(*start.0);
        // while pending.len() > 0 {

        // }
        parse_input()
    };
    let start = grid.iter().find(|(pos, item)| **item == 'S');
    
    if let Some(start_pos) = start {
        println!("Start is at: {},{}", start_pos.0.0, start_pos.0.1);
        let mut found_nodes_left = Vec::new();
        let mut found_nodes_right = Vec::new();
        found_nodes_left.push(*start_pos.0);
        let next_connections = find_connections(&grid, start_pos.0, None);
        // start checking each direction
        let mut next_left = next_connections.0;
        let mut next_right = next_connections.1;
        let mut left_steps = 0;
        let mut right_steps = 0;
        while next_left.is_some() && !found_nodes_left.contains(&next_left.unwrap()) && !found_nodes_right.contains(&next_left.unwrap()) {
            // println!("Found next left at {:?}", next_left);
            found_nodes_left.push(next_left.unwrap());
            next_left = find_connections(&grid, &next_left.unwrap(), found_nodes_left.get(found_nodes_left.len() - 2)).0;
            left_steps += 1;

            if next_right.is_some() && !found_nodes_left.contains(&next_right.unwrap()) && !found_nodes_right.contains(&next_right.unwrap()) {
                // println!("Found next right at {:?}", next_right);
                found_nodes_right.push(next_right.unwrap());
                let last_right_node = if found_nodes_right.len() < 2 { None } else { found_nodes_right.get(found_nodes_right.len() - 2) };
                next_right = find_connections(&grid, &next_right.unwrap(), last_right_node).0;
                right_steps += 1;
            }
            // println!("Left nodes: {:?}", found_nodes_left);
            // println!("Right nodes: {:?}", found_nodes_right);
            // println!("Next Left: {:?}", next_left);
            // println!("{:?} {:?} {:?} Will continue: {:?}", next_left.is_some(), next_left.is_some() && !found_nodes_left.contains(&next_left.unwrap()), next_left.is_some() && !found_nodes_right.contains(&next_left.unwrap()), next_left.is_some() && !found_nodes_left.contains(&next_left.unwrap()) && !found_nodes_right.contains(&next_left.unwrap()));
        }

        match left_steps > right_steps {
            true => println!("Most steps taken was on the left path: {}", left_steps),
            false => println!("Most steps taken was on the right path: {}", right_steps)
        }
        // save the node loop for part 2
        if let Ok(nodes) = get_loop_nodes().try_lock().as_deref_mut() {
             nodes.append(& mut found_nodes_left);
             nodes.append(& mut found_nodes_right);
        };
    }
}

fn find_connections(grid: &HashMap<(usize, usize), char>, node: &(usize, usize), last_node: Option<&(usize, usize)>) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let mut connections = (None, None);
    let mut add_to_connections = |node: (usize, usize)| {
        if connections.0.is_none() {
            connections.0 = Some(node);
            return;
        }
        connections.1 = Some(node);
    };
    // get the chars
    let me = grid.get(node).unwrap();
    let above = if node.1 > 0 { grid.get_key_value(&(node.0, node.1 - 1)) } else { grid.get_key_value(&(usize::MAX, usize::MAX)) };
    let right = grid.get_key_value(&(node.0 + 1, node.1 ));
    let down = grid.get_key_value(&(node.0, node.1 + 1));
    let left = if node.0 > 0 { grid.get_key_value(&(node.0 - 1, node.1))} else { grid.get_key_value(&(usize::MAX, usize::MAX)) };

    if matches!(me, 'S'|'|'|'J'|'L') && above.map(|x| x.0) != last_node {
        // check above
        match above {
            Some(x) if matches!(x.1, '|'|'F'|'7') => {
                // println!("I'm adding top path because I'm {:?} and it is {} and last_node is {:?}", grid.get_key_value(node).unwrap(), x.1, last_node);
                add_to_connections(*x.0);
            },
            _ => {}
        }
    }
    

    if matches!(me, 'S'|'-'|'F'|'L') && right.map(|x| x.0) != last_node {
        // check right
        match right {
            Some(x) if matches!(x.1, '-'|'J'|'7') => {
                // println!("I'm adding right path because I'm {:?} and it is {} and last_node is {:?}", grid.get_key_value(node).unwrap(), x.1, last_node);
                add_to_connections(*x.0);
            },
            _ => {}
        }
    }
    

    if matches!(me, 'S'|'|'|'F'|'7') && down.map(|x| x.0) != last_node {
        // check down
        match down {
            Some(x) if matches!(x.1, '|'|'J'|'L') => {
                // println!("I'm adding bottom path because I'm {:?} and it is {} and last_node is {:?}", grid.get_key_value(node).unwrap(), x.1, last_node);
                add_to_connections(*x.0);
            },
            _ => {}
        }
    }
    

    if matches!(me, 'S'|'-'|'J'|'7') && left.map(|x| x.0) != last_node {
        // check left
        match left {
            Some(x) if matches!(x.1, '-'|'F'|'L') => {
                // println!("I'm adding left path because I'm {:?} and it is {} and last_node is {:?}", grid.get_key_value(node).unwrap(), x.1, last_node);
                add_to_connections(*x.0);
            },
            _ => {}
        }
    }
    
    connections
}

#[derive(PartialEq)]
enum InOutLoopState {
    IN,
    OUT
}

struct ScanState {
    line: usize,
    state: InOutLoopState
}

pub fn run_part2() {
    if let Ok(nodes) = get_loop_nodes().try_lock().as_deref_mut() {
        // let's start by sorting the node list by the second element in each tuple
        nodes.sort_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)));

        // now for each line
        // iterate over the nodes in that line, keeping a state for inside/outside the loop
        // count +1 for each node that you pass over that isn't a loop node while in the "inside" state
        // state starts as "IN" because the first node on each line will open the loop and then it'll count to the next
        let mut state = ScanState { line: 0, state: InOutLoopState::IN };
        let mut node_iter = nodes.iter().peekable();
        let inner_blocks = {
            let mut count = 0;
            while let Some(node) = node_iter.next() {
                let next = node_iter.peek();
                println!("current: {:?} next: {:?}", node, next);
                // check if we are still on the same line, otherwise reset the state for the next line and continue
                if next.is_some() && next.unwrap().1 != state.line {
                    state.state = InOutLoopState::IN;
                    state.line = next.unwrap().1;
                    // now check if the next node is on the same line, if not, then continue 
                    println!("last node of line");
                    continue;
                }
                
                if next.is_some() && next.unwrap().0 - node.0 == 1 {
                    println!("next node is adjacent");
                    state.line = node.1;
                    // if it's not a dash, change the IN/OUT state
                    
                    continue;
                }
                
                println!("processing next node");
                
                // now the interesting part, if we are IN, then count the number of nodes between this and the next
                if state.state == InOutLoopState::IN {
                    let new_count = match next {
                        Some(n) => n.0 - node.0 - 1, // minus one because it's strictly exclusive of both endpoints
                        None => 0 // no more nodes on this line to enclose
                    };
                    println!("Adding {} tiles", new_count);
                    count += new_count;
                }

                // now update the state, just toggle it
                state.state = if state.state == InOutLoopState::IN { InOutLoopState::OUT } else { InOutLoopState::IN };
                state.line = node.1;
            }
            count
        };
        println!("Found {} tiles enclosed", inner_blocks);
    }

}

fn parse_input() -> HashMap<(usize, usize), char> {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    let mut grid = HashMap::new();
    for (row, l) in lines.enumerate() {
        let line = l.unwrap();
        for (col, c) in line.chars().enumerate() {
            grid.insert((col, row), c);
        }
    }
    grid
}
