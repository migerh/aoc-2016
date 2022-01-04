use std::collections::VecDeque;
use std::fmt::Display;
use crate::utils::ParseError;

#[derive(PartialEq)]
enum State {
    Parsing,
    Hypernet,
}

#[derive(Debug)]
struct Ip {
    ip: String,
}

impl Ip {
    fn new(ip: &str) -> Self {
        Ip { ip: ip.to_string() }
    }
}

impl Display for Ip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(self.ip.as_str())
    }
}

#[aoc_generator(day7)]
fn get_input(input: &str) -> Result<Vec<Ip>, ParseError> {
    Ok(input.lines().map(|l| Ip::new(l)).collect::<Vec<_>>())
}

impl Ip {
    fn supports_tls(&self) -> bool {
        use State::*;

        let mut state = Parsing;
        let mut last_four = VecDeque::new();
        let mut supports_in_parsing = false;
        for c in self.ip.chars() {
            if c == '[' {
                state = Hypernet;
                last_four.clear();
            }

            if c == ']' {
                state = Parsing;
                last_four.clear();
            }

            last_four.push_back(c);
            if last_four.len() > 4 {
                last_four.pop_front();
            }

            if last_four.len() == 4 {
                if last_four[0] == last_four[3] && last_four[1] == last_four[2] && last_four[0] != last_four[1] {
                    if state == Parsing {
                        supports_in_parsing = true;
                    }
                    if state == Hypernet {
                        return false;
                    }
                }
            }
        }

        supports_in_parsing
    }
}

#[aoc(day7, part1)]
fn problem1(input: &Vec<Ip>) -> Result<usize, ParseError> {
    let result = input.iter().filter(|i| i.supports_tls()).count();
    Ok(result)
}

#[derive(Debug)]
struct Triplet {
    outer: char,
    inner: char,
}

impl Triplet {
    fn new(outer: char, inner: char) -> Self {
        Self { outer, inner }
    }

    fn matches(&self, o: &Triplet) -> bool {
        self.outer == o.inner && self.inner == o.outer && self.outer != o.outer
    }
}

impl Ip {
    fn collect_triplets(&self) -> (Vec<Triplet>, Vec<Triplet>) {
        use State::*;

        let mut supernet = vec![];
        let mut hypernet = vec![];

        let mut state = Parsing;

        let mut last_three = VecDeque::new();
        for c in self.ip.chars() {
            if c == '[' {
                state = Hypernet;
                last_three.clear();
            }

            if c == ']' {
                state = Parsing;
                last_three.clear();
            }

            last_three.push_back(c);
            if last_three.len() > 3 {
                last_three.pop_front();
            }

            if last_three.len() == 3 {
                if last_three[0] == last_three[2] && last_three[0] != last_three[1] {
                    let triplet = Triplet::new(last_three[0], last_three[1]);
                    if state == Parsing {
                        supernet.push(triplet);
                    } else {
                        hypernet.push(triplet);
                    }
                }
            }
        }

        (supernet, hypernet)
    }

    fn supports_ssl(&self) -> bool {
        let (supernet, hypernet) = self.collect_triplets();

        for s in supernet {
            for h in &hypernet {
                if s.matches(h) {
                    return true;
                }
            }
        }

        false
    }
}

#[aoc(day7, part2)]
fn problem2(input: &Vec<Ip>) -> Result<usize, ParseError> {
    let result = input.iter().filter(|i| i.supports_ssl()).count();
    Ok(result)
}
