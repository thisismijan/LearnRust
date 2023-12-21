use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let result = process(input);
    println!("Result: {}", result);
}

fn process(input: &str) -> i64 {
    let (_, data) = parse_input(input).expect("data");
    data.iter().map(|nums| get_next_in_sequence(&nums.iter().cloned().rev().collect::<Vec<i64>>())).sum::<i64>()
}

fn get_next_in_sequence(nums: &Vec<i64>) -> i64 {
    let mut result = *nums.last().unwrap();
    let mut diffs = nums.windows(2).map(|pair| pair[1] - pair[0]).collect::<Vec<i64>>();
    result += *diffs.last().unwrap();
    while diffs.iter().any(|&x| x != 0) {
        diffs = diffs.windows(2).map(|pair| pair[1] - pair[0]).collect();
        result += *diffs.last().unwrap();
    }
    result
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, complete::i64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = process(input);
        assert_eq!(2, result);
    }
}
