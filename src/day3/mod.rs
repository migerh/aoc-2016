use regex::Regex;
use crate::utils::ParseError;

type Triangle = [usize; 3];

fn parse_triangle(s: &str) -> Result<Triangle, ParseError> {
    lazy_static!{
        static ref RE: Regex = Regex::new(r"^\s*(?P<a>\d*?)\s*(?P<b>\d*?)\s*(?P<c>\d*)\w*$").unwrap();
    }

    let (a, b, c) = RE.captures(s).and_then(|cap| {
        let a = cap.name("a").map(|v| v.as_str().parse::<usize>())?.ok()?;
        let b = cap.name("b").map(|v| v.as_str().parse::<usize>())?.ok()?;
        let c = cap.name("c").map(|v| v.as_str().parse::<usize>())?.ok()?;

        Some((a, b, c))
    }).ok_or(ParseError::new("Error during parse"))?;

    Ok([a, b, c])
}

#[aoc_generator(day3)]
fn get_input(input: &str) -> Result<Vec<Triangle>, ParseError> {
    Ok(input
        .lines()
        .map(|t| parse_triangle(t))
        .collect::<Result<Vec<_>, ParseError>>()?)
}

fn is_valid(triangle: &Triangle) -> bool {
    let a = triangle[0];
    let b = triangle[1];
    let c = triangle[2];

    a < b + c && b < a + c && c < a + b
}

#[aoc(day3, part1)]
fn problem1(input: &Vec<Triangle>) -> Result<usize, ParseError> {
    Ok(input.iter().filter(|t| is_valid(t)).count())
}

fn transform(triangles_in: &Vec<Triangle>) -> Vec<Triangle> {
    let mut result = vec![];

    for t in (0..triangles_in.len()).step_by(3) {
        for i in 0..3 {
            let new_t = [triangles_in[t][i], triangles_in[t+1][i], triangles_in[t+2][i]];
            result.push(new_t);
        }
    }

    result
}

#[aoc(day3, part2)]
fn problem2(input: &Vec<Triangle>) -> Result<usize, ParseError> {
    let triangles = transform(input);
    Ok(triangles.iter().filter(|t| is_valid(t)).count())
}
