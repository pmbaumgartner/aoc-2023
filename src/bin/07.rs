advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    face: char,
}

impl Card {
    fn new(face: char) -> Self {
        Self { face }
    }
    fn value(&self) -> u32 {
        match self.face {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => self.face.to_digit(10).unwrap(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32) -> Self {
        Self { cards, bid }
    }
    fn score(&self) -> HandType {
        // There are always 5 cards
        let mut cards = self.cards.clone();
        cards.sort_by(|a, b| b.value().cmp(&a.value()));
        let mut counts = std::collections::HashMap::new();
        for card in cards.iter() {
            let count = counts.entry(card.value()).or_insert(0);
            *count += 1;
        }
        let mut counts: Vec<(u32, u32)> = counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        let counts: Vec<u32> = counts.into_iter().map(|(_, count)| count).collect();
        match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [2, 1, 1, 1] => HandType::OnePair,
            [2, 2, 1] => HandType::TwoPair,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [3, 2] => HandType::FullHouse,
            [4, 1] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => panic!("Invalid hand"),
        }
    }

    fn substitute_card(&self, original: Card, replacement: Card) -> Hand {
        // Return a new hand substituting the original card with the replacement card, maintaining
        // the original order of the cards
        let mut cards = self.cards.clone();
        let index = cards.iter().position(|c| *c == original).unwrap();
        cards[index] = replacement;
        Hand::new(cards, self.bid)
    }
    fn score_with_jokers(&self) -> HandType {
        // Check if all cards are jokers, if so, return Five of a Kind
        let mut all_jokers = true;
        for card in self.cards.iter() {
            if card.face != 'J' {
                all_jokers = false;
                break;
            }
        }
        if all_jokers {
            return HandType::FiveOfAKind;
        }
        // if there are no jokers, return the score
        if !self.cards.iter().any(|card| card.face == 'J') {
            return self.score();
        }
        let mut hand_without_jokers = self.clone();
        // Remove all jokers from the hand
        hand_without_jokers.cards.retain(|card| card.face != 'J');
        // Identify most common card
        let most_common = most_common_card(&hand_without_jokers);
        // Form a new hand with 5 cards, replacing cards in hand_without_jokers with the most common card
        let mut new_hand = self.substitute_card(Card::new('J'), most_common);
        // Score the new hand
        new_hand.score()
    }
}

fn most_common_card(hand: &Hand) -> Card {
    // Identify most common card and return it.
    // If it's a tie, return the card with the highest value.

    // Check if all cards are 'J', if so, return 'A'
    let mut all_jokers = true;
    for card in hand.cards.iter() {
        if card.face != 'J' {
            all_jokers = false;
            break;
        }
    }
    if all_jokers {
        return Card::new('A');
    }
    let mut counts = std::collections::HashMap::new();
    for card in hand.cards.iter() {
        let count = counts.entry(card.value()).or_insert(0);
        *count += 1;
    }
    let mut counts: Vec<(u32, u32)> = counts.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    let mut counts: Vec<u32> = counts.into_iter().map(|(card, _)| card).collect();
    counts.sort_by(|a, b| b.cmp(&a));
    let most_common = counts[0];
    match most_common {
        11 => Card::new('J'),
        12 => Card::new('Q'),
        13 => Card::new('K'),
        14 => Card::new('A'),
        _ => Card::new(most_common.to_string().chars().next().unwrap()),
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards: Vec<String> = self
            .cards
            .clone()
            .into_iter()
            .map(|card| card.face.to_string())
            .collect();
        // display the card string, a space, and the bid
        write!(f, "{} {}", cards.join(""), self.bid)
    }
}

// Implement the PartialOrd for Hand based on the score method
// If the score is the same, then pairwise compare the cards in the original order from the hands.
// While the cards are equal, continue to the next card. If all cards are equal, then the hands are
// equal.
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_score = self.score_with_jokers();
        let other_score = other.score_with_jokers();
        match self_score != other_score {
            true => return self_score.partial_cmp(&other_score),
            false => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    let self_value = self_card.value();
                    let other_value = other_card.value();
                    println!("{}", self);
                    println!("{}", other);
                    println!("{} {}", self_value, other_value);
                    if self_value > other_value {
                        println!("self is greater");
                        return Some(std::cmp::Ordering::Greater);
                    } else if self_value < other_value {
                        println!("other is greater");
                        return Some(std::cmp::Ordering::Less);
                    }
                }
                panic!("Hands are equal");
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}

fn parse(input: &str) -> Vec<Hand> {
    // Parse input into hands. Input is a list of the 5 cards, a space, then the bid
    // 32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483
    let mut hands = Vec::new();
    for line in input.lines() {
        let mut cards = Vec::new();
        let split = line.split(' ').collect::<Vec<_>>();
        let bid = split[1].parse::<u32>().unwrap();
        for card in split[0].chars() {
            cards.push(Card::new(card));
        }
        hands.push(Hand::new(cards, bid));
    }
    hands
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands = parse(input);
    // Sort and rank all the hands by comparing them to each other
    // There should be no ties
    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
    sorted_hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // The total value of all hands is the bid * the rank, where the lowest rank is 1
    println!("HANDS");
    for hand in sorted_hands.iter() {
        println!("{}", hand);
    }
    // reverse the hands
    sorted_hands.reverse();
    let mut total = 0;
    for (rank, hand) in sorted_hands.iter().enumerate() {
        total += hand.bid * (rank as u32 + 1);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands = parse(input);
    // Sort and rank all the hands by comparing them to each other
    // There should be no ties
    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
    sorted_hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // The total value of all hands is the bid * the rank, where the lowest rank is 1
    println!("HANDS");
    for hand in sorted_hands.iter() {
        println!("{}", hand);
    }
    // reverse the hands
    sorted_hands.reverse();
    let mut total = 0;
    for (rank, hand) in sorted_hands.iter().enumerate() {
        total += hand.bid * (rank as u32 + 1);
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        // 249735396 too low
        // 249557138 too low
        // 248757094 too low
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_hand_comparison() {
        let hand1 = Hand {
            cards: vec![
                Card::new('7'),
                Card::new('7'),
                Card::new('8'),
                Card::new('8'),
                Card::new('8'),
            ],
            bid: 0,
        };
        let hand2 = Hand {
            cards: vec![
                Card::new('7'),
                Card::new('7'),
                Card::new('7'),
                Card::new('8'),
                Card::new('8'),
            ],
            bid: 0,
        };

        assert_eq!(hand1.partial_cmp(&hand2), Some(std::cmp::Ordering::Greater));
    }
}
