use std::fmt::Display;
use std::cmp::min;
use std::collections::VecDeque;
use regex::Regex;
use std::str::FromStr;
use crate::utils::ParseError;
use std::fmt::Write;

enum Command {
    Rect((usize, usize)),
    RotateRow((usize, usize)),
    RotateColumn((usize, usize)),
}

impl Command {
    fn from_rect(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^rect (?P<x>\d+)?x(?P<y>\d+)$").unwrap();
        }

        let (x, y) = RE.captures(s).and_then(|cap| {
            let x = cap.name("x").map(|v| v.as_str().parse::<usize>())?.ok()?;
            let y = cap.name("y").map(|v| v.as_str().parse::<usize>())?.ok()?;

            Some((x, y))
        }).ok_or(ParseError::new("Error during parse"))?;

        Ok(Self::Rect((x, y)))
    }

    fn from_rotate_col(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^rotate column x=(?P<col>\d+)? by (?P<len>\d+)$").unwrap();
        }

        let (col, len) = RE.captures(s).and_then(|cap| {
            let col = cap.name("col").map(|v| v.as_str().parse::<usize>())?.ok()?;
            let len = cap.name("len").map(|v| v.as_str().parse::<usize>())?.ok()?;

            Some((col, len))
        }).ok_or(ParseError::new("Error during parse"))?;

        Ok(Self::RotateColumn((col, len)))
    }

    fn from_rotate_row(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^rotate row y=(?P<row>\d+)? by (?P<len>\d+)$").unwrap();
        }

        let (row, len) = RE.captures(s).and_then(|cap| {
            let row = cap.name("row").map(|v| v.as_str().parse::<usize>())?.ok()?;
            let len = cap.name("len").map(|v| v.as_str().parse::<usize>())?.ok()?;

            Some((row, len))
        }).ok_or(ParseError::new("Error during parse"))?;

        Ok(Self::RotateRow((row, len)))
    }
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        if s.starts_with("rect") {
            Self::from_rect(s)
        } else if s.starts_with("rotate c") {
            Self::from_rotate_col(s)
        } else if s.starts_with("rotate r") {
            Self::from_rotate_row(s)
        } else {
            Err(ParseError::new("Could not parse command"))
        }
    }
}

#[aoc_generator(day8)]
fn get_input(input: &str) -> Result<Vec<Command>, ParseError> {
    input
        .lines()
        .map(|l| Command::from_str(l))
        .collect::<Result<Vec<_>, ParseError>>()
}

struct TinyDisplay {
    data: Vec<VecDeque<bool>>
}

impl TinyDisplay {
    fn new(w: usize, h: usize) -> Self {
        let row = vec![false; w].into_iter().collect::<VecDeque<_>>();
        let data = vec![row; h];

        Self { data }
    }

    fn rect(&mut self, w: usize, h: usize) {
        let w = min(w, self.data[0].len());
        let h = min(h, self.data.len());

        for y in 0..h {
            for x in 0..w {
                self.data[y][x] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, len: usize) {
        let row = min(row, self.data.len());
        let len = len % self.data[0].len();

        self.data[row].rotate_right(len);
    }

    fn rotate_col(&mut self, col: usize, len: usize) {
        let height = self.data.len();
        let col = min(col, self.data[0].len());
        let len = len % self.data.len();

        let backup = self.data.iter().map(|r| r[col]).collect::<Vec<_>>();

        for (i, b) in backup.into_iter().enumerate() {
            self.data[(i + len) % height][col] = b;
        }
    }

    fn apply(&mut self, c: &Command) {
        use Command::*;

        match c {
            &Rect((w, h)) => self.rect(w, h),
            &RotateColumn((col, len)) => self.rotate_col(col, len),
            &RotateRow((row, len)) => self.rotate_row(row, len),
        }
    }

    fn check(&self) -> usize {
        self.data.iter().map(|r| r.iter().filter(|p| **p).count()).sum()
    }
}

impl Display for TinyDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for row in &self.data {
            for p in row {
                f.write_char(if *p { '#' } else { ' ' })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[aoc(day8, part1)]
fn problem1(input: &Vec<Command>) -> Result<usize, ParseError> {
    let mut display = TinyDisplay::new(50, 6);

    for c in input {
        display.apply(c);
    }

    Ok(display.check())
}

#[aoc(day8, part2)]
fn problem2(input: &Vec<Command>) -> Result<usize, ParseError> {
    let mut display = TinyDisplay::new(50, 6);

    for c in input {
        display.apply(c);
    }
    println!("{}", display);

    Ok(0)
}
