
use crate::{helpers::get_input_lines, get_file_path};

pub fn run_part1() {
    let cards = parse_game_cards();
    let total = cards.iter().fold(0 as usize, |acc, item| item.get_points() + acc);
    println!("Total points: {}", total);
}

pub fn run_part2() {
    let cards = parse_game_cards();
    let mut copies = Vec::new();
    for (i, card) in cards.iter().enumerate() {
        {
            let num: Option<& mut usize> = copies.get_mut(i);
            match num {
                Some(x) => *x += 1 as usize,
                None => copies.insert(i, 1 as usize)
            };
        }
        let current_copies = *copies.get(i).to_owned().unwrap();
        for j in i+1..i+1+card.get_matches() {
            let number = copies.get_mut(j);
            match number {
                Some(x) => *x += current_copies,
                None => copies.insert(j, current_copies)
            }
        }
    }
    println!("Total cards: {}", copies.iter().sum::<usize>());
}

#[derive(Debug)]
#[allow(unused)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    player_numbers: Vec<usize>
}

impl Card {
    pub fn get_points(&self) -> usize {
        self.player_numbers.iter().fold(0 as usize, |acc, item| {
            if self.winning_numbers.contains(item) {
                return usize::max(1, acc * 2);
            }
            acc
        })
    }

    pub fn get_matches(&self) -> usize {
        self.player_numbers.iter().filter(|item| self.winning_numbers.contains(item)).count()
    }
}

fn parse_game_cards() -> Vec<Card>{
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);
    let mut cards = Vec::new();
    for line in lines {
        if let Ok(l) = line.as_ref() {
            // let parsed = Ok("something");
            let card: Result<Card, _> = scan! {l; 
                ("Card ", let game_no, ": ", [let winning_numbers: usize] {5,10}, " | ", [let player_numbers: usize] +) => Card { id: game_no, winning_numbers, player_numbers},
            };
            if let Ok(c) = card {
                cards.push(c);
            } else {
                println!("Unable to parse card! {:?}", card);
            }
            
        }
    }
    cards
}