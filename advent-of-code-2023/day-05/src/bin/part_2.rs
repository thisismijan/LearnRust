use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = include_str!("input1.txt");
    let result = process(input);
    println!("Result: {}", result);
}

fn process(input: &str) -> u64 {
    let (_, (seeds, seed_maps)) = parse_seed_maps(input).expect("seed maps");

    seeds
        .iter()
        .flat_map(|range| range.clone().into_iter())
        .map(|seed| {
            seed_maps.iter().fold(seed, |seed, seed_map| {
                let valid_map = seed_map
                    .maps
                    .iter()
                    .find(|(_, source)| source.contains(&seed));
                let Some((destination, source)) = valid_map else {
                    return seed;
                };
                let offset = seed - source.start;
                destination.start + offset
            })
        })
        .min()
        .expect("min location")
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<Range::<u64>>> {
    preceded(tag("seeds: "), 
    separated_list1(space1, separated_pair(complete::u64, space1, complete::u64).map(|(start, offset)| {
        start..(start + offset)
}
),)).parse(input)
}

#[derive(Debug)]
struct SeedMap {
    maps: Vec<(Range<u64>, Range<u64>)>,
}

fn parse_seed_maps(input: &str) -> IResult<&str, (Vec<Range::<u64>>, Vec<SeedMap>)> {
    let (input, seeds) = parse_seeds(input).expect("seeds");
    let (input, maps) = many1(parse_seed_map)(input).expect("seedMaps");
    Ok((input, (seeds, maps)))
}

fn parse_seed_map(input: &str) -> IResult<&str, SeedMap> {
    preceded(
        preceded(take_until("map:"), tag("map:")),
        many1(preceded(line_ending, parse_line)).map(|maps| SeedMap { maps }),
    )
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, length)) = tuple((
        complete::u64,
        preceded(tag(" "), complete::u64),
        preceded(tag(" "), complete::u64),
    ))(input)?;

    Ok((
        input,
        (
            destination..(destination + length),
            source..(source + length),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result = process(input);
        assert_eq!(46, result);
    }
}
