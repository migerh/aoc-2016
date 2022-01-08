use std::collections::VecDeque;
use std::mem::size_of;
use crate::utils::ParseError;

#[aoc_generator(day19)]
fn get_input(input: &str) -> Result<usize, ParseError> {
    Ok(input.parse::<usize>()?)
}

#[allow(dead_code)]
fn find_next(elves: &Vec<u8>, start: usize) -> Option<usize> {
    let len = elves.len();
    elves.iter()
        .enumerate()
        .cycle()
        .skip(start + 1)
        .take(len-1).filter(|(_, v)| **v != 0)
        .map(|(i, _)| i)
        .next()
}

// Simulation. Works for smaller n but not for 3M+.
#[allow(dead_code)]
fn find_survivor(n: usize) -> Result<usize, ParseError> {
    let mut pos = 0;
    let mut elves = vec![1; n];

    while let Some(e) = find_next(&elves, pos) {
        elves[pos] += elves[e];
        elves[e] = 0;

        if let Some(p_next) = find_next(&elves, e) {
            pos = p_next;
        } else {
            break;
        }
    }

    let result = elves.iter().enumerate()
        .filter(|(_, v)| **v != 0)
        .map(|(i, _)| i)
        .next()
        .ok_or(ParseError::new("No elf wins"))?;

    Ok(result + 1)
}

// Today's puzzle is also known as the Josephus Problem.
// See https://en.wikipedia.org/wiki/Josephus_problem.
//
// Our configuration for part 1 is
//    n = number of elves
//    k = 2 (every other elf gets their presents stolen)
#[aoc(day19, part1)]
fn problem1(input: &usize) -> Result<usize, ParseError> {
    let m = size_of::<usize>() * 8 - input.leading_zeros() as usize - 1;
    let l = input - (1 << m);

    Ok(2*l + 1)
}

// Simulating implementation that is far too slow for the actual input.
// But it helps to find patterns in smaller n.
// If we look at the winners for n up to 1000 we find that the winner for
// n elves where n = 3^k + 1, k in N is elf #1.
// The winner for n = 3^k + i is either i for 3^k < n <= (3^(k+1) + 3^k) / 2
// and 3^k + 2*(i - 3^k) for (3^(k+1) + 3^k) / 2 < n <= 3^(k+1)
#[allow(dead_code)]
fn stupid(n: usize) -> usize {
    let mut elves = (1..=n).collect::<VecDeque<_>>();

    while elves.len() > 1 {
        let rot = elves.len() / 2;
        elves.rotate_left(rot);
        elves.pop_front();
        elves.rotate_right(rot - 1);
    }

    elves[0]
}

fn find_powers(n: usize) -> (usize, usize) {
    let mut low = 1;
    let mut high = 3;

    while high <= n {
        low = high;
        high *= 3;
    }

    (low, high)
}

// fast solution, only works for n > 1
fn find_winner(n: usize) -> usize {
    let (low, high)= find_powers(n);
    let mean = (low + high) / 2;

    if n == low {
        n
    } else if n <= mean {
        n - low
    } else {
        low + (n - 2 * low) * 2
    }
}

#[aoc(day19, part2)]
fn problem2(input: &usize) -> Result<usize, ParseError> {
    Ok(find_winner(*input))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_powers__4__returns_3_9() {
        assert_eq!((3, 9), find_powers(4));
    }

    #[test]
    fn find_powers__19__returns_9_27() {
        assert_eq!((9, 27), find_powers(19));
    }

    #[test]
    fn find_winner__1_to_1000__correct() {
        for i in 2..=1000 {
            let simple = stupid(i);
            let fast = find_winner(i);

            assert_eq!((i, simple), (i, fast));
        }
    }
}
