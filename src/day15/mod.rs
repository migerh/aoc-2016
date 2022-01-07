use crate::utils::ParseError;

#[derive(Debug, Clone)]
struct Crt {
    remainder: usize,
    modulus: usize,
}

impl Crt {
    fn new(remainder: isize, modulus: usize) -> Self {
        let remainder = (remainder % (modulus as isize) + modulus as isize) as usize;

        Self { remainder, modulus }
    }
}

// stolen from my 2020/13 solution
fn chinese_remainder(crts: &Vec<Crt>) -> usize {
    // search the solution with the chinese remainder theorem
    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving

    let mut time = crts[0].remainder;
    let mut increment = crts[0].modulus;

    for crt in crts.iter().skip(1) {
        loop {
            if time % crt.modulus == crt.remainder {
                break;
            }

            time += increment;
        }

        increment *= crt.modulus;
    }

    time
}

#[aoc_generator(day15)]
fn get_input(_input: &str) -> Result<Vec<Crt>, ParseError> {
    let start = [0, 0, 2, 2, 0, 7];
    let modulus = [7, 13, 3, 5, 17, 19];

    Ok(start.into_iter().enumerate()
        .map(|(i, v)| {
            let start = -v - (i as isize + 1);
            Crt::new(start, modulus[i])
        })
        .collect::<Vec<_>>())
}

#[aoc(day15, part1)]
fn problem1(input: &Vec<Crt>) -> Result<usize, ParseError> {
    let time = chinese_remainder(input);
    Ok(time)
}

#[aoc(day15, part2)]
fn problem2(input: &Vec<Crt>) -> Result<usize, ParseError> {
    let mut input = input.clone();
    input.push(Crt::new(-7, 11));

    let time = chinese_remainder(&input);
    Ok(time)
}
