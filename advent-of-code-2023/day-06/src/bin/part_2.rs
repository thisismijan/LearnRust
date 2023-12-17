use nom::{
    bytes::complete::is_not,
    character::complete::{digit1, line_ending, space1},
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
    let (_, (time, distance)) = parse_race(input).expect("races");

    (0..time)
        .into_iter()
        .filter_map(|held_time| {
            let distance_traveled = (time - held_time) * held_time;
            (distance_traveled > distance).then_some(distance_traveled)
        })
        .count()
}

fn parse_values(input: &str) -> IResult<&str, u64> {
    let (input, num) =
        preceded(is_not("0123456789"), separated_list1(space1, digit1)).parse(input)?;
    let value = num.join("").parse::<u64>().expect("value");
    Ok((input, value))
}

fn parse_race(input: &str) -> IResult<&str, (u64, u64)> {
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
        assert_eq!(71503, result);
    }
}
