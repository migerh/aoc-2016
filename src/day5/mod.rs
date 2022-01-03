use crate::utils::ParseError;

#[aoc(day5, part1)]
fn problem1(input: &str) -> Result<String, ParseError> {
    let mut passcode = vec![];

    for i in 0.. {
        let str = format!("{}{}", input, i);
        let hash = format!("{:x}", md5::compute(str.as_bytes()));

        if hash.chars().take(5).collect::<String>() == "00000" {
            let next = hash.chars().skip(5).next().ok_or(ParseError::new("Could not fetch next char"))?;
            passcode.push(next);
        }

        if passcode.len() == 8 {
            break;
        }
    }

    Ok(passcode.iter().collect::<String>())
}

#[aoc(day5, part2)]
fn problem2(input: &str) -> Result<String, ParseError> {
    let mut passcode = vec![' '; 8];

    for i in 0.. {
        let str = format!("{}{}", input, i);
        let hash = format!("{:x}", md5::compute(str.as_bytes()));

        if hash.chars().take(5).collect::<String>() == "00000" {
            let next = hash.chars().skip(6).next().ok_or(ParseError::new("Could not fetch next char"))?;
            if let Some(pos) = hash.chars().skip(5).next().ok_or(ParseError::new("Could not fetch next char pos"))?.to_digit(10) {
                if pos < 8 && passcode[pos as usize] == ' ' {
                    passcode[pos as usize] = next;
                }
            }
        }

        if !passcode.contains(&' ') {
            break;
        }
    }

    Ok(passcode.iter().collect::<String>())
}
