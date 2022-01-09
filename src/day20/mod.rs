use std::cmp::max;
use std::ops::Range;
use std::num::ParseIntError;
use crate::utils::ParseError;

#[aoc_generator(day20)]
fn get_input(input: &str) -> Result<Vec<Range<usize>>, ParseError> {
    input.lines()
        .map(|l| -> Result<Range<usize>, ParseError> {
            let range = l.split("-")
                .map(|s| s.parse::<usize>())
                .collect::<Result<Vec<_>, ParseIntError>>()?;

            if range.len() != 2 {
                Err(ParseError::new("Invalid range"))?;
            }

            Ok(Range { start: range[0], end: range[1] })
        })
        .collect::<Result<Vec<_>, ParseError>>()
}

#[aoc(day20, part1)]
fn problem1(input: &Vec<Range<usize>>) -> Result<usize, ParseError> {
    let mut ranges = input.clone();
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut lowest = 0;
    for r in &ranges {
        if r.start <= lowest && lowest < r.end {
            lowest = r.end + 1;
        }
    }

    Ok(lowest)
}

fn merge(ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    let mut merged = vec![];

    let mut start = ranges[0].start;
    let mut end = ranges[0].end;
    let mut open = true;
    for r in ranges.iter().skip(1) {
        if end < r.start {
            merged.push(Range { start, end });
            start = r.start;
            end = r.end;
            open = false;
            continue;
        }

        open = true;
        end = max(end, r.end);
    }

    if open {
        merged.push(Range { start, end });
    }
    merged
}

#[aoc(day20, part2)]
fn problem2(input: &Vec<Range<usize>>) -> Result<usize, ParseError> {
    let mut ranges = input.clone();
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    ranges = merge(ranges);

    let mut allowed = ranges[0].start;
    for r in ranges.windows(2) {
        allowed += r[1].start - r[0].end - 1;
    }
    let max = u32::MAX as usize;
    let last = ranges[ranges.len() - 1].end;
    allowed += max - last;

    Ok(allowed)
}
