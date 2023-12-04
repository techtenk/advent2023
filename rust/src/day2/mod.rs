use crate::{helpers::get_input_lines, get_file_path};

struct CubeBag {
    reds: i32,
    greens: i32,
    blues: i32
}

impl CubeBag {
    pub fn new(reds: i32, greens: i32, blues: i32)  -> CubeBag {
        CubeBag { reds, greens, blues }
    }

    pub fn take(& mut self, reds: i32, greens: i32, blues: i32) -> Option<CubeBag> {
        if reds > self.reds || greens > self.greens || blues > self.blues {
            return None;
        }
        self.reds -= reds;
        self.greens -= greens;
        self.blues -= blues;
        
        Some(CubeBag { reds, greens, blues })
    }

    pub fn replace(& mut self, cubes: CubeBag) {
        self.reds += cubes.reds;
        self.greens += cubes.greens;
        self.blues += cubes.blues;
    }
}

pub fn run_part1() {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);
    let mut possible_games = Vec::new();
    for (game_id, line) in lines.enumerate() {
        let mut bag = CubeBag::new(12, 13, 14);
        let game = line.unwrap();
        let mut draws = game.split(&[':',';','\n']);
        // throw away first one, it's the game id
        draws.next();
        let mut possible = true;
        for draw in draws {
            let (red, green, blue) = get_cubes_from_desc(draw.to_string());
            let hand = bag.take(red, green, blue);
            if hand.is_none() {
                possible = false;
                break;
            } else {
                bag.replace(CubeBag { reds: red, greens: green, blues: blue });
            }
            
        }
        if possible {
            possible_games.push((game_id + 1) as i32);
        }
    }
    println!("Total of games possible: {}", possible_games.iter().sum::<i32>());
}

fn get_cubes_from_desc(desc: String) -> (i32, i32, i32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for result in desc.split(",") {
        let mut color_number = result.trim().split(" ");
        let number = color_number.next();
        let color = color_number.next();
        match color.unwrap() {
            "red" => {
                red = number.unwrap().parse::<i32>().unwrap();
            },
            "green" => {
                green = number.unwrap().parse::<i32>().unwrap();
            },
            "blue" => {
                blue = number.unwrap().parse::<i32>().unwrap();
            },
            _ => ()
        };
    }
    (red, green, blue)
}

pub fn run_part2() {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);
    let mut total_power: i32 = 0;
    for line in lines {
        let mut min_reds = 0;
        let mut min_greens = 0;
        let mut min_blues = 0;
        if let Ok(game) = line {
            let mut draws = game.split([':', ';']);
            // throw away first draw
            draws.next();
            for draw in draws {
                let (red, green, blue) = get_cubes_from_desc(draw.to_string());
                min_reds = i32::max(min_reds, red);
                min_greens = i32::max(min_greens, green);
                min_blues = i32::max(min_blues, blue);
            }
            let game_power = min_reds * min_greens * min_blues;
            total_power += game_power;
        }
    }
    println!("Total power: {}", total_power);
}