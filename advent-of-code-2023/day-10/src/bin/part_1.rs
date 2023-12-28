use std::collections::BTreeMap;

use nom::{
    branch::alt,
    character::complete::{self, multispace0},
    IResult, Parser,
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let input = include_str!("input1.txt");
    let result = process(input);
    println!("Result: {}", result);
}

fn process(input: &str) -> u64 {
    let (_, grid) = parse_input(input).expect("data");
    let mut steps = 0;
    let mut current_pos = grid.start_pos;
    let mut current_pipe = grid.get_pipe(current_pos);
    let mut current_dir = Direction::North;
    for dir in Direction::iter() {
        if current_pipe.valid_dir(&dir) {
            current_dir = dir;
            break;
        }
    }
    let dir = DIRS[current_dir as usize];
    current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
    current_pipe = grid.get_pipe(current_pos);
    current_dir = current_pipe.next_dir(&current_dir);
    steps += 1;
    while current_pos != grid.start_pos {
        let dir = DIRS[current_dir as usize];
        current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
        current_pipe = grid.get_pipe(current_pos);
        steps += 1;
        current_dir = current_pipe.next_dir(&current_dir);
    }
    steps / 2
}

fn parse_input(input: &str) -> IResult<&str, Grid> {
    let (_, mut grid) = parse_pipe(input, Grid::new(), 0, 0).expect("grid");
    let north = grid
        .get_pipe((grid.start_pos.0 + DIRS[0].0, grid.start_pos.1 + DIRS[0].1))
        .valid_dir(&Direction::South);
    let east = grid
        .get_pipe((grid.start_pos.0 + DIRS[1].0, grid.start_pos.1 + DIRS[1].1))
        .valid_dir(&Direction::West);
    let south = grid
        .get_pipe((grid.start_pos.0 + DIRS[2].0, grid.start_pos.1 + DIRS[2].1))
        .valid_dir(&Direction::North);
    let west = grid
        .get_pipe((grid.start_pos.0 + DIRS[3].0, grid.start_pos.1 + DIRS[3].1))
        .valid_dir(&Direction::East);
    let start_pipe = match (north, east, south, west) {
        (true, true, false, false) => Pipe::NorthEast,
        (true, false, true, true) => Pipe::SouthWest,
        (true, false, true, false) => Pipe::NorthSouth,
        (true, false, false, true) => Pipe::NorthWest,
        (false, true, true, false) => Pipe::SouthEast,
        (false, true, false, true) => Pipe::EastWest,
        (false, false, true, true) => Pipe::SouthWest,
        _ => panic!("{north} {east} {south} {west} invalid start"),
    };
    grid.map.insert(grid.start_pos, start_pipe);
    Ok((input, grid))
}

fn parse_pipe(
    input: &str,
    mut grid: Grid,
    mut current_col: i64,
    mut current_row: i64,
) -> IResult<&str, Grid> {
    if input.is_empty() {
        return Ok((input, grid));
    }

    let (input, pipe) = alt((
        complete::char('|').map(|_| Pipe::NorthSouth),
        complete::char('-').map(|_| Pipe::EastWest),
        complete::char('7').map(|_| Pipe::SouthWest),
        complete::char('F').map(|_| Pipe::SouthEast),
        complete::char('J').map(|_| Pipe::NorthWest),
        complete::char('L').map(|_| Pipe::NorthEast),
        complete::char('.').map(|_| Pipe::Nothing),
        complete::char('S').map(|_| Pipe::Start), // for now, will figure out Pipe later
        multispace0.map(|_| Pipe::NewLine),
    ))(input)?;
    if pipe == Pipe::Start {
        grid.start_pos = (current_col, current_row);
        current_col += 1;
    } else if pipe == Pipe::NewLine {
        current_row += 1;
        current_col = 0;
    } else {
        grid.map.insert((current_col, current_row), pipe);
        current_col += 1;
    }

    parse_pipe(input, grid, current_col, current_row)
}

const DIRS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug, PartialEq, Default, Copy, Clone)]
enum Pipe {
    Start,      //S to be replaced once figured out which pipe it is
    NorthSouth, //|
    EastWest,   //-
    SouthWest,  //7
    SouthEast,  //F
    NorthWest,  //J
    NorthEast,  //L
    #[default]
    Nothing, //.
    NewLine,    //use to determine line break in input
}

impl Pipe {
    fn valid_dir(&self, dir: &Direction) -> bool {
        matches!(
            (self, dir),
            (Pipe::NorthSouth, Direction::North | Direction::South)
                | (Pipe::EastWest, Direction::East | Direction::West)
                | (Pipe::NorthEast, Direction::North | Direction::East)
                | (Pipe::NorthWest, Direction::North | Direction::West)
                | (Pipe::SouthEast, Direction::South | Direction::East)
                | (Pipe::SouthWest, Direction::South | Direction::West)
        )
    }

    fn next_dir(&self, dir: &Direction) -> Direction {
        match (self, dir) {
            (Pipe::NorthSouth, Direction::North) => Direction::North,
            (Pipe::NorthSouth, Direction::South) => Direction::South,
            (Pipe::EastWest, Direction::East) => Direction::East,
            (Pipe::EastWest, Direction::West) => Direction::West,
            (Pipe::NorthEast, Direction::South) => Direction::East,
            (Pipe::NorthEast, Direction::West) => Direction::North,
            (Pipe::NorthWest, Direction::South) => Direction::West,
            (Pipe::NorthWest, Direction::East) => Direction::North,
            (Pipe::SouthEast, Direction::North) => Direction::East,
            (Pipe::SouthEast, Direction::West) => Direction::South,
            (Pipe::SouthWest, Direction::North) => Direction::West,
            (Pipe::SouthWest, Direction::East) => Direction::South,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Debug, PartialEq, EnumIter, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Grid {
    map: BTreeMap<(i64, i64), Pipe>,
    start_pos: (i64, i64),
}

impl Grid {
    fn new() -> Grid {
        Grid {
            map: BTreeMap::new(),
            start_pos: (0, 0),
        }
    }

    fn get_pipe(&self, pos: (i64, i64)) -> Pipe {
        *self
            .map
            .get(&pos)
            .unwrap_or(&Pipe::Nothing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let result = process(input);
        assert_eq!(4, result);
    }

    #[test]
    fn test_process_() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let result = process(input);
        assert_eq!(8, result);
    }
}
