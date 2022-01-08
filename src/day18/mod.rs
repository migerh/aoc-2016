use crate::utils::ParseError;

#[aoc_generator(day18)]
fn get_input(input: &str) -> Result<Vec<char>, ParseError> {
    Ok(input.chars().collect::<Vec<_>>())
}

fn is_trap(window: &[char]) -> bool {
    match window {
        // rules as written, in order
        ['^', '^', '.'] => true,
        ['.', '^', '^'] => true,
        ['^', '.', '.'] => true,
        ['.', '.', '^'] => true,
        _ => false,
    }
}

fn next_row(row: &Vec<char>) -> Vec<char> {
    let mut extended = vec!['.'];
    extended.append(&mut row.clone());
    extended.push('.');

    let mut next = vec![];
    for window in extended.windows(3) {
        next.push(if is_trap(window) {
            '^'
        } else {
            '.'
        });
    }

    next
}

fn count_safe(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .map(|v| v.iter().filter(|c| **c == '.').count())
        .sum()
}

fn extend_and_count(start: &Vec<char>, target_size: usize) -> usize {
    let mut map = vec![start.clone()];
    for i in 0..target_size-1 {
        let next = next_row(&map[i]);
        map.push(next);
    }

    count_safe(&map)
}

#[aoc(day18, part1)]
fn problem1(input: &Vec<char>) -> Result<usize, ParseError> {
    // account for the reduced example size
    let target_size = if input.len() == 10 { 10 } else { 40 };
    Ok(extend_and_count(input, target_size))
}

#[aoc(day18, part2)]
fn problem2(input: &Vec<char>) -> Result<usize, ParseError> {
    let target_size = 400_000;
    Ok(extend_and_count(input, target_size))
}
