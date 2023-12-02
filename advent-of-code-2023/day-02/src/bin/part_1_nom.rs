use std::collections::HashMap;
use nom::{sequence::separated_pair, multi::separated_list1};
use nom::{character::complete, bytes::complete::tag};
use nom::{character::complete::alpha1, IResult};
use nom::{character::complete::{line_ending, digit1}, sequence::preceded};

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
    fn new(id: u32, cubes: Vec<Cube>) -> Self {
        Self {
            id,
            cubes,
        }
    }

    fn is_valid(&self, max_values: &HashMap<&str, u32>) -> u32 {
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
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, colour)) =
        separated_pair(complete::u32, tag(" "), alpha1)(
            input,
        )?;
        
    Ok((input, Cube::new( colour.to_string(), amount )))
}

fn parse_cubes(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) =
        separated_list1(tag(", "), parse_cube)(input)?;
    Ok((input, cubes))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    dbg!(input);
    let (input, id) =
        preceded(tag("Game "), digit1)(input)?;
    let (input, cubes) = preceded(
        tag(": "),
        separated_list1(tag("; "), parse_cubes),
    )(input)?;
    Ok((input, Game::new(id.parse::<u32>().unwrap(),  cubes.into_iter().flatten().collect())))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) =
        separated_list1(line_ending, parse_game)(input)?;
    Ok((input, games))
}


fn process(input: &str) -> u32 {
    let games = parse_games(input).unwrap();
    // dbg!(&games);
    let max_values: HashMap<&str, u32> =
    HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    games.1.iter().map(|game| {
        // dbg!(game);
        game.is_valid(&max_values)
    }).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

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
