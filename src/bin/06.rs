use std::iter::zip;

advent_of_code::solution!(6);

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn calculate_possibilities(&self) -> Vec<Race> {
        // Time:      7  15   30
        // Distance:  9  40  200

        let mut race_times = Vec::new();
        for time in 1..self.time {
            let rate = time;
            let time_to_travel = self.time - time;
            let distance_traveled = rate * time_to_travel;
            race_times.push(Race {
                time: time,
                distance: distance_traveled,
            })
        }
        race_times
    }
    fn calculate_winners(&self) -> Vec<Race> {
        let possibilities = self.calculate_possibilities();
        let mut winners = Vec::new();
        for possibility in possibilities {
            if possibility.distance > self.distance {
                winners.push(possibility);
            }
        }
        winners
    }
}

fn parser(input: &str) -> Vec<Race> {
    // Parse the input file of races, which contain a distance and a time
    // e.g. an input file of 3 races:
    // Time:      7  15   30
    // Distance:  9  40  200
    let input_lines = input.lines().collect::<Vec<_>>();
    // The first value is a string of the type of number, e.g. "Time:"
    let times = input_lines[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let distances = input_lines[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut races = Vec::new();

    for (time, distance) in zip(times, distances) {
        races.push(Race { time, distance })
    }
    races
}

fn concat_parser(input: &str) -> Race {
    // Parse the input file of a single race, which contains a time and distance
    // that have been split over several columns
    // e.g. an input file of 1 race
    // Time:      7  15   30
    // Distance:  9  40  200
    // is Race { time: 71530, distance: 940200 }
    let input_lines = input.lines().collect::<Vec<_>>();
    let time = input_lines[0]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = input_lines[1]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    Race { time, distance }
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = parser(input);
    let mut winning_possibilities = Vec::new();
    for race in races {
        let winners = race.calculate_winners();
        winning_possibilities.push(winners.len())
    }
    // Multiply all winning possibilities together
    let mut total_possibilities = 1;
    for possibility in winning_possibilities {
        total_possibilities *= possibility;
    }
    Some(total_possibilities as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = concat_parser(input);
    let n_winners = race.calculate_winners().len();
    Some(n_winners as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
