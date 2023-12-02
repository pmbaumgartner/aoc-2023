use regex::Regex;
use std::str::FromStr;
advent_of_code::solution!(2);

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
    fn check_possible(&self, criteria: &CubeSet) -> bool {
        // Check if this pull is possible given the criteria
        // All colors must be less than or equal to the criteria
        self.red <= criteria.red && self.green <= criteria.green && self.blue <= criteria.blue
    }
    fn power(&self) -> u32 {
        // Calculate the power of this pull
        // The power is the product of all colors
        self.red * self.green * self.blue
    }
}

impl FromStr for CubeSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse a string into a CubePull, where a pull is:
        // 3 blue, 4 red == CubePull { red: 4, green: 0, blue: 3 }
        // 3 green, 4 blue, 1 red == CubePull { red: 1, green: 3, blue: 4 }

        let re = Regex::new(r"(\d+) (\w+)").unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cap in re.captures_iter(s) {
            let count = cap[1].parse::<u32>().unwrap();
            match &cap[2] {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => return Err(format!("Invalid color: {}", &cap[2])),
            }
        }

        Ok(Self::new(red, green, blue))
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<CubeSet>,
}

impl Game {
    fn new(id: u32, cubes: Vec<CubeSet>) -> Self {
        Self { id, cubes }
    }
    fn min_cubes(&self) -> CubeSet {
        // We must iterate through each game and find the minimum number of cubes that
        // would make that game possible, which means finding the max for each color across
        // all games
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for pull in &self.cubes {
            if pull.red > red {
                red = pull.red;
            }
            if pull.green > green {
                green = pull.green;
            }
            if pull.blue > blue {
                blue = pull.blue;
            }
        }

        CubeSet::new(red, green, blue)
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse a string into a Game, where a game is comprised of a list of CubePulls:
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

        let re = Regex::new(r"Game (\d+): (.+)").unwrap();
        let cap = re.captures(s).unwrap();
        let id = cap[1].parse::<u32>().unwrap();
        let cubes = cap[2]
            .split("; ")
            .map(|pull| pull.parse::<CubeSet>().unwrap())
            .collect::<Vec<_>>();

        Ok(Self::new(id, cubes))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let criteria = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Game>().unwrap())
        .collect::<Vec<_>>();
    // Check how many games can meet our criteria
    // If one pull fails, the game fails
    // keep track of the IDs of games that match
    // println!("{:?}", games);
    let mut matches = Vec::new();
    // Named Loops, so good
    'gloop: for game in games {
        for pull in game.cubes {
            if !pull.check_possible(&criteria) {
                continue 'gloop;
            }
        }
        matches.push(game.id);
    }
    // sum games that match
    let sum = matches.iter().sum::<u32>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let criteria = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Game>().unwrap())
        .collect::<Vec<_>>();
    // Find the minimum number of cubes that would make each game possible
    // Calculate the power of that CubeSet for each game and sum them
    let mut sum = 0;
    for game in games {
        let min_cubes = game.min_cubes();
        sum += min_cubes.power();
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
