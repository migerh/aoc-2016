use std::collections::HashMap;
use regex::Regex;
use std::str::FromStr;
use crate::utils::ParseError;

#[derive(Clone, Debug)]
enum Recipient {
    Output(usize),
    Bot(usize),
}

impl FromStr for Recipient {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^(?P<what>.*?) (?P<id>\d+)$").unwrap();
        }

        let (what, id) = RE.captures(s).and_then(|cap| {
            let what = cap.name("what").map(|v| v.as_str())?;
            let id = cap.name("id").map(|v| v.as_str().parse::<usize>())?.ok()?;

            Some((what, id))
        }).ok_or(ParseError::new(&format!("Error parsing recipient {}", s)))?;

        let result = match what {
            "bot" => Recipient::Bot(id),
            "output" => Recipient::Output(id),
            _ => Err(ParseError::new("Unknown recipient type"))?,
        };

        Ok(result)
    }
}

#[derive(Clone)]
struct Command {
    bot: usize,
    high: Recipient,
    low: Recipient,
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^bot (?P<bot>\d+) gives low to (?P<low>.*) and high to (?P<high>.*)$").unwrap();
        }

        let (bot, low, high) = RE.captures(s).and_then(|cap| {
            let low = cap.name("low").map(|v| v.as_str())?;
            let low = Recipient::from_str(low).ok()?;

            let high = cap.name("high").map(|v| v.as_str())?;
            let high = Recipient::from_str(high).ok()?;

            let bot = cap.name("bot").map(|v| v.as_str().parse::<usize>())?.ok()?;


            Some((bot, low, high))
        }).ok_or(ParseError::new(&format!("Error parsing command {}", s)))?;

        Ok(Self { bot, low, high })
    }
}

#[derive(Clone)]
struct Bot {
    values: Vec<usize>,
    command: Option<Command>,
}

impl Bot {
    fn new() -> Self {
        let values = vec![];
        let command = None;

        Self { values, command }
    }

    fn receive(&mut self, value: usize) {
        self.values.push(value);
    }

    fn set_command(&mut self, command: Command) {
        self.command = Some(command);
    }

    fn ready(&self) -> bool {
        self.values.len() >= 2
    }

    fn run(&mut self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
        self.values.sort();

        let mut handover = vec![];
        let mut output = vec![];
        if let Some(c) = &self.command {
            if let Recipient::Bot(id) = c.low {
                handover.push((id, self.values[0]));
            }

            if let Recipient::Output(id) = c.low {
                output.push((id, self.values[0]));
            }

            if let Recipient::Bot(id) = c.high {
                handover.push((id, self.values[1]));
            }

            if let Recipient::Output(id) = c.high {
                output.push((id, self.values[1]));
            }
        }

        self.values = vec![];
        (handover, output)
    }
}

type BotArmy = HashMap<usize, Bot>;

fn parse_value(s: &str) -> Result<(usize, usize), ParseError> {
    let value = s.split(" ").skip(1).next().map(|v| v.parse::<usize>()).ok_or(ParseError::new("Could not determine value"))??;
    let bot_id = s.split(" ").skip(5).next().map(|v| v.parse::<usize>()).ok_or(ParseError::new("Could not read bot id"))??;

    Ok((bot_id, value))
}

#[aoc_generator(day10)]
fn get_input(input: &str) -> Result<BotArmy, ParseError> {
    let mut army = HashMap::new();
    for line in input.lines() {
        if line.starts_with("value") {
            let (bot_id, value) = parse_value(line)?;
            let bot = army.entry(bot_id).or_insert(Bot::new());
            bot.receive(value);
        }

        if line.starts_with("bot") {
            let command = Command::from_str(line)?;
            let bot = army.entry(command.bot).or_insert(Bot::new());
            bot.set_command(command);
        }
    }

    Ok(army)
}

#[aoc(day10, part1)]
fn problem1(input: &BotArmy) -> Result<usize, ParseError> {
    let mut army = input.clone();

    loop {
        let mut executed_commands = 0;

        let bot_ids = army.iter().map(|(id, _)| id).cloned().collect::<Vec<_>>();
        for i in bot_ids {
            let bot = army.get_mut(&i).ok_or(ParseError::new("Bot not found"))?;

            if bot.values.contains(&61) && bot.values.contains(&17) {
                return Ok(i);
            }

            if bot.ready() {
                executed_commands += 1;
                let handover = bot.run().0;

                for (id, value) in handover {
                    let other = army.entry(id).or_insert(Bot::new());
                    other.receive(value);
                }
            }
        }

        if executed_commands == 0 {
            break;
        }
    }

    Ok(0)
}

#[aoc(day10, part2)]
fn problem2(input: &BotArmy) -> Result<usize, ParseError> {
    let mut army = input.clone();
    let mut output = HashMap::new();

    loop {
        let mut executed_commands = 0;

        let bot_ids = army.iter().map(|(id, _)| id).cloned().collect::<Vec<_>>();
        for i in bot_ids {
            let bot = army.get_mut(&i).ok_or(ParseError::new("Bot not found"))?;

            if bot.ready() {
                executed_commands += 1;
                let handover = bot.run();

                for (id, value) in handover.0 {
                    let other = army.entry(id).or_insert(Bot::new());
                    other.receive(value);
                }

                for (id, value) in handover.1 {
                    output.entry(id).and_modify(|v: &mut Vec<usize>| v.push(value)).or_insert(vec![value]);
                }
            }
        }

        if executed_commands == 0 {
            break;
        }
    }

    Ok(output.iter().filter(|(k, _)| **k == 0 || **k == 1 || **k == 2).map(|(_, v)| v[0]).product())
}
