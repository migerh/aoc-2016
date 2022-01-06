use std::collections::HashMap;
use crate::utils::ParseError;
use rayon::prelude::*;

fn generate(salt: &str, index: usize) -> String {
    let str = format!("{}{}", salt, index);
    format!("{:x}", md5::compute(str.as_bytes()))
}

fn generate_stretched(salt: &str, index: usize) -> String {
    let str = format!("{}{}", salt, index);

    let mut hash = md5::compute(str.as_bytes());

    for _ in 0..2016 {
        hash = md5::compute(format!("{:x}", hash));
    }

    format!("{:x}", hash)
}

struct KeyGen {
    salt: String,
    v2: bool,
    cache: HashMap<usize, String>,
}

impl KeyGen {
    fn new(salt: &str) -> Self {
        let cache = HashMap::new();
        let salt = salt.to_string();
        let v2 = false;

        Self { cache, v2, salt }
    }

    fn v2_from_cache(salt: &str, map: HashMap<usize, String>) -> Self {
        let salt = salt.to_string();
        let v2 = true;

        Self { cache: map, v2, salt }
    }

    fn get(&mut self, index: usize) -> String {
        let salt = self.salt.as_str();
        let v2 = self.v2;

        if let Some(hash) = self.cache.get(&index) {
            hash.to_string()
        } else {
            self.cache.entry(index).or_insert(if v2 { generate_stretched(salt, index) } else { generate(salt, index) }).to_string()
        }
    }
}

fn is_key(gen: &mut KeyGen, index: usize) -> bool {
    let hash = gen.get(index);
    let mut c = None;

    for window in hash.chars().collect::<Vec<_>>().windows(3) {
        if window[0] == window[1] && window[1] == window[2] {
            c = Some(window[0]);
            break;
        }
    }

    if c.is_none() {
        return false;
    }

    let c = c.unwrap();
    for i in 1..=1000 {
        let hash = gen.get(index + i);
        for window in hash.chars().collect::<Vec<_>>().windows(5) {
            if window[0] == c && window[0] == window[1] && window[1] == window[2] && window[2] == window[3] && window[3] == window[4] {
                return true;
            }
        }
    }

    false
}

#[aoc(day14, part1)]
fn problem1(input: &str) -> Result<usize, ParseError> {
    let mut gen = KeyGen::new(input);
    let mut number_of_keys = 0;

    for index in 0.. {
        if is_key(&mut gen, index) {
            number_of_keys += 1;
        }

        if number_of_keys == 64 {
            return Ok(index);
        }
    }

    Ok(0)
}

#[aoc(day14, part2)]
fn problem2(input: &str) -> Result<usize, ParseError> {
    // upper limit determined by wrong solution that ran a lot longer :/
    let keys = (0..22500).collect::<Vec<_>>();
    let map = keys
        .par_iter()
        .map(|i| (*i, generate_stretched(input, *i)))
        .collect::<HashMap<_, _>>();
    let mut gen = KeyGen::v2_from_cache(input, map);

    let mut number_of_keys = 0;
    for index in 0.. {
        if is_key(&mut gen, index) {
            number_of_keys += 1;
        }

        if number_of_keys == 64 {
            return Ok(index);
        }
    }

    Ok(0)
}
