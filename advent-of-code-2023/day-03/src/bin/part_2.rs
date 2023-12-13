fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize{
    let part_numbers = parse_numbers(input);
    let stars = parse_stars(input);
    dbg!(&stars);
    let nums: Vec<(PartNumber,PartNumber)> = find_adjacent_part_numbers(stars, &part_numbers);
    dbg!(&nums);
    nums.iter().map(|(i,j)| i.num * j.num).sum::<usize>()
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

fn parse_stars(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row_index, line)| {
            line.chars().enumerate().filter_map(move |(col_index, char)| {
                if char == '*' {
                    Some((row_index, col_index))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn find_adjacent_part_numbers(stars: Vec::<(usize, usize)>, part_numbers: &[PartNumber]) -> Vec<(PartNumber, PartNumber)> {
    let mut result = Vec::new();

    for star_coords in stars {
        let adjacent_part_numbers: Vec<PartNumber> = part_numbers
            .iter()
            .filter(|part_number| {
                part_number
                    .coords
                    .iter()
                    .any(|&(row, col)| {
                        (row as isize - star_coords.0 as isize).abs() <= 1
                            && (col as isize - star_coords.1 as isize).abs() <= 1
                    })
            })
            .cloned()
            .collect();

        if adjacent_part_numbers.len() == 2 {
            result.push((adjacent_part_numbers[0].clone(), adjacent_part_numbers[1].clone()));
        }
    }

    result
}


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
        assert_eq!(467835, result);
    }
}
