
use std::{collections::HashMap, cmp::Ordering};
use scan_rules::scanner::NonSpace;

use crate::{helpers::get_input_lines, get_file_path};

pub fn run_part1() {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    let mut hands: Vec<Hand> = Vec::new();
    for l in lines {
        if let Ok(line) = l {
            let (cards, bid) = line.split(" ").fold(("".to_string(), 0 as i32), |acc, item| {
                match (acc.0.as_str(), acc.1) {
                    ("", _) => (item.to_string(), 0),
                    (_, 0) => (acc.0, item.parse::<i32>().unwrap()),
                    _ => panic!("Did not expect a third element on line!")
                }
            });
            hands.push(Hand::new(cards, bid));
        }
    }
    let winnings = calculate_winnings(hands);
    println!("Total winnings: {}", winnings);
}

fn calculate_winnings(mut hands: Vec<Hand>) -> i32 {
    hands.sort();

    let mut winnings = 0;
    for (rank, hand) in hands.iter().enumerate() {
        winnings += (rank+1) as i32 * hand.bid;
    }
    winnings
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
    bid: i32
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct Card {
    value: char
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        Card { value }
    }
}


impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let val = |x: char| {
            match x { 'A' => 14 as u8, 'K' => 13, 'Q' => 12, 'J' => 11, 'T' => 10, 'j' => 1, x => x.to_digit(10).unwrap() as u8 }
        };
        val(self.value).cmp(&val(other.value))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn new(cards: String, bid: i32) -> Hand {
        let hand_arr: [Card; 5] = {
            let mut card_iter = cards.chars();
            [card_iter.next(), card_iter.next(), card_iter.next(), card_iter.next(), card_iter.next()].map(|item| item.unwrap().into())
        };
        Hand {cards: hand_arr, bid}
    }

    fn hand_strength(&self) -> HandStrength {
            let mut card_map = HashMap::new();
            for card in self.cards.iter() {
                card_map.entry(card).and_modify(|e| *e += 1).or_insert(1 as u8);
            }
            // jokers are wild, so before evaluating the general strength, add the 'j' to the highest other block
            let jokers = card_map.remove(&Card { value: 'j' });
            if let Some(j) = jokers {
                let max_val = card_map.values_mut().max();
                match max_val {
                    Some(max_val) => {*max_val += j;},
                    None => {
                        // a whole hand of jokers!
                        return HandStrength::FIVEOFAKIND;
                    }
                }
            }
            match card_map.values().max().unwrap() {
                5 => HandStrength::FIVEOFAKIND,
                4 => HandStrength::FOUROFAKIND,
                3 => if card_map.values().any(|count| *count == 2) { HandStrength::FULLHOUSE } else { HandStrength::THREEOFAKIND },
                2 => if card_map.len() == 3 { HandStrength::TWOPAIR } else { HandStrength::ONEPAIR }
                _ => HandStrength::HIGHCARD
            }
    }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    if self.hand_strength() > other.hand_strength() {
        return Ordering::Greater;
    } else if self.hand_strength() < other.hand_strength() {
        return Ordering::Less;
    }
    // general strength is equal, check the cards
    let card_pairs = self.cards.iter().zip(other.cards.iter());
    card_pairs.fold(Ordering::Equal, | acc, pair | {
            if acc == Ordering::Equal {
                return pair.0.cmp(pair.1);
            }
            acc
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength {
    HIGHCARD,
    ONEPAIR,
    TWOPAIR,
    THREEOFAKIND,
    FULLHOUSE,
    FOUROFAKIND,
    FIVEOFAKIND
}

pub fn run_part2() {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);

    let mut hands = Vec::new();
    for l in lines {
        if let Ok(line) = l.as_ref() {
            let mut hand = scan! {line;
                (let hand: NonSpace<String>, let bid: i32) => Hand::new(hand, bid)
            };
            match hand.as_mut() {
                Ok(hand) => { 
                    hand.cards.iter_mut().for_each(|card| { if card.value == 'J' { card.value = 'j' }});
                    hands.push(hand.to_owned());
                },
                Err(scan_error) => { println!("Scan Error: {:?}", scan_error); }
            }
        }
    }

    let winnings = calculate_winnings(hands);

    println!("Total winnings: {}", winnings);
}