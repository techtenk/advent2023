use std::{collections::HashMap};

use crate::{helpers::get_input_lines, get_file_path};

use scan_rules::scanner::{NonSpace, Everything};
use regex::{self, Regex};

pub fn run_part1() {
    // basic idea is to create a Regex out of the groups of damaged springs and check it against the input
    // we could generate all of the permutations of each line and check each one
    // but for efficiency, let's use ? as a wildcard and start filling in possibilites from the left
    // once we find a path that doesn't work anymore with the wildcards, then we stop
    // and once we reach the end of the input, having filled out all the ? with their concrete type, then we count it valid

    let records = parse_input();
    let mut total_arrangements = 0;
    let unmatched_group_regex = Regex::new("(^|\\.)(#*\\?)").unwrap();

    for r in records {
        // println!("{:?}", r);
        // println!("{:?}", r.get_regex());
        let a = r.get_arrangements_count(None, &unmatched_group_regex);
        // println!("{} has {} arrangements", r.springs, a);
        total_arrangements += a.combinations;
    }
    println!("Total arrangements: {}", total_arrangements);
}

pub fn run_part2() {
    let mut records = parse_input();

    let mut total_arrangements: u64 = 0;
    let unmatched_group_regex = Regex::new("(^|\\.)(#*\\?)").unwrap();


    records.iter_mut().for_each(|record| record.unfold());
    for (line_no, r) in records.iter().enumerate() {
        println!("Processing Record #: {}", line_no);
        // println!("{:?}", r);
        // println!("{:?}", r.get_regex());
        let a = r.get_arrangements_count(None, &unmatched_group_regex).combinations as u64;
        // println!("{} has {} arrangements", r.springs, a);
        total_arrangements += a;
    }
    println!("Total arrangements part 2: {}", total_arrangements);
}

#[derive(Debug)]
struct ConditionRecord {
    springs: String,
    regex: Regex,
    groups: Vec<usize>
}

#[derive(PartialEq, Eq, Hash)]
struct GroupIndexInsertPoint(usize, usize);

struct ArrangementResult {
    combinations: u32,
    map: Option<HashMap<GroupIndexInsertPoint, u32>>
}

impl ConditionRecord {
    pub fn new(springs: String, groups: Vec<usize>) -> ConditionRecord {
        ConditionRecord {
            springs,
            regex: ConditionRecord::get_regex(&groups),
            groups
        }
    }
    pub fn get_regex(groups: &Vec<usize>) -> Regex {
        let mut reg_string = "^[\\.?]*".to_string();
        for (i, g) in groups.iter().enumerate() {
            if i > 0 { reg_string.push_str("[\\.?]+"); }
            reg_string.push_str("[#?]{");
            reg_string.push_str(&g.to_string());
            reg_string.push_str("}");
        }
        reg_string.push_str("[\\.?]*$");
        Regex::new(&reg_string).unwrap()
    }

