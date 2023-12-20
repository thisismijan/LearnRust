use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{self, alpha1, alphanumeric1, line_ending},
    combinator::eof,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let result = process(input);
    println!("Result: {}", result);
}

fn process(input: &str) -> u32 {
    let (_, (directions, map)) = parse_input(input).expect("input");
    let mut key = *map.iter().next().unwrap().0;
    let mut count = 0;
    let mut iter = directions.chars().into_iter();
    loop {
    match iter.next(){
        Some(direction) => {
            if key == "ZZZ" {
                return count;
            }
            match direction {
                'L' => {
                    key = map.get(key).unwrap().0;
                    count+=1;
                },
                'R' => {
                    key = map.get(key).unwrap().1;
                    count +=1;
                },
                x => panic!("cant be {}", x)
            }
        },
        None => iter = directions.chars().into_iter(),
    }
}
}

fn parse_input(input: &str) -> IResult<&str, (&str, BTreeMap<&str, (&str, &str)>)> {
    separated_pair(alphanumeric1, line_ending, parse_maps)(input)
}

fn parse_maps(input: &str) -> IResult<&str, BTreeMap<&str, (&str, &str)>> {
    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alpha1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(alpha1, tag(", "), alpha1),
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
        let input = "RL
AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let result = process(input);
        assert_eq!(2, result);
    }

    #[test]
    fn test_process1() {
        let input = "LLR
AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let result = process(input);
        assert_eq!(6, result);
    }
}
