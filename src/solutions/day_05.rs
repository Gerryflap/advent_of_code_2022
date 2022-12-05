use std::collections::LinkedList;

use lazy_static::lazy_static;
use itertools::Itertools;
use std::str::Split;
use regex::Regex;

use crate::aoc_day;

pub struct Day05;

impl aoc_day::AocDay for Day05 {
    fn run(&self, inp: &str) -> String {
        format!("q1: \"{}\"\nq2: \"{}\"", q1(inp), q2(inp))
    }

    fn get_path(&self) -> &str {
        "day_05"
    }
}

fn q1(inp: &str) -> String {
    let mut stacks: Vec<LinkedList<char>> = parse_input(inp);
    for cmd in inp.lines().filter_map(parse_command_line) {
        execute_command(&cmd, &mut stacks);
    }
    stacks.iter().map(|stack| stack.front().unwrap().to_owned()).collect()
}

fn q2(inp: &str) -> String {
    let mut stacks: Vec<LinkedList<char>> = parse_input(inp);
    for cmd in inp.lines().filter_map(parse_command_line) {
        execute_command_q2(&cmd, &mut stacks);
    }
    stacks.iter().map(|stack| stack.front().unwrap().to_owned()).collect()
}


// ============== Parsing crates into the stacks ====================
fn parse_input(inp: &str) -> Vec<LinkedList<char>> {
    let lines: Split<char> = inp.split('\n');
    let (bottom_index, number_line) = inp.split('\n').find_position(|line| line.starts_with(" 1")).unwrap();
    let n_columns: usize = number_line.split_whitespace().filter_map(|n| n.parse::<usize>().ok()).max().unwrap();
    let mut stacks: Vec<LinkedList<char>> = vec![];
    let mut ll: LinkedList<char>;
    for _col in 0..n_columns {
        ll = LinkedList::new();
        stacks.push(ll);
    }
    fill_stacks_from_str(&mut stacks, lines, bottom_index);
    return stacks;
}

fn fill_stacks_from_str(stacks: &mut Vec<LinkedList<char>>, lines: Split<char>, bottom_index: usize) {
    // Fill stacks from the bottom/back with all the crates in the input
    lines.take(bottom_index).for_each(|l| parse_line_of_crates_and_add_to_stack(0, l, stacks));

}

fn parse_line_of_crates_and_add_to_stack(i :usize, line: &str, stacks: &mut Vec<LinkedList<char>>) {
    if line.len() >= 3 {
        if line.starts_with('[') {
            stacks[i].push_back(line.chars().nth(1).unwrap());
        }
        if line.len() >= 4 {
            parse_line_of_crates_and_add_to_stack(i+1, &line[4..], stacks)
        }
    }
}

// ============== Parsing commands and executing them on the stacks ====================

// Commands
struct MoveCommand {
    amount: usize,
    // From and To are 1-indexed in this struct, to match the input strings. The actual stacks are 0-indexed to match Rust
    from: usize,
    to: usize, 
}

fn parse_command_line(line: &str) -> Option<MoveCommand> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    RE.captures(line).map(
        |capt| MoveCommand {
            amount: capt[1].parse().unwrap(), 
            from: capt[2].parse().unwrap(), 
            to: capt[3].parse().unwrap()
        }
    )
}

fn execute_command(cmd: &MoveCommand, stacks: &mut Vec<LinkedList<char>>) {
    for _i in 0..cmd.amount {
        let c: char = stacks[cmd.from - 1].pop_front().unwrap();
        stacks[cmd.to - 1].push_front(c);
    }
}

fn execute_command_q2(cmd: &MoveCommand, stacks: &mut Vec<LinkedList<char>>) {
    let vec: &mut Vec<char> = &mut vec!();
    for _i in 0..cmd.amount {
        vec.push(stacks[cmd.from - 1].pop_front().unwrap());
    }
    vec.reverse();
    vec.iter().for_each(|c| stacks[cmd.to - 1].push_front(c.to_owned()))
}