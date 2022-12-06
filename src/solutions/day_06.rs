use std::collections::{LinkedList, HashSet};

use crate::aoc_day;

pub struct Day06;

impl aoc_day::AocDay for Day06 {
    fn run(&self, inp: &str) -> String {
        format!("q1: {}\nq2: {}", q1(inp), q2(inp))
    }

    fn get_path(&self) -> &str {
        "day_06"
    }
}

fn q1(inp: &str) -> usize {
    let last_4: &mut LinkedList<char> = &mut LinkedList::new();
    let mut solution: usize = 0;
    for i in 0..inp.len() {
        if append_and_check_q1(last_4, inp.chars().nth(i).unwrap()) {
            solution = i + 1;
            break;
        }
    }
    solution
}

fn append_and_check_q1(ll: &mut LinkedList<char>, chr: char) -> bool {
    ll.push_front(chr);
    if ll.len() > 4 {
        ll.pop_back();
    }
    ll.iter().collect::<HashSet<&char>>().len() == 4
}

fn q2(inp: &str) -> usize {
    let last_14: &mut LinkedList<char> = &mut LinkedList::new();
    let mut solution: usize = 0;
    for i in 0..inp.len() {
        if append_and_check_q2(last_14, inp.chars().nth(i).unwrap()) {
            solution = i + 1;
            break;
        }
    }
    solution
}

fn append_and_check_q2(ll: &mut LinkedList<char>, chr: char) -> bool {
    ll.push_front(chr);
    if ll.len() > 14 {
        ll.pop_back();
    }
    ll.iter().collect::<HashSet<&char>>().len() == 14
}