use std::{collections::BTreeMap, ops::Deref};

use nom::{
    character::complete::{self, line_ending, space1, alphanumeric1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let result = process(input);
    println!("Result: {}", result);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
enum HandType {
    HighC,
    OneP,
    TwoP,
    ThreeOK,
    FullH,
    FourOK,
    FiveOK,
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: u64,
    hand_type: HandType,
}

impl<'a> Hand<'a> {
    fn new(cards: &'a str, bid: u64) -> Hand<'a> {
        Hand { cards, bid, hand_type: Self::get_hand_type(cards) }
    }

    fn get_hand_type(cards: &str) -> HandType {
        let counts = cards.chars().fold(BTreeMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let updated_counts = if let Some(jokers) = counts.get(&'J') {
            if *jokers == 5 {
                counts.clone()
            } else {
                let mut updated_counts = counts.clone();
                updated_counts.remove(&'J');
                let entry = updated_counts.iter().max_by_key(|entry| entry.1).expect("entry");
                updated_counts.insert(*entry.0, entry.1 + jokers);
                updated_counts
            }
        } else {
            counts
        };

        let counts_str = updated_counts.into_iter().fold(String::new(), |mut acc, (_,i)| {
            acc.push_str(&i.to_string());
            acc
        });
        match counts_str.deref() {
            "5" => HandType::FiveOK,
            "14" | "41" => HandType::FourOK,
            "32" | "23" => HandType::FullH,
            "113" | "131" | "311" => HandType::ThreeOK,
            "122" | "212" | "221" => HandType::TwoP,
            "1112" | "1121" | "1211" | "2111" => HandType::OneP,
            "11111" => HandType::HighC,
            s => panic!("? {}", s),
        }
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid && self.hand_type == other.hand_type
    }
}

impl<'a> Eq for Hand<'a> {}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cards.partial_cmp(&other.cards)
            .or_else(|| self.bid.partial_cmp(&other.bid))
            .or_else(|| self.hand_type.partial_cmp(&other.hand_type))
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type.cmp(&other.hand_type)
            .then_with(|| {
                self.cards.chars().zip(other.cards.chars())
                    .find_map(|(a, b)| Some(Card::from(a).cmp(&Card::from(b)).then(std::cmp::Ordering::Equal)))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        use Card::*;
        match value {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Joker,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => panic!("? {}", value),
        }
    }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, bids)) = separated_pair(alphanumeric1, space1, complete::u64)(input)?;
    Ok((input, Hand::new(cards, bids)))
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_hand)(input)
}

fn process(input: &str) -> usize {
    let (_, mut hands) = parse_hands(input).expect("hand and bids");
    hands.sort_by(|a, b| a.cmp(b));
    hands.iter().enumerate().map(|(index, hand)| (index + 1) * hand.bid as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = process(input);
        assert_eq!(5905, result);
    }
}
