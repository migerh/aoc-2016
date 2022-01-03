use std::cmp::Ordering;
use std::collections::HashMap;
use crate::utils::ParseError;

#[aoc_generator(day6)]
fn get_input(input: &str) -> Result<Vec<Vec<char>>, ParseError> {
    Ok(input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

type Comparer = fn(&(char, usize), &(char, usize)) -> Ordering;
fn decode_pos(input: &Vec<Vec<char>>, pos: usize, cmp: Comparer) -> char {
    let mut histogram: HashMap<char, usize> = HashMap::new();
    for line in input {
        let c = line[pos];
        histogram.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut freq = histogram.into_iter().collect::<Vec<_>>();
    freq.sort_by(cmp);

    freq[0].0
}

#[aoc(day6, part1)]
fn problem1(input: &Vec<Vec<char>>) -> Result<String, ParseError> {
    let size = input[0].len();
    Ok((0..size).map(|p| decode_pos(input, p, |a, b| b.1.cmp(&a.1))).collect::<String>())
}

#[aoc(day6, part2)]
fn problem2(input: &Vec<Vec<char>>) -> Result<String, ParseError> {
    let size = input[0].len();
    Ok((0..size).map(|p| decode_pos(input, p, |a, b| a.1.cmp(&b.1))).collect::<String>())
}
