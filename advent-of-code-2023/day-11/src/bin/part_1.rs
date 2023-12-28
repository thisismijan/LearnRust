use std::iter::from_fn;

use itertools::Itertools;

fn main() {
    let input = include_str!("input1.txt");
    let result = process(input);
    println!("Result: {}", result);
}

fn process(input: &str) -> i32 {
    let points = parse_input(input);
    points
        .iter()
        .combinations(2)
        .map(|points| {

            (points[1].0 as i32 - points[0].0 as i32).abs()
                + (points[1].1 as i32 - points[0].1 as i32).abs()
        })
        .sum::<i32>()
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let empty_rows = input
        .lines()
        .enumerate()
        .filter_map(|(y, row)| row.chars().all(|c| c == '.').then_some(y))
        .collect::<Vec<usize>>();
    let mut columns = input.lines().map(|line| line.chars()).collect::<Vec<_>>();
    let empty_columns = from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns {
            match iter.next() {
                Some(item) => {
                    items.push(item);
                }
                None => return None,
            }
        }
        Some(items)
    })
    .enumerate()
    .filter_map(|(x, column)| column.iter().all(|c| c == &'.').then_some(x))
    .collect::<Vec<usize>>();
    let mut result = Vec::new();

    let mut y_inc = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        let mut yinc = false;
        if empty_rows.contains(&y) & !yinc {
            y_inc += 1;
            yinc = true;
        }
        let mut x_inc = 0;
        line.chars().enumerate().for_each(|(x, c)| {

            let mut xinc = false;
            if empty_columns.contains(&x) & !xinc {
                x_inc += 1;
                xinc = true;
            }
            if c.eq_ignore_ascii_case(&'#') {
                result.push((x + x_inc, y + y_inc));
            }
        })
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = process(input);
        assert_eq!(374, result);
    }
}
