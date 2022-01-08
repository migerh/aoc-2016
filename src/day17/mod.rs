use crate::utils::ParseError;
use pathfinding::prelude::{dijkstra, dijkstra_all};

type C = isize;
type Coords = (C, C, Vec<char>);

fn door_config(code: &str, path: &Vec<char>) -> [bool; 4] {
    let input = format!("{}{}", code, path.iter().collect::<String>());
    let hash = format!("{:x}", md5::compute(input.as_bytes()));

    hash.chars()
        .take(4)
        .map(|c| match c {
            'b'..='f' => true,
            _ => false,
        })
        .collect::<Vec<_>>()
        .try_into().unwrap()
}

fn get_neighbors(code: &str, p: &Coords) -> Vec<Coords> {
    if p.0 == 3 && p.1 == 3 {
        return vec![]
    }

    let config = door_config(code, &p.2);

    [(0, -1, 'U'), (0, 1, 'D'), (-1, 0, 'L'), (1, 0, 'R')].into_iter()
        .enumerate()
        .map(|(i, c)| {
            let mut path = p.2.clone();
            path.push(c.2);

            (c.0 + p.0, c.1 + p.1, path, config[i])
        })
        .filter(|c| c.0 >= 0 && c.1 >= 0 && c.0 <= 3 && c.1 <= 3 && c.3)
        .map(|c| {
            (c.0, c.1, c.2)
        })
        .collect::<Vec<_>>()
}

fn get_weighted_neighbors(code: &str, p: &Coords) -> Vec<(Coords, usize)> {
    get_neighbors(code, p).into_iter()
        .map(|c| (c, 1))
        .collect::<Vec<_>>()
}

#[aoc(day17, part1)]
fn problem1(input: &str) -> Result<String, ParseError> {
    let start = (0, 0, vec![]);

    let shortest = dijkstra(&start, |p| get_weighted_neighbors(input, p), |p| p.0 == 3 && p.1 == 3)
        .ok_or(ParseError::new("Could not find shortest path"))?;
    let path = shortest.0.into_iter().rev().next().ok_or(ParseError::new("Path is empty"))?;
    Ok(path.2.into_iter().collect::<String>())
}

#[aoc(day17, part2)]
fn problem2(input: &str) -> Result<usize, ParseError> {
    let start = (0, 0, vec![]);

    let all = dijkstra_all(&start, |p| get_weighted_neighbors(input, p));
    let mut lengths = all.into_iter()
        .filter(|(k, _)| k.0 == 3 && k.1 == 3)
        .map(|(_, v)| v.1)
        .collect::<Vec<_>>();
    lengths.sort();
    let max = lengths.into_iter().rev().next().ok_or(ParseError::new("Could not find path with max length"))?;

    Ok(max)
}
