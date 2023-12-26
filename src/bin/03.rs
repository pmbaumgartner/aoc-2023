use std::str::FromStr;

advent_of_code::solution!(3);

#[derive(Debug)]
enum Element {
    Number(u32),
    Symbol(char),
    Empty,
}
#[derive(Debug)]
struct ElementPosition {
    x: usize,
    y: usize,
    element: Element,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Element>>,
}

impl Grid {
    fn get_full_number(&self, x: usize, y: usize) -> Option<ElementPosition> {
        // If there is a single number at a position, we need to check all the way to the left
        // and right to see if there are adjacent numbers. If that's true, we return the full
        // number that's the string concatenation of all these digits. We also want to return where
        // the number starts, so we return the x coordinate of the leftmost digit.
        if let Element::Number(n) = self.grid[y][x] {
            let mut full_number = n.to_string();
            let mut x_left = x;
            let mut x_right = x;
            while x_left > 0 {
                x_left -= 1;
                if let Element::Number(n) = self.grid[y][x_left] {
                    full_number = format!("{}{}", n, full_number);
                } else {
                    break;
                }
            }
            while x_right < self.grid[y].len() - 1 {
                x_right += 1;
                if let Element::Number(n) = self.grid[y][x_right] {
                    full_number = format!("{}{}", full_number, n);
                } else {
                    break;
                }
            }
            return Some(ElementPosition {
                x: x_left,
                y,
                element: Element::Number(full_number.parse().unwrap()),
            });
        } else {
            return None;
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        let x_len = self.grid[y].len();
        let y_len = self.grid.len();
        for &dx in [-1, 0, 1].iter() {
            for &dy in [-1, 0, 1].iter() {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < x_len as isize && ny >= 0 && ny < y_len as isize {
                    neighbors.push((nx as usize, ny as usize));
                }
            }
        }

        neighbors
    }

    fn get_neighbors_full_digits(&self, x: usize, y: usize) -> Vec<ElementPosition> {
        // Return the full numbers of all the neighbors of a given position.
        // Don't repeat any, use the x position returned by get_full_number
        // to ensure this. For example, a 3 digit number directly above a position
        // should only appear once in the returned vector.
        let mut neighbors = Vec::new();
        for (x, y) in self.get_neighbors(x, y) {
            if let Some(n) = self.get_full_number(x, y) {
                neighbors.push(n);
            }
        }
        // deduplicate neighbors by x, y coordinates
        neighbors.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
        neighbors.dedup_by(|a, b| a.x == b.x && a.y == b.y);

        neighbors
    }

    fn get_symbol_positions(&self, symbol: Option<char>) -> Vec<(usize, usize)> {
        // Return the coordinates of all the positions that contain a given symbol.
        // If no symbol is given, return the coordinates of all the positions that contain
        // any symbol.

        let mut positions = Vec::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                if let Element::Symbol(s) = element {
                    if symbol.is_none() || symbol == Some(*s) {
                        positions.push((x, y));
                    }
                }
            }
        }
        positions
    }

    fn get_full_numbers_neighboring_symbol(&self, symbol: Option<char>) -> Vec<ElementPosition> {
        // Return the full numbers that are adjacent to a symbol.
        let mut full_numbers = Vec::new();
        for (x, y) in self.get_symbol_positions(symbol) {
            for (x, y) in self.get_neighbors(x, y) {
                if let Some(n) = self.get_full_number(x, y) {
                    full_numbers.push(n);
                }
            }
        }
        full_numbers
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse the input Grid. Elements can be either numbers or symbols.
        // A period ('.') represents an empty space, do not parse it as a symbol.

        let mut grid = Vec::new();

        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                if c.is_digit(10) {
                    row.push(Element::Number(c.to_digit(10).unwrap()));
                } else if c == '.' {
                    row.push(Element::Empty);
                } else {
                    row.push(Element::Symbol(c));
                }
            }
            grid.push(row);
        }

        Ok(Grid { grid })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input).unwrap();
    let symbols = grid.get_symbol_positions(None);
    let mut full_numbers = Vec::new();
    for (x, y) in symbols {
        full_numbers.extend(grid.get_full_numbers_neighboring_symbol(None));
    }

    // Deduplicate full numbers
    // Sort by both x,y coordinates
    full_numbers.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

    // Deduplicate by coordinates
    full_numbers.dedup_by(|a, b| a.x == b.x && a.y == b.y);
    dbg!(&full_numbers);

    // Sum full numbers
    let total = full_numbers
        .iter()
        .map(|n| {
            if let Element::Number(n) = n.element {
                n
            } else {
                0
            }
        })
        .sum();

    // 329312 is too low
    Some(total)
}

// ...738..
// ......*.
// ...231..
pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input).unwrap();
    let symbols = grid.get_symbol_positions(Some('*'));
    // println!("{:?}", symbols);
    // Get full neighboring numbers of all the stars
    // Keep only the values where the number of neighbors is 2
    let mut gear_ratios = Vec::new();
    for (x, y) in symbols {
        let neighbors = grid.get_neighbors_full_digits(x, y);
        if (x, y) == (6, 17) {
            println!("{:?}", neighbors);
        }
        if neighbors.len() != 2 {
            continue;
        } else {
            // println!("{:?}", neighbors);
            // multiply the two numbers together
            let el0 = &neighbors[0].element;
            let el1 = &neighbors[1].element;
            if let (Element::Number(n0), Element::Number(n1)) = (el0, el1) {
                gear_ratios.push(n0 * n1);
            }
        }
    }

    // Sum gear ratios
    let total = gear_ratios.iter().sum();
    // 67506297 too low
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
