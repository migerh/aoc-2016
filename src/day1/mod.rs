use std::collections::HashSet;
use std::str::FromStr;
use crate::utils::ParseError;

type C = isize;
type Coords = (C, C);

enum Command {
    Right(usize),
    Left(usize),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        use Command::*;

        let first = s.trim().chars().next().ok_or(ParseError::new("Could not get first char"))?;
        let len = s.trim().chars().skip(1).collect::<String>().parse::<usize>()?;

        Ok(match first {
            'L' => Left(len),
            'R' => Right(len),
            _ => Err(ParseError::new("Could not parse command"))?
        })
    }
}

#[aoc_generator(day1)]
fn get_input(input: &str) -> Result<Vec<Command>, ParseError> {
    input
        .split(",")
        .map(|c| Command::from_str(c))
        .collect::<Result<Vec<_>, ParseError>>()
}

fn rotate(dir: Coords, c: &Command) -> (Coords, usize) {
    use Command::*;

    match (dir, c) {
        ((0, 1), Right(v)) => ((1, 0), *v),
        ((1, 0), Right(v)) => ((0, -1), *v),
        ((0, -1), Right(v)) => ((-1, 0), *v),
        ((-1, 0), Right(v)) => ((0, 1), *v),

        ((0, 1), Left(v)) => ((-1, 0), *v),
        ((-1, 0), Left(v)) => ((0, -1), *v),
        ((0, -1), Left(v)) => ((1, 0), *v),
        ((1, 0), Left(v)) => ((0, 1), *v),
        _ => panic!("Oh no"),
    }
}

#[aoc(day1, part1)]
fn problem1(input: &Vec<Command>) -> Result<isize, ParseError> {
    let mut pos = (0, 0);
    let mut dir = (0, 1);

    for c in input {
        let r = rotate(dir, c);
        dir = r.0;

        let len = r.1 as isize;
        pos.0 += len * dir.0;
        pos.1 += len * dir.1;
    }

    Ok(pos.0.abs() + pos.1.abs())
}

// 242
// 156
#[aoc(day1, part2)]
fn problem2(input: &Vec<Command>) -> Result<isize, ParseError> {
    let mut pos = (0, 0);
    let mut dir = (0, 1);

    let mut visited = HashSet::new();
    visited.insert(pos);

    for c in input {
        let r = rotate(dir, c);
        dir = r.0;

        let len = r.1 as isize;

        for _ in 0..len {
            pos.0 += dir.0;
            pos.1 += dir.1;

            if visited.contains(&pos) {
                return Ok(pos.0.abs() + pos.1.abs())
            }
            visited.insert(pos);
        }

    }

    Err(ParseError::new("Could not find place that was visited twice"))
}
