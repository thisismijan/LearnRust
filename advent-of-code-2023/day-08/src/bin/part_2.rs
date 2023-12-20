use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending},
    combinator::eof,
    multi::fold_many1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let result = process(input);
    println!("Result: {}", result);
}

fn process(input: &str) -> usize {
    let (_, (directions, map)) = parse_input(input).expect("input");
    let keys = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();
    let mut counts = Vec::new();
    for key in keys {
        let mut count = 0;
        let mut current = key;
        while !current.ends_with('Z') {
            let direction = directions.chars().nth(count % directions.len()).unwrap();
            match direction {
                'L' => {
                    current = map.get(current).unwrap().0;
                }
                'R' => {
                    current = map.get(current).unwrap().1;
                }
                x => panic!("cant be {}", x),
            }
            count += 1;
        }
        counts.push(count);
    }
    lcm_of(counts)
}


fn lcm_of(counts: Vec<usize>) -> usize {
    counts.iter().cloned().fold(1, |acc, x| lcm(acc, x))
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn parse_input(input: &str) -> IResult<&str, (&str, BTreeMap<&str, (&str, &str)>)> {
    separated_pair(alphanumeric1, line_ending, parse_maps)(input)
}

fn parse_maps(input: &str) -> IResult<&str, BTreeMap<&str, (&str, &str)>> {
    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alphanumeric1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                    complete::char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        BTreeMap::new,
        |mut acc: BTreeMap<&str, (&str, &str)>, (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;
    Ok((input, map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "LR
11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let result = process(input);
        assert_eq!(6, result);
    }
}
