use crate::utils::ParseError;

#[aoc_generator(day16)]
fn get_input(input: &str) -> Result<Vec<char>, ParseError> {
    Ok(input.chars().collect::<Vec<_>>())
}

fn extend(input: &Vec<char>, size: usize) -> Vec<char> {
    let mut extended = input.clone();

    while extended.len() < size {
        let mut b = extended.clone().into_iter().rev().map(|v| {
            if v == '0' { '1' } else { '0' }
        }).collect::<Vec<_>>();
        extended.push('0');
        extended.append(&mut b);
    }

    extended
}

fn checksum(input: &Vec<char>, size: usize) -> Vec<char> {
    let mut checksum = input.iter().take(size).cloned().collect::<Vec<_>>();

    while checksum.len() % 2 == 0 {
        let mut next = vec![];
        for pair in checksum.chunks(2) {
            next.push(if pair[0] == pair[1] { '1' } else { '0' });
        }
        checksum = next;
    }

    checksum
}

#[aoc(day16, part1)]
fn problem1(input: &Vec<char>) -> Result<String, ParseError> {
    let size = 272;
    let extended = extend(input, size);
    let checksum = checksum(&extended, size);
    Ok(checksum.into_iter().collect::<String>())
}

#[aoc(day16, part2)]
fn problem2(input: &Vec<char>) -> Result<String, ParseError> {
    let size = 35651584;
    let extended = extend(input, size);
    let checksum = checksum(&extended, size);
    Ok(checksum.into_iter().collect::<String>())
}
