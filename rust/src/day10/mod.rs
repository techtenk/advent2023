use std::{collections::HashMap, sync::Mutex};

use crate::{helpers::get_input_lines, get_file_path};

static mut LOOP_NODES: Vec<(usize, usize)> = Vec::new();
// NOW HERE IS A PATTERN YOU CAN'T UNDERSTAND, HOW I COULD JUST SINGLETON (KILL A MAN)
pub fn get_loop_nodes() -> Mutex<&'static mut Vec<(usize, usize)>> {
    unsafe {
        return Mutex::new(&mut LOOP_NODES);
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
    let start = grid.iter().find(|(_, item)| **item == 'S');
    
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
    state: InOutLoopState,
    unmatched: Vec<char>
}

pub fn run_part2() {
    if let Ok(nodes) = get_loop_nodes().try_lock().as_deref_mut() {
        // let's start by sorting the node list by the second element in each tuple
        nodes.sort_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)));

        // now for each line
        // iterate over the nodes in that line, keeping a state for inside/outside the loop
        // count +1 for each node that you pass over that isn't a loop node while in the "inside" state
        // state starts as "IN" because the first node on each line will open the loop and then it'll count to the next
        let grid = parse_input();
        
        // find the edges
        // | F L 7 J toggle the state
        // "-" does nothing
        let mut state = ScanState { line: 0, state: InOutLoopState::OUT, unmatched: Vec::new() };
        let mut node_iter = nodes.iter().peekable();
        let inner_blocks = {
            let mut count = 0;
            while let Some(node) = node_iter.next() {
                let next = node_iter.peek();
                println!("current: {:?} next: {:?}", node, next);
                
                println!("processing next node");

                // now update the state, based on what character it was
                // | toggles 
                // J and 7 
                // F and L 
                // - doesn't do anything
                // . shouldn't exist on the path
                // S, in our input is a J
                let lrl = state.unmatched.pop();
                match (lrl, grid.get(node)) {
                    (_, Some('|')) => state.state = if state.state == InOutLoopState::IN { InOutLoopState::OUT } else { InOutLoopState::IN },
                    (Some('F'), Some('J'|'S')) => state.state = if state.state == InOutLoopState::IN { InOutLoopState::OUT } else { InOutLoopState::IN },//toggle
                    (Some('F'), Some('7')) => {}, // do nothing
                    (Some('F'), Some('L')) => panic!("I can't see how you get 'L' after 'F'"),
                    (Some('F'), Some('F')) => panic!("I can't see how you get 'F' after 'F'"),
                    (Some('J'), Some('J')) => panic!("I can't see how you get 'J' after 'J'"),
                    (Some('J'), Some('7')) => panic!("I can't see how you get '7' after 'J'"),
                    (Some('J'), Some('L')) => state.state = if state.state == InOutLoopState::IN { InOutLoopState::OUT } else { InOutLoopState::IN }, //toggle
                    (Some('J'), Some('F')) => {
                        // stack on top
                        state.unmatched.push('J');
                        state.unmatched.push('F')
                    }
                    (Some('7'), Some('J')) => panic!("I can't see how you get 'J' after '7'"),
                    (Some('7'), Some('7')) => panic!("I can't see how you get '7' after '7'"),
                    (Some('7'), Some('L')) => {
                        state.unmatched.push('7');
                        state.unmatched.push('L');
                    },
                    (Some('7'), Some('F')) => {}, // do nothing
                    (Some('L'), Some('J')) => {}, // do nothing
                    (Some('L'), Some('7')) => state.state = if state.state == InOutLoopState::IN { InOutLoopState::OUT } else { InOutLoopState::IN }, // toggle
                    (Some('L'), Some('L')) => panic!("I can't see how you get 'L' after 'L'"),
                    (Some('L'), Some('F')) => panic!("I can't see how you get 'F' after 'L'"),
                    (None, Some('J')) => panic!("I can't see how you open a line with 'J'"),
                    (None, Some('7')) => panic!("I can't see how you open a line with '7'"), //panic
                    (None, Some('L')) => { state.unmatched.push('L'); }, // do nothing
                    (None, Some('F')) => { state.unmatched.push('F'); }, // do nothing
                    (Some(x), Some('-')) => { state.unmatched.push(x); }, // no match, just put same character back
                    (_,Some(x)) => panic!("Unrecognized character {}!", x),
                    (_,None) => panic!("Node in loop that's not in the grid! Node: {:?}", node)
                }

                // now the interesting part, if we are IN
                // count the number of nodes between this and the next
                if state.state == InOutLoopState::IN {
                    let new_count = match next {
                        Some(n) => n.0 - node.0 - 1, // minus one because it's strictly exclusive of both endpoints
                        None => 0//panic!("Not expected to end while state is open") // no more nodes on this line to enclose
                    };
                    println!("Adding {} tiles", new_count);
                    count += new_count;
                }

                // check if we are still on the same line, otherwise reset the state for the next line and continue
                if next.is_some() && next.unwrap().1 != state.line {
                    if state.state != InOutLoopState::OUT { panic!("Not sure how you can end a line in the \"IN\" state")};
                    state.state = InOutLoopState::OUT;
                    state.line = next.unwrap().1;
                    state.unmatched.clear();
                    println!("last node of line");
                }

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
