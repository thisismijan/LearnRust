use std::collections::HashMap;

use nom::character::complete::space0;
use nom::multi::{many1, separated_list0};
use nom::{branch::alt, bytes::complete::tag, sequence::separated_pair, IResult, Parser};

fn main() {
    let input = include_str!("input1.txt");
    let result = process(input);
    println!("Result: {}", result);
}

fn process(input: &str) -> u64 {
    let (_, data) = parse_input(input).expect("data");
    let springs = data.iter().map(move |spring| {
        let mut records: Vec<Record> = Vec::new();
        let mut sizes = Vec::new();
        for _ in 0..4 {
            records.extend(spring.records.iter().chain(&[Record::Unknown]));
        }
        records.extend(spring.records.iter());
        for _ in 0..5 {
            sizes.extend(spring.sizes.iter());
        }
        Springs {records, sizes}
    }).collect::<Vec<_>>();
    springs.iter().map(score).sum::<u64>()
}

fn score(springs: &Springs) -> u64 {
    let mut cache = HashMap::new();
    compute_score(&springs.records, &springs.sizes, &mut cache)
}

fn compute_score(records: &[Record], sizes: &[u64], cache: &mut Cache ) -> u64 {
    if let Some(num) = cache.get(&(records.to_vec(), sizes.to_vec())) {
     return *num;
    }
    if sizes.is_empty() {
        return !(records.contains(&Record::Damaged)) as u64;
    }

    let min_remaining = sizes.iter().sum::<u64>() as usize + sizes.len() - 1;

    if records.len() < min_remaining {
        return 0;
    }

    let score = match records[0] {
        Record::Unknown => {
            compute_score(&records[1..], sizes, cache) + compute_damaged(records, sizes, cache)

        },
        Record::Operational => {
            compute_score(&records[1..], sizes, cache)

        },
        Record::Damaged => {
            compute_damaged(records, sizes, cache)

        },
    };
    cache.insert((records.to_vec(),sizes.to_vec()), score);
    score
}

type Cache = HashMap<(Vec<Record>, Vec<u64>), u64>;

fn compute_damaged(records: &[Record], sizes: &[u64], cache: &mut Cache ) -> u64 {
    if records.len() < sizes[0] as usize
        || records[0..sizes[0] as usize].contains(&Record::Operational)
    {
        return 0;
    }

    if records.len() == sizes[0] as usize {
        return (sizes.len() == 1) as u64;
    }

    if records[sizes[0] as usize] == Record::Damaged {
        return 0;
    }

    compute_score(&records[sizes[0] as usize + 1..], &sizes[1..], cache)
}

#[derive(Debug, PartialEq)]
struct Springs {
    records: Vec<Record>,
    sizes: Vec<u64>,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Springs>> {
    let a = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect::<Vec<_>>();
    Ok(("", a))
}

fn parse_line(input: &str) -> IResult<&str, Springs> {
    separated_pair(parse_records, space0, parse_numbers)(input)
        .map(|(input, (records, sizes))| (input, Springs { records, sizes }))
}

fn parse_records(input: &str) -> IResult<&str, Vec<Record>> {
    many1(alt((
        tag(".").map(|_| Record::Operational),
        tag("#").map(|_| Record::Damaged),
        tag("?").map(|_| Record::Unknown),
    )))(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(","), nom::character::complete::u64)(input)
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Record {
    Unknown,
    Operational,
    Damaged,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = process(input);
        assert_eq!(525152, result);
    }
}
