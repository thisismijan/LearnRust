
fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize{
    let part_numbers = parse_numbers(input);
    let valid_part_numbers = find_numbers_with_adjacent_symbols(&part_numbers, input);
    let n: Vec<usize> = valid_part_numbers.iter().map(|num| num.num).collect();
    n.iter().sum::<usize>()
}

#[derive(Debug, Clone)]
struct PartNumber {
    num: usize,
    coords: Vec<(usize, usize)>,
}

fn parse_numbers(input: &str) -> Vec<PartNumber> {
    let mut part_numbers = Vec::new();
    let mut current_number = String::new();
    let mut current_coords = Vec::new();
    let mut current_row = 0;
    let mut current_col = 0;

    for c in input.chars() {
        if c.is_digit(10) {
            current_number.push(c);
            current_coords.push((current_row, current_col));
        } else if !current_number.is_empty() {
            if let Ok(parsed_number) = current_number.parse::<usize>() {
                let part_number = PartNumber {
                    num: parsed_number,
                    coords: current_coords.clone(),
                };
                part_numbers.push(part_number);
            }
            current_number.clear();
            current_coords.clear();
        }

        if c == '\n' {
            current_row += 1;
            current_col = 0;
        } else {
            current_col += 1;
        }
    }

    // Check for the last number if the input ends with a digit
    if let Ok(parsed_number) = current_number.parse::<usize>() {
        let part_number = PartNumber {
            num: parsed_number,
            coords: current_coords.clone(),
        };
        part_numbers.push(part_number);
    }

    part_numbers
}

fn find_numbers_with_adjacent_symbols(
    part_numbers: &[PartNumber],
    input: &str,
) -> Vec<PartNumber> {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    part_numbers
        .iter()
        .filter(|part_number| {
            part_number
                .coords
                .iter()
                .any(|&(row, col)| has_adjacent_symbol(row, col, &matrix))
        })
        .cloned()
        .collect()
}

fn has_adjacent_symbol(row: usize, col: usize, matrix: &[Vec<char>]) -> bool {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1), (1, 0),  (1, 1),
    ];

    directions.iter().any(|&(dr, dc)| {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        new_row >= 0
            && new_col >= 0
            && new_row < matrix.len() as isize
            && new_col < matrix[0].len() as isize
            && !matrix[new_row as usize][new_col as usize].is_digit(10)
            && matrix[new_row as usize][new_col as usize] != '.'
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;


    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = process(input);
        assert_eq!(4361, result);
    }
}

