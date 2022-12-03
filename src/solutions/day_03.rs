use std::collections::HashSet;
use itertools::Itertools;

use crate::aoc_day;

pub struct Day03;

impl aoc_day::AocDay for Day03 {
    fn run(&self, inp: &str) -> String {
        format!("q1: {}\nq2: {}", q1(inp), q2(inp))
    }

    fn get_path(&self) -> &str {
        "day_03"
    }
}

fn q1(inp: &str) -> u64 {
    inp.split('\n').map(get_common_char).map(compute_char_score).sum()
}

fn q2(inp: &str) -> u64 {
    inp.split('\n').chunks(3).into_iter().map(
        |chunk| chunk.map(
            |line| line.chars().collect::<HashSet<char>>()
        ).reduce(intersect)
        .unwrap().into_iter().next().unwrap()        
    ).map(|c| compute_char_score(c)).sum()
}

fn intersect(first: HashSet<char>, second: HashSet<char>) -> HashSet<char> {
    first.intersection(&second).map(|c| *c).collect::<HashSet<char>>()
}



fn get_common_char(inp_line: &str) -> char {
    let (part1, part2) = inp_line.split_at(inp_line.len()/2);
    let set1: HashSet<char> = part1.chars().collect::<HashSet<char>>();
    let set2: HashSet<char> = part2.chars().collect::<HashSet<char>>();
    let result: Option<&char> = (set1.intersection(&set2)).next();
    *result.unwrap_or_else(|| panic!("Could not find common character!!!"))    
}

fn compute_char_score(chr: char) -> u64 {
    if chr >= 'A' && chr <= 'Z' {
        return chr as u64 - 65 + 27;
    } else if chr >= 'a' && chr <= 'z' {
        return chr as u64 - 97 + 1;
    } else {
        panic!("Cannot parse character '{}'", chr);
    }
}