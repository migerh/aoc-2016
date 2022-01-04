use std::cmp::min;
use std::collections::VecDeque;
use crate::utils::ParseError;

fn read_until(input: &mut VecDeque<char>, stop: char) -> (String, usize) {
    let mut read = 0;
    let mut buffer = vec![];
    while let Some(l) = input.pop_front() {
        read += 1;
        if l == stop {
            break;
        }
        buffer.push(l);
    }
    (buffer.into_iter().collect::<String>(), read)
}

fn read_next(input: &mut VecDeque<char>, len: usize) -> String {
    let mut buffer = vec![];
    let mut i = 0;
    while let Some(l) = input.pop_front() {
        buffer.push(l);
        i += 1;

        if i >= len {
            break;
        }
    }
    buffer.into_iter().collect::<String>()
}

fn expand(s: &str) -> Result<String, ParseError> {
    let mut basket = vec![];
    let mut input = s.chars().collect::<VecDeque<_>>();

    while let Some(c) = input.pop_front() {
        if c == '(' {
            let len = read_until(&mut input, 'x').0.parse::<usize>()?;
            let times = read_until(&mut input, ')').0.parse::<usize>()?;

            let substr = read_next(&mut input, len);
            for _ in 0..times {
                basket.append(&mut substr.chars().collect::<Vec<_>>());
            }

            continue;
        }

        basket.push(c);
    }

    Ok(basket.into_iter().collect::<String>())
}

#[aoc(day9, part1)]
fn problem1(input: &str) -> Result<usize, ParseError> {
    Ok(expand(input)?.len())
}

fn count_expansion(s: &str) -> Result<usize, ParseError> {
    let mut weights = vec![1; s.len()];
    let mut input = s.chars().collect::<VecDeque<_>>();
    let mut pos = 0;
    let mut sum = 0;

    while let Some(c) = input.pop_front() {
        if c == '(' {
            pos += 1;

            let (len, read) = read_until(&mut input, 'x');
            pos += read;
            let len = len.parse::<usize>()?;

            let (times, read) = read_until(&mut input, ')');
            pos += read;
            let times = times.parse::<usize>()?;

            let end = min(pos + len, weights.len());
            for i in pos..end {
                weights[i] *= times;
            }

            continue;
        }

        sum += weights[pos];
        pos += 1;
    }

    Ok(sum)
}

#[aoc(day9, part2)]
fn problem2(input: &str) -> Result<usize, ParseError> {
    count_expansion(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1() -> Result<(), ParseError> {
        Ok(assert_eq!("ADVENT", expand("ADVENT")?))
    }

    #[test]
    pub fn example_2() -> Result<(), ParseError> {
        Ok(assert_eq!("ABBBBBC", expand("A(1x5)BC")?))
    }

    #[test]
    pub fn example_3() -> Result<(), ParseError> {
        Ok(assert_eq!("XYZXYZXYZ", expand("(3x3)XYZ")?))
    }

    #[test]
    pub fn example_4() -> Result<(), ParseError> {
        Ok(assert_eq!("ABCBCDEFEFG", expand("A(2x2)BCD(2x2)EFG")?))
    }

    #[test]
    pub fn example_5() -> Result<(), ParseError> {
        Ok(assert_eq!("(1x3)A", expand("(6x1)(1x3)A")?))
    }

    #[test]
    pub fn example_6() -> Result<(), ParseError> {
        Ok(assert_eq!("X(3x3)ABC(3x3)ABCY", expand("X(8x2)(3x3)ABCY")?))
    }

    #[test]
    pub fn example_7() -> Result<(), ParseError> {
        Ok(assert_eq!(241920, count_expansion("(27x12)(20x12)(13x14)(7x10)(1x12)A")?))
    }

    #[test]
    pub fn example_8() -> Result<(), ParseError> {
        Ok(assert_eq!(445, count_expansion("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")?))
    }
}
