fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|i| {
                let line_substr = &line[i..];
                let result = if line_substr.starts_with("one") {
                    '1'
                } else if line_substr.starts_with("two") {
                    '2'
                } else if line_substr.starts_with("three") {
                    '3'
                } else if line_substr.starts_with("four") {
                    '4'
                } else if line_substr.starts_with("five") {
                    '5'
                } else if line_substr.starts_with("six") {
                    '6'
                } else if line_substr.starts_with("seven") {
                    '7'
                } else if line_substr.starts_with("eight") {
                    '8'
                } else if line_substr.starts_with("nine") {
                    '9'
                } else {
                    line_substr.chars().next().unwrap()
                };
                result.to_digit(10)
            });
            let first = it.next().unwrap();
            let last = it.last();
            match last {
                Some(num) => format!("{}{}", first, num),
                None => format!("{}{}", first, first),
            }
            .parse::<u32>()
            .unwrap()
        })
        .sum::<u32>();

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let result = process(input);
        assert_eq!(281, result);
    }
}
