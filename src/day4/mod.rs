use std::collections::HashMap;
use regex::Regex;
use std::cmp::Ordering;
use std::str::FromStr;
use crate::utils::ParseError;

#[derive(Debug)]
struct Entry {
    name: String,
    sector: usize,
    checksum: String,
}

impl FromStr for Entry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^(?P<name>.*)-(?P<section>\d*)\[(?P<check>\w{5,5})\]$").unwrap();
        }

        let (name, section, checksum) = RE.captures(s).and_then(|cap| {
            let name = cap.name("name").map(|v| v.as_str())?;
            let section = cap.name("section").map(|v| v.as_str().parse::<usize>())?.ok()?;
            let checksum = cap.name("check").map(|v| v.as_str())?;

            Some((name, section, checksum))
        }).ok_or(ParseError::new("Error during parse"))?;

        Ok(Self::new(name, section, checksum))
    }
}

impl Entry {
    fn new(name: &str, sector: usize, checksum: &str) -> Self {
        let name = name.to_string();
        let checksum = checksum.to_string();

        Self { name, sector, checksum }
    }

    fn checksum(&self) -> String {
        let mut histogram = HashMap::new();
        let chars = self.name.chars().filter(|c| *c != '-');
        for c in chars {
            histogram.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut freq = histogram.into_iter().collect::<Vec<_>>();
        freq.sort_by(|a, b| {
            let val = b.1.cmp(&a.1);

            if val == Ordering::Equal {
                a.0.cmp(&b.0)
            } else {
                val
            }
        });

        freq.into_iter().take(5).map(|(c, _)| c).collect::<String>()
    }

    fn verify(&self) -> bool {
        let check = self.checksum();

        check == self.checksum
    }

    fn shift(c: char, rot: usize) -> char {
        (((((c as u8 - 97) as usize) + rot) % 26) as u8 + 97) as char
    }

    fn decrypt(&self) -> String {
        self.name.chars().map(|c| if c == '-' {
            ' '
        } else {
            Self::shift(c, self.sector)
        })
        .collect::<String>()
    }
}

#[aoc_generator(day4)]
fn get_input(input: &str) -> Result<Vec<Entry>, ParseError> {
    input
        .lines()
        .map(|l| Entry::from_str(l))
        .collect::<Result<Vec<_>, ParseError>>()
}

#[aoc(day4, part1)]
fn problem1(input: &Vec<Entry>) -> Result<usize, ParseError> {
    Ok(input
        .iter()
        .filter(|r| r.verify())
        .map(|r| r.sector)
        .sum())
}

#[aoc(day4, part2)]
fn problem2(input: &Vec<Entry>) -> Result<usize, ParseError> {
    let north = input
        .iter()
        .filter(|r| r.verify())
        .filter(|r| r.decrypt().contains("orth"))
        .next()
        .ok_or(ParseError::new("No room with 'north' in its name found"))?;

    Ok(north.sector)
}
