advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    id: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn count_winning_numbers(&self) -> u32 {
        // Return the numbers that are winners
        self.winners
            .iter()
            .filter(|&x| self.numbers.contains(x))
            .count() as u32
    }
    fn value(&self) -> u32 {
        // Identify how many winners are in the numbers
        // and return 2^winners unless there are no winners
        // return 0
        let winners = self
            .winners
            .iter()
            .filter(|&x| self.numbers.contains(x))
            .count();
        if winners == 0 {
            return 0;
        }
        2u32.pow(winners as u32 - 1)
    }
}

fn parse_input(input: &str) -> Vec<Card> {
    // Parse a line into a Card with ID, Winners, Numbers
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

    let mut cards = Vec::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let id = parts[1].trim_end_matches(':').parse::<u32>().unwrap();
        let pipe_index = parts.iter().position(|&x| x == "|").unwrap();
        let winners = parts[2..pipe_index]
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let numbers = parts[pipe_index + 1..]
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        cards.push(Card {
            id,
            winners,
            numbers,
        });
    }
    cards
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse_input(input);
    let mut total = 0;
    for card in cards {
        total += card.value();
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse_input(input);
    let mut repeats: Vec<u32> = vec![1; cards.len()];
    let mut total = 0;
    for (i, card) in cards.iter().enumerate() {
        let winners = card.count_winning_numbers();
        // // print card, winners, repeats
        // println!(
        //     "Card: {}, Winners: {}, Repeats: {:?}",
        //     card.id, winners, repeats
        // );
        for _ in 1..repeats[i] + 1 {
            total += 1;
            for j in i + 1..i + 1 + winners as usize {
                repeats[j] += 1;
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