    pub fn get_arrangements_count(&self, record_string: Option<String>, unmatched_group_regex: &Regex) -> ArrangementResult {
        let rs = match record_string {
            Some(x) => x,
            None => self.springs.to_owned()
        };
        let mut map: HashMap<GroupIndexInsertPoint, u32> = HashMap::new();
        // base case, there are no more '?' in the string, so we just see if it matches and return 1 or 0
        if !rs.contains('?') {
            match self.regex.is_match(&rs) {
                true => {
                    // println!("regex: {} matches {}", self.regex.as_str(), &rs);
                    return ArrangementResult { combinations: 1, map: None };
                },
                false => return ArrangementResult { combinations: 0, map: None }
            }
        }

        // second base case, if the pattern doesn't match now, then we can short circuit the rest of the attempts
        if !self.regex.is_match(&rs) {
            // println!("regex: {} does not match {}", self.regex, &rs);
            return ArrangementResult { combinations: 0, map: None };
        }

        // fill in the next '?' with '.' or the next group of '#' and recurse on each
        let mut arrangements = 0;
        let (next_group_index, next_group_size) = self.next_group_to_match(&rs);
        let mut next_record_broken = rs.to_owned();

        if next_group_size > 0 {
            if let Some(first_unmatched_group) = unmatched_group_regex.captures_iter(&next_record_broken).next() {
                let insert_point = first_unmatched_group.get(2).unwrap().start();
                let next_broken = vec!["#"; next_group_size].join("");
                let before = next_record_broken.to_owned();
                // println!("Next record before replace: {}", next_record_broken);
                next_record_broken.replace_range(insert_point..(usize::min(insert_point+next_broken.len(), next_record_broken.len())), next_broken.as_str());
                // println!("Next record after replace: {}", next_record_broken);

                // check if we've solved this before
                match map.get(&GroupIndexInsertPoint(next_group_index, insert_point)) {
                    Some(r) => return ArrangementResult { combinations: *r, map: None },
                    None => {} // continue on
                }

                if before != next_record_broken {
                    let result = self.get_arrangements_count(Some(next_record_broken), &unmatched_group_regex);
                    arrangements += result.combinations;
                    if result.map.is_some() {
                        map.extend(result.map.unwrap());
                    }
                }
            } else {
                // no unmatched groups left, return ?
                println!("Nowhere to start the next group of {} in {}", next_group_size, next_record_broken);
                panic!("Need to validate this is a possibility and if we return 0 or 1");
                // return ArrangementResult { combinations: 0, map: None };
                
            }
            
            

        } else {
            // println!("All groups match! {}", rs);
            return ArrangementResult { combinations: 1, map: Some(map) };
        }
        let insert_point = rs.find("?").unwrap();
        if let Some(existing_entry) = map.get(&GroupIndexInsertPoint(next_group_index, insert_point)) {
            return ArrangementResult { combinations: *existing_entry, map: None };
        }
        let result = self.get_arrangements_count(Some(rs.replacen("?", ".", 1).to_string()), &unmatched_group_regex);
        if let Some(m) = result.map {
            map.extend(m);
        }
        arrangements += result.combinations;
        map.insert(GroupIndexInsertPoint(next_group_index, insert_point), arrangements);
        ArrangementResult { combinations: arrangements, map: Some(map) }
    }

    fn next_group_to_match(&self, current_records: &String) -> (usize, usize) {
        let mut existing_groups = Vec::new();
        // existing groups of broken springs
        for c in current_records.split(".") {
            match c {
                x if !x.is_empty() && x.chars().all(|i| i == '#') => {
                    existing_groups.push(c.len())
                },
                "" => {}, // do nothing but don't break yet
                _ => {
                    break;
                } // add no more when we find a question mark
            }
        }
        // find the first expected group that doesn't match existing, return it
        for (i, g) in self.groups.iter().enumerate() {
            if let Some(next) = existing_groups.get(i) {
                if *next != *g {
                    return (i, *g);
                }
            } else {
                return (i, *g);
            }
        }
        (usize::MAX, 0) // all groups match!
    }

    pub fn unfold(& mut self) {
        self.springs = format!("{}?{}?{}?{}?{}", self.springs, self.springs, self.springs, self.springs, self.springs);
        let orig_len = self.groups.len();
        for _ in 0..4 {
            self.groups.extend_from_within(0..orig_len);
        }
        self.regex = ConditionRecord::get_regex(&self.groups);
    }

}

fn parse_input() -> Vec<ConditionRecord> {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    let mut records: Vec<ConditionRecord> = Vec::new();
    for line in lines.map(|l| l.unwrap()) {
        let record = scan!{line.as_str();
            (let springs: NonSpace<String>, let groups: Everything) => {
                let g: Vec<usize> = groups.split(",").map(|item| item.parse::<usize>().unwrap()).collect();
                ConditionRecord::new(springs, g)
            }
        };
        records.push(record.unwrap());
    }
    records
}
