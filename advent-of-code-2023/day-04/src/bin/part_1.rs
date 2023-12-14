use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space0, space1},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, tuple, terminated},
    IResult, Parser,
};

fn main() {
    let input = include_str!("input1.txt");
    let result = process(input);
    println!("Result: {}", result);
}

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, parse_card)(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = delimited(tuple((tag("Card"), space1)), digit1, tuple((tag(":"), space1)))(input)?;
    separated_pair(parse_numbers, tuple((tag("|"), space1)), parse_numbers)
        .map(|(winning_numbers, my_numbers)| Card { winning_numbers, my_numbers })
        .parse(input)
}

fn parse_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(terminated(complete::u32, space0), HashSet::new, |mut acc, item| {
        acc.insert(item);
        acc
    })(input)
}

fn process(input: &str) -> u32 {
    let cards = match parse_cards(input) {
        Ok((_, cards)) => cards,
        Err(err) => {
            eprintln!("Error parsing input: {:?}", err);
            return 0;
        }
    };

    dbg!(&cards);

    cards.iter().map(|card| {
        let count = card.winning_numbers.intersection(&card.my_numbers).count() as u32;
        if count == 0 {
            return 0;
        }
        2u32.pow(count.saturating_sub(1))
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = process(input);
        assert_eq!(13, result);
    }
}
