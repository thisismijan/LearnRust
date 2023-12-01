fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|c| c.to_digit(10));
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
        let input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

        let result = process(input);
        assert_eq!(142, result);
    }
}
