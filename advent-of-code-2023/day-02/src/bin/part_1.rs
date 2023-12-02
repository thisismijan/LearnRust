use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
pub struct Cube {
    colour: String,
    amount: u32,
}

impl Cube {
    fn new(colour: String, amount: u32) -> Self {
        Self { colour, amount }
    }
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    cubes: Vec<Cube>,
}

impl Game {
    fn new(id: u32) -> Self {
        Self {
            id,
            cubes: Vec::<Cube>::new(),
        }
    }

    fn is_valid(&mut self, max_values: HashMap<&str, u32>) -> u32 {
        if self
            .cubes
            .iter()
            .all(|c| c.amount <= *max_values.get(c.colour.as_str()).unwrap())
        {
            self.id
        } else {
            0
        }
    }

    fn add(&mut self, colour: &str, amount: u32) {
        self.cubes.push(Cube::new(colour.to_string(), amount));
    }
}

fn process(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|line| {
            let split = &mut line.split(":");
            let id = split
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let mut game = Game::new(id);
            let games = split.next().unwrap();
            games.split(";").for_each(|g| {
                g.split(",").for_each(|s| {
                    let split = &mut s.split(" ");
                    split.next();
                    let amount = split.next().unwrap().parse::<u32>().unwrap();
                    let colour = split.next().unwrap();
                    game.add(colour, amount);
                });
            });
            let max_values: HashMap<&str, u32> =
                HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
            game.is_valid(max_values)
        })
        .sum::<u32>();
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 1 green, 1 blue, 1 red; 3 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green", 1)]
    #[case("Game 5: 2 red, 1 blue, 4 green; 6 blue, 2 green; 2 red, 5 green", 5)]
    #[case("Game 95: 13 blue, 5 red; 9 blue, 3 red, 7 green; 10 green, 4 red, 12 blue; 14 blue; 7 green, 2 blue, 1 red", 95)]
    fn test_case(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, process(input))
    }

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = process(input);
        assert_eq!(8, result);
    }
}
