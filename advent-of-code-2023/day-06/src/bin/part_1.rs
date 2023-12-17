use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    let input = include_str!("input1.txt");
    let result = process(input);
    println!("Result: {}", result);
}

fn process(input: &str) -> usize {
    let (_, (times, distances)) = parse_races(input).expect("races");
    times
        .iter()
        .zip(distances)
        .map(|(time, distance)| {
            (0..*time)
                .into_iter()
                .filter_map(|held_time| {
                    let distance_traveled = (time - held_time) * held_time;
                    (distance_traveled > distance).then_some(distance_traveled)
                })
                .count()
        })
        .product()
}

fn parse_values(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(is_not("0123456789"), separated_list1(space1, complete::u32)).parse(input)
}

fn parse_races(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(parse_values, line_ending, parse_values)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let result = process(input);
        assert_eq!(288, result);
    }
}
