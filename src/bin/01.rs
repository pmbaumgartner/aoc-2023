use std::collections::HashMap;
advent_of_code::solution!(1);

// Out input are lines of numbers and characters.
// For each line, we need to capture the first number that appears
// And the last number that appears.
// e.g.
// 1abc2 == 12
// pqr3stu8vwx == 38
// a1b2c3d4e5f == 15
// treb7uchet == 77
fn parse_digits_from_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)))
        .map(|digits| digits.collect())
        .collect()
}

fn parse_digits_from_input_part_two(input: &str) -> Vec<(u32, u32)> {
    // This needs to parse numeric digits as well as word digits from strings
    // It should return the first digit and the last digit as a vector
    // The last digit needs to search from right to left
    // e.g.
    // "two1nine" == (2, 9)
    // "abcone2threexyz" == (1, 3)
    // "fourfive6six" == (4, 6)
    // "xtwone3four" == (2, 4)
    // "4nineeightseven2" == (4, 2)
    // "three49oneightf" == (3, 8)

    fn parse_first_digit(line: &str) -> Option<u32> {
        let mut word_to_digit_map = HashMap::new();
        word_to_digit_map.insert("one", 1);
        word_to_digit_map.insert("two", 2);
        word_to_digit_map.insert("three", 3);
        word_to_digit_map.insert("four", 4);
        word_to_digit_map.insert("five", 5);
        word_to_digit_map.insert("six", 6);
        word_to_digit_map.insert("seven", 7);
        word_to_digit_map.insert("eight", 8);
        word_to_digit_map.insert("nine", 9);
        let mut word = String::new();

        for c in line.chars() {
            if c.is_digit(10) {
                return Some(c.to_digit(10).unwrap());
            } else {
                word.push(c);
                // The digit can occur in the latter half of a collected word
                // e.g. "abcone" == 1
                // So we need to check all substrings to see if a digit word is present
                for i in 0..word.len() {
                    let substr = &word[i..];
                    if let Some(digit) = word_to_digit_map.get(substr) {
                        return Some(*digit);
                    }
                }
            }
        }
        None
    }

    fn parse_last_digit(line: &str) -> Option<u32> {
        let mut word_to_digit_map = HashMap::new();
        word_to_digit_map.insert("one", 1);
        word_to_digit_map.insert("two", 2);
        word_to_digit_map.insert("three", 3);
        word_to_digit_map.insert("four", 4);
        word_to_digit_map.insert("five", 5);
        word_to_digit_map.insert("six", 6);
        word_to_digit_map.insert("seven", 7);
        word_to_digit_map.insert("eight", 8);
        word_to_digit_map.insert("nine", 9);
        let mut word = String::new();

        for c in line.chars().rev() {
            // Check if we have a word is a digit
            // Otherwise we have an edge case where we have a digit word
            // but this will return the next digit in reverse
            // e.g. "oneightf" == 8
            // We ned to check all substrings to see if a digit word is present
            // Only iterating through the forward-looking substrings
            // Will not get us what we want - for example "eightf" needs to return 8
            // but if we only check at each point words will be: f, tf, ghtf, ightf, eightf
            // And we would need to extract 8

            if !c.is_digit(10) {
                word.insert(0, c);
                // The digit can occur in the first part of the substring,
                // e.g. "eightf" == 8
                // So we need to check all substrings to see if a digit word is present
                for i in 0..word.len() {
                    let substr = &word[..i + 1];
                    if let Some(digit) = word_to_digit_map.get(substr) {
                        return Some(*digit);
                    }
                }
            } else {
                return Some(c.to_digit(10).unwrap());
            }
        }
        None
    }

    input
        .lines()
        .map(|line| {
            let first = parse_first_digit(line);
            let last = parse_last_digit(line);

            // println!("first: {:?}, last: {:?}", first, last);
            // If we have a first and last digit, return them
            if let (Some(first), Some(last)) = (first, last) {
                (first, last)
            } else {
                panic!("Could not parse digits from line: {}", line);
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let digits = parse_digits_from_input(input);
    let mut total = 0;
    for line in digits {
        // concatenate the first and last digits as a string
        // e.g. 7 and 8 becomes "78"
        let first = line[0];
        let last = line[line.len() - 1];
        let concat = format!("{}{}", first, last);
        // parse the string as a u32
        let num = concat.parse::<u32>().unwrap();
        // add to the total
        total += num;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let digits = parse_digits_from_input_part_two(input);
    // 54194 is too low
    // 54226 is too high
    let mut total = 0;
    for line in digits {
        // concatenate the first and last digits as a string
        // e.g. 7 and 8 becomes "78"
        // digits is just a pair of first and last digits
        let first = line.0;
        let last = line.1;
        let concat = format!("{}{}", first, last);
        println!("{} + {} = {}", first, last, concat);
        // parse the string as a u32
        let num = concat.parse::<u32>().unwrap();
        // add to the total
        total += num;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
