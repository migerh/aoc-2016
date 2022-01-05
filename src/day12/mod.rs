use std::collections::HashMap;
use std::str::FromStr;
use crate::utils::ParseError;

fn first(s: &str) -> Result<char, ParseError> {
    s.chars().next().ok_or(ParseError::new("Empty string"))
}

enum Param {
    Register(char),
    Value(isize),
}

impl FromStr for Param {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let f = first(s)?;

        Ok(if s.len() == 1 && f.is_alphabetic() {
            Param::Register(f)
        } else {
            Param::Value(s.parse::<isize>()?)
        })
    }
}

enum Instruction {
    Cpy((Param, char)),
    Inc(char),
    Dec(char),
    Jnz((Param, Param)),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let op = s.split(" ").next().ok_or(ParseError::new("Empty string"))?;
        let params = s.split(" " ).skip(1).collect::<Vec<_>>();

        Ok(match op {
            "cpy" => {
                if params.len() != 2 {
                    Err(ParseError::new("Invalid number of params"))?;
                }

                Instruction::Cpy((Param::from_str(params[0])?, first(params[1])?))
            },
            "inc" => {
                if params.len() != 1 {
                    Err(ParseError::new("Invalid number of params"))?;
                }

                Instruction::Inc(first(params[0])?)
            },
            "dec" => {
                if params.len() != 1 {
                    Err(ParseError::new("Invalid number of params"))?;
                }

                Instruction::Dec(first(params[0])?)
            },
            "jnz" => {
                if params.len() != 2 {
                    Err(ParseError::new("Invalid number of params"))?;
                }

                Instruction::Jnz((Param::from_str(params[0])?, Param::from_str(params[1])?))
            },
            _ => Err(ParseError::new(&format!("Invalid op code '{}'", op)))?
        })
    }
}

#[aoc_generator(day12)]
fn get_input(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input
        .lines()
        .map(|l| Instruction::from_str(l))
        .collect::<Result<Vec<_>, ParseError>>()
}

struct AssemBunny {
    registers: HashMap<char, isize>,
}

impl AssemBunny {
    fn new() -> Self {
        let registers = HashMap::new();

        Self { registers }
    }
    fn set(&mut self, register: &char, value: isize) {
        self.registers.entry(*register).and_modify(|v| *v = value).or_insert(value);
    }

    fn get(&self, register: &char) -> isize {
        *self.registers.get(&register).unwrap_or(&0)
    }

    fn resolve(&self, p: &Param) -> isize {
        match p {
            Param::Value(v) => *v,
            Param::Register(r) => *self.registers.get(&r).unwrap_or(&0),
        }
    }

    fn run(&mut self, instructions: &Vec<Instruction>) {
        use Instruction::*;

        let mut ip = 0;
        while ip < instructions.len() {
            let i = &instructions[ip];
            match i {
                Cpy((x, y)) => {
                    self.set(y, self.resolve(x));
                },
                Inc(x) => {
                    self.set(x, self.get(x) + 1);
                },
                Dec(x) => {
                    self.set(x, self.get(x) - 1);
                },
                Jnz((x, y)) => {
                    let x = self.resolve(x);
                    let y = self.resolve(y);
                    if x != 0 {
                        ip = ((ip as isize) + y) as usize;
                        continue;
                    }
                },
            }

            ip += 1;
        }
    }
}

#[aoc(day12, part1)]
fn problem1(input: &Vec<Instruction>) -> Result<isize, ParseError> {
    let mut ab = AssemBunny::new();
    ab.run(input);

    Ok(ab.get(&'a'))
}

#[aoc(day12, part2)]
fn problem2(input: &Vec<Instruction>) -> Result<isize, ParseError> {
    let mut ab = AssemBunny::new();
    ab.set(&'c', 1);
    ab.run(input);

    Ok(ab.get(&'a'))
}
