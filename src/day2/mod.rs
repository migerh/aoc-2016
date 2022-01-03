use std::cmp::min;
use std::cmp::max;
use crate::utils::ParseError;

type C = isize;
type Coords = (C, C);

#[aoc_generator(day2)]
fn get_input(input: &str) -> Result<Vec<Vec<char>>, ParseError> {
    Ok(input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

fn keypad() -> Vec<Vec<u8>> {
    "123\n456\n789"
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn trace(start: Coords, dir: &Vec<char>) -> Coords {
    let mut p = start;

    for d in dir {
        let next = match d {
            'U' => {
                (p.0, p.1 - 1)
            },
            'L' => {
                (p.0 - 1, p.1)
            },
            'D' => {
                (p.0, p.1 + 1)
            },
            'R' => {
                (p.0 + 1, p.1)
            },
            _ => panic!("Impossible!"),
        };
        p.0 = min(2, max(next.0, 0));
        p.1 = min(2, max(next.1, 0));
    }

    p
}

#[aoc(day2, part1)]
fn problem1(input: &Vec<Vec<char>>) -> Result<String, ParseError> {
    let kp = keypad();
    let mut pos = (1, 1);
    let mut code = vec![];
    for n in input {
        pos = trace(pos, n);
        code.push(kp[pos.1 as usize][pos.0 as usize]);
    }
    Ok(code.iter().map(|v| char::from_digit(*v as u32, 10).unwrap()).collect::<String>())
}

fn keypad2() -> Vec<Vec<char>> {
    "  1  \n 234 \n56789\n ABC \n  D  "
        .lines()
        .map(|l| l.chars().map(|c| c).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn trace2(start: Coords, dir: &Vec<char>, kp: &Vec<Vec<char>>) -> Coords {
    let mut p = start;

    for d in dir {
        let mut next = match d {
            'U' => {
                (p.0, p.1 - 1)
            },
            'L' => {
                (p.0 - 1, p.1)
            },
            'D' => {
                (p.0, p.1 + 1)
            },
            'R' => {
                (p.0 + 1, p.1)
            },
            _ => panic!("Impossible!"),
        };

        next.0 = min(4, max(next.0, 0));
        next.1 = min(4, max(next.1, 0));
        if kp[next.1 as usize][next.0 as usize] != ' ' {
            p = next;
        }
    }

    p
}

#[aoc(day2, part2)]
fn problem2(input: &Vec<Vec<char>>) -> Result<String, ParseError> {
    let kp = keypad2();
    let mut pos = (0, 2);
    let mut code = vec![];
    for n in input {
        pos = trace2(pos, n, &kp);
        code.push(kp[pos.1 as usize][pos.0 as usize]);
    }
    Ok(code.iter().collect::<String>())
}
