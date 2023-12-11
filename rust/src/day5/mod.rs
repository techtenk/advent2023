use std::ops::Range;

use crate::{get_file_path, helpers::get_input_lines};
use scan_rules::scanner::{Everything, NonSpace, until_pat_str};

pub fn run_part1() {
    let (seeds, maps) = parse_maps();
    let mut closest_location: i64 = i64::MAX;
    for seed in seeds {
        closest_location = closest_location.min(get_closest_location(seed..(seed+1), &maps))
    }
    println!("Closest Location: {}", closest_location);
}

fn get_closest_location(seeds: Range<i64>, maps: & Vec<AlmanacMap>) -> i64 {
    let mut closest_location = i64::MAX;
    let mut seeds_processed:  i128 = 0;
    for seed in seeds {
        // find its destination
        let mut current_src = seed;
        let mut current_map = Some(MapType::SeedSoil);
        while current_map != None {
            // for all the current_map type, check if any map our src
            for map_of_type in maps.iter().filter(|item| item.map_type == current_map.unwrap()) {
                match map_of_type.get_dest(current_src) {
                    Some(x) => {
                        current_src = x;
                        break;
                    },
                    None => {
                        // this map didn't match, so we check the next
                    }
                }
            }
            /* at this point we have either found a map and updated the src for the next map type, or we have
               checked all the maps of the current type and none mapped the src, so the dest stays the same
            */
            // move onto the next map type
            current_map = MapType::get_next_map_type(current_map.unwrap());
        }
        // we've made it through all the map types, so now 'current_src' is the location
        closest_location = closest_location.min(current_src);
        seeds_processed += 1;
        if seeds_processed % 1000000 == 0 {
            println!("Seeds processed: {}", seeds_processed);
        }
    }
    closest_location
}

pub fn run_part2() {
    // for part two, reinterpret the seeds line and run again
    let (seeds, maps) = parse_maps();
    let mut seed_iter = seeds.into_iter();
    let mut closest_location: i64 = i64::MAX;
    while let Some(seed_start) = seed_iter.next() {
        if let Some(range_len) = seed_iter.next() {
            let range_seeds = seed_start..(seed_start + range_len);
            closest_location = closest_location.min(get_closest_location(range_seeds, &maps));
        }
    }
    println!("Closest Location: {}", closest_location);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MapType {
    SeedSoil,
    SoilFertilizer,
    FertilizerWater,
    WaterLight,
    LightTemp,
    TempHumid,
    HumidLocation,
    Unknown
}

impl MapType {
    pub fn get_next_map_type(current: MapType) -> Option<MapType> {
        match current {
            MapType::SeedSoil => Some(MapType::SoilFertilizer),
            MapType::SoilFertilizer => Some(MapType::FertilizerWater),
            MapType::FertilizerWater => Some(MapType::WaterLight),
            MapType::WaterLight => Some(MapType::LightTemp),
            MapType::LightTemp => Some(MapType::TempHumid),
            MapType::TempHumid => Some(MapType::HumidLocation),
            _ => None
        }
    }
}

impl From<String> for MapType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "seedsoil" => MapType::SeedSoil,
            "soilfertilizer" => MapType::SoilFertilizer,
            "fertilizerwater" => MapType::FertilizerWater,
            "waterlight" => MapType::WaterLight,
            "lighttemperature" => MapType::LightTemp,
            "temperaturehumidity" => MapType::TempHumid,
            "humiditylocation" => MapType::HumidLocation,
            _ => MapType::Unknown
        }
    }
}

#[derive(Debug)]
struct AlmanacMap {
    map_type: MapType,
    start: i64,
    dest: i64,
    len: i64
}

impl AlmanacMap {
    pub fn get_dest(&self, src: i64) -> Option<i64> {
        if (self.start..self.start+self.len).contains(&src) {
            return Some(self.dest + (src-self.start));
        }
        None
    }    
}

fn parse_maps() -> (Vec<i64>, Vec<AlmanacMap>) {
    let mut buf = Vec::new();
    let lines = get_input_lines(&get_file_path!("input.txt"), & mut buf);
    
    let mut maps: Vec<AlmanacMap> = Vec::new();
    let mut seeds: Option<Vec<i64>> = None;
    let mut current_map_type: Option<MapType> = None;

    for line in lines {
        if let Ok(l) = line.as_ref() {
            let result = scan! {l;
                ("seeds:", [let initial_seeds: i64] +) => { seeds = Some(initial_seeds); },
                ("") => { /* no action for blank lines */},
                (let this<| until_pat_str("-"), "-to-", let that: NonSpace<String>, let _: Everything) => { 
                    current_map_type = match format!("{this}{that}").into() {
                        MapType::Unknown => { println!("No map for line: {}", l); None},
                        x => Some(x)
                    }
                },
                (let dest, let start, let len) => {
                    let new_map = AlmanacMap {
                        map_type: current_map_type.unwrap(),
                        start,
                        dest,
                        len
                    };
                    maps.push(new_map);
                 }
                // (..other) => { /* do nothing */}
            };
            if let Err(err) = result {
                println!("Error parsing line: {} \n Error: {:?}", l, err);
            }
        } else {
            println!("Could not read line: {:?}", line);
        }
    }
    
    // println!("seeds {:?}", seeds);
    // println!("maps: {:?}", maps);
    
    
    (seeds.unwrap(), maps)
}