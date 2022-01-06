use std::collections::HashSet;
use crate::utils::ParseError;
use pathfinding::prelude::dijkstra;
use memoize::memoize;

type C = isize;
type Coords = (C, C);

#[aoc_generator(day13)]
fn get_input(input: &str) -> Result<isize, ParseError> {
    Ok(input.parse::<isize>()?)
}

#[memoize]
fn is_open(p: Coords, c: isize) -> bool {
    let x = p.0;
    let y = p.1;
    let v = x*x + 3*x + 2*x*y + y + y*y + c;

    v.count_ones() % 2 == 0
}

fn neighbors(p: &Coords, c: isize) -> Vec<(Coords, usize)> {
    let delta = [-1, 1];
    let mut result = vec![];

    for d in delta {
        if is_open((p.0 + d, p.1), c) {
            result.push((p.0 + d, p.1));
        }

        if is_open((p.0, p.1 + d), c) {
            result.push((p.0, p.1 + d));
        }
    }

    result.into_iter()
        .filter(|p| p.0 >= 0 && p.1 >= 0)
        .map(|p| (p, 1))
        .collect::<Vec<_>>()
}

#[aoc(day13, part1)]
fn problem1(input: &isize) -> Result<usize, ParseError> {
    let start = (1, 1);
    let destination = (31, 39);
    let shortest = dijkstra(&start, |p| neighbors(p, *input), |p| *p == destination).ok_or(ParseError::new("Could not find shortest path"))?;

    Ok(shortest.1)
}

#[aoc(day13, part2)]
fn problem2(input: &isize) -> Result<usize, ParseError> {
    let mut in_range = HashSet::new();

    for y in 0..50 {
        for x in 0..50 {
            if !is_open((x, y), *input) {
                continue
            }

            let start = (1, 1);
            let destination = (x, y);
            if let Some(shortest) = dijkstra(&start, |p| neighbors(p, *input), |p| *p == destination) {
                if shortest.1 <= 50 {
                    in_range.insert((x, y));
                }
            }
        }
    }

    Ok(in_range.len())
}
