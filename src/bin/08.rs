use regex::Regex;
use std::collections::BTreeMap;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Movement {
    Left,
    Right,
}

type Instructions = Vec<Movement>;

fn parse(input: &str) -> (Instructions, BTreeMap<Movement, BTreeMap<&str, &str>>) {
    // Parse the input into instructions and a mappling of left and right movements.
    // Example:
    // LLR
    //
    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    // ZZZ = (ZZZ, ZZZ)
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Movement::Left,
            'R' => Movement::Right,
            _ => panic!("Invalid direction"),
        })
        .collect::<Instructions>();
    let mut movement_map: BTreeMap<Movement, BTreeMap<&str, &str>> = BTreeMap::new();
    movement_map.insert(Movement::Left, BTreeMap::new());
    movement_map.insert(Movement::Right, BTreeMap::new());
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for line in lines {
        // Skip if empty
        if line.is_empty() {
            continue;
        }
        let caps = re.captures(line).unwrap();
        let source = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        let right = caps.get(3).unwrap().as_str();
        movement_map
            .get_mut(&Movement::Left)
            .unwrap()
            .insert(source, left);
        movement_map
            .get_mut(&Movement::Right)
            .unwrap()
            .insert(source, right);
    }
    (instructions, movement_map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, movement_map) = parse(input);
    println!("{:?}", instructions);
    println!("{:?}", movement_map);
    let start = "AAA";
    let mut current = start;
    // cycle indefinitely through the instructions until we reach node "ZZZ"
    // Count the number of moves it takes to get there
    // At each node, follow the Movement of the instruction.
    let mut moves = 0;
    // cycle indefinitely through the instructions until we reach node "ZZZ"
    for instruction in instructions.iter().cycle() {
        // follow the movement of the instruction
        current = movement_map
            .get(instruction)
            .unwrap()
            .get(current)
            .unwrap()
            .clone();
        moves += 1;
        if current == "ZZZ" {
            break;
        }
    }

    Some(moves as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (instructions, movement_map) = parse(input);
    // println!("{:?}", instructions);
    // println!("{:?}", movement_map);
    // There are multiple start nodes we need to traverse from. Identify all nodes that end with 'A'
    // and start from there.
    let mut starts = vec![];
    for (node, _) in movement_map.get(&Movement::Left).unwrap().iter() {
        if node.ends_with('A') {
            starts.push(node);
        }
    }
    println!("{:?}", starts);

    // cycle indefinitely through the instructions until all nodes end in a 'Z'
    // Count the number of moves it takes to get there for all nodes
    // At each node, follow the Movement of the instruction.
    // cycle indefinitely through the instructions until all nodes end in a 'Z'
    let mut moves = 0;
    // cycle indefinitely through the instructions until all nodes end in a 'Z'
    for instruction in instructions.iter().cycle() {
        // follow the movement of the instruction for each node. If the node ends with a 'Z', we can remove it
        // from the list of nodes we need to traverse.
        let mut new_starts = vec![];
        for node in starts.iter() {
            let next_node = movement_map
                .get(instruction)
                .unwrap()
                .get(node.clone())
                .unwrap()
                .clone();
            if next_node.ends_with('Z') {
                continue;
            }
            new_starts.push(next_node);
        }
        starts = new_starts;
        if starts.is_empty() {
            break;
        }
        moves += 1;
    }
    Some(moves as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
