advent_of_code::solution!(5);

use cached::proc_macro::cached;
use regex::Regex;
use std::{collections::BTreeMap, ops::Range};
#[derive(Debug, Clone)]
struct MapRange {
    source: Option<Range<u64>>,
    destination: Option<Range<u64>>,
}

#[derive(Debug, Clone)]
struct InstructionMap {
    maps: Vec<MapRange>,
}

impl MapRange {
    fn new() -> MapRange {
        MapRange {
            source: None,
            destination: None,
        }
    }
    fn is_empty(&self) -> bool {
        self.destination.is_none() && self.source.is_none()
    }
    fn get(&self, value: &u64) -> Option<u64> {
        // Check whether the value is contained witin an input range
        // If it is, then we need to do a lookup
        if let Some(source) = self.source.clone() {
            if source.contains(&(*value as u64)) {
                // We need to do a lookup
                let offset = value - source.start as u64;
                if let Some(destination) = &self.destination {
                    return Some(destination.start + offset);
                }
            }
        }
        None
    }
    fn split_source_by_other_dest(&self, other_range: MapRange) -> Option<MapRange> {
        // Maps the destination of the other map to a new source map. If there is no overlap
        // Return None
        let other_dest = other_range.destination.unwrap();
        let source = self.source.as_ref().unwrap();
        let dest = self.destination.as_ref().unwrap();

        if other_dest.start >= source.start && other_dest.start <= source.end {
            let new_source_start = other_dest.start;
            let new_source_end = vec![other_dest.end, source.end]
                .iter()
                .min()
                .unwrap()
                .clone();
            let new_range = new_source_end - new_source_start;
            let offset = other_dest.start - source.start;

            let new_dest_start = (dest.start) + offset;
            let new_dest_end = new_dest_start + new_range;
            return Some(MapRange {
                source: Some(new_source_start..new_source_end),
                destination: Some(new_dest_start..new_dest_end),
            });
        }
        None
    }
}

impl InstructionMap {
    fn new() -> InstructionMap {
        InstructionMap { maps: Vec::new() }
    }
    fn get(&self, value: &u64) -> u64 {
        // Identify the relevant map, then perform the lookup. If there is no map, then return the value unchanged
        for map in &self.maps {
            if let Some(result) = map.get(value) {
                return result;
            }
        }
        *value
    }

    fn is_empty(&self) -> bool {
        self.maps.is_empty()
    }
    fn segment_maps(&self, initial_map: &InstructionMap) -> InstructionMap {
        let mut new_maps = Vec::new();
        for map in &self.maps {
            for other_map in &initial_map.maps {
                if let Some(segment_map) = map.split_source_by_other_dest(other_map.clone()) {
                    new_maps.push(segment_map);
                }
            }
        }
        InstructionMap { maps: new_maps }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<InstructionMap>,
}

impl Almanac {
    fn seed_ranges(&self) -> InstructionMap {
        // The values in seeds are actually ranges: each pair represents a starting point
        // and the number of seeds in that range. We need to convert this into a list of
        // all of the seed locations. For example:
        // seeds: 79 14 55 13
        // Should return (79, 79+14), (55, 55+13)
        let mut seed_locations = Vec::new();

        for i in 0..self.seeds.len() / 2 {
            let start = self.seeds[i * 2];
            let count = self.seeds[i * 2 + 1];
            let range = MapRange {
                source: None,
                destination: Some(start..start + count),
            };
            seed_locations.push(range);
        }
        InstructionMap {
            maps: seed_locations,
        }
    }
    fn revise_maps(&self) -> Almanac {
        let mut new_maps = Vec::new();
        let mut other_map = self.seed_ranges();
        for map in &self.maps {
            let split_maps = map.segment_maps(&other_map);
            // dbg!(&split_maps, map);
            new_maps.push(split_maps.clone());
            other_map = split_maps;
        }
        Almanac {
            seeds: self.seeds.clone(),
            maps: new_maps,
        }
    }
}

fn parse(input: &str) -> Almanac {
    // Seeds is the first line, and looks like this
    // seeds: 79 14 55 13
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    // The Maps are the remaining lines, and look like this:
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48

    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15

    // fertilizer-to-water map:
    // 49 53 8
    // 0 11 42
    // 42 0 7
    // 57 7 4

    let mut maps = Vec::new();
    // We have an arbitrary number of maps we need to parse.
    // We need to identify when a new map is starting, then capture information on the following
    // lines until the next map starts.
    let mut lines = input.lines().skip(1);
    let mut map_name = String::new();
    let mut map = InstructionMap::new();
    let map_name_regex = Regex::new(r"^(.*) map:$").unwrap();
    while let Some(line) = lines.next() {
        if let Some(captures) = map_name_regex.captures(line) {
            // We've found a new map, so we need to save the current one and start a new one.
            if !map.is_empty() {
                maps.push(map.clone());
                map = InstructionMap::new();
            }
            map_name = captures[1].to_string();
        } else {
            // We're in the middle of a map, so we need to parse the line into a range
            // We parse a line as three numbers: destination, source, range
            // Then we need to update the map with with lookups for source+range to destination+range
            // Also check if the line is blank, then proceed
            if line.is_empty() {
                continue;
            }
            let mut numbers = line.split_whitespace();
            let destination_start = numbers.next().unwrap().parse().unwrap();
            let source_start = numbers.next().unwrap().parse().unwrap();
            let range: u64 = numbers.next().unwrap().parse().unwrap();
            let destination = destination_start..destination_start + range;
            let source = source_start..source_start + range;
            let map_range = MapRange {
                source: Some(source),
                destination: Some(destination),
            };

            map.maps.push(map_range);
        }
        // Save the current map if we've terminated the loop
    }
    if !map.is_empty() {
        maps.push(map.clone());
    }
    Almanac { seeds, maps }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = parse(input);
    // println!("{:?}", almanac);
    let mut seed_locations = Vec::new();
    for seed in almanac.seeds {
        // println!("Seed: {}", seed);
        // Run each seed through all of the maps available within the almanac. This means look it up in
        // each map in order, and if there is no mapping for the seed, then the number remains the same.
        let mut value = seed;
        for map in &almanac.maps {
            value = map.get(&value);
            // println!("Value: {}", value);
        }
        // println!("Result: {}", value);
        seed_locations.push(value);
    }
    // Return the minimum value of seed locations
    Some(*seed_locations.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = parse(input);
    // println!("{:?}", almanac);
    let seed_ranges = almanac.seed_ranges();
    let updated_almanac = almanac.revise_maps();

    let mut seed_locations = Vec::new();
    for seed_range in &seed_ranges.maps {
        for range in &seed_range.destination {
            for seed in range.start..range.end {
                // println!("Seed: {}", seed);
                let mut value = seed;
                let maps = &almanac.maps;
                // println!("{:?}", maps);
                for map in maps {
                    value = map.get(&value);
                    // println!("Value: {}", value);
                    // println!("Result: {}", value);
                }
                seed_locations.push(value);
            }
        }
    }
    // Return the minimum value of seed locations
    Some(*seed_locations.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
