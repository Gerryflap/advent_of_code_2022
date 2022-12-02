use crate::aoc_day;

pub struct Day02;

impl aoc_day::AocDay for Day02 {
    fn run(&self, inp: &str) -> String {
        format!("q1: {}\nq2: {}", q1(inp), q2(inp))
    }

    fn get_path(&self) -> &str{
        "day_02"
    }
}

fn q1(inp: &str) -> i64 {
    inp.split('\n').map(|l| outcome_points(l) + choice_points(l)).sum()
}

fn q2(inp: &str) -> i64 {
    inp.split('\n').map(q2_to_q1_converter)
    .map(|l| outcome_points(l) + choice_points(l)).sum()
}

fn outcome_points(inp_line: &str) -> i64 {
    match inp_line {
        "A X" => 3,
        "A Y" => 6,
        "A Z" => 0,
        "B X" => 0,
        "B Y" => 3,
        "B Z" => 6,
        "C X" => 6,
        "C Y" => 0,
        "C Z" => 3,
        x => panic!("Got unexpected input {}", x),
    }
}

fn choice_points(inp_line: &str) -> i64 {
    let char: Option<char> = inp_line.chars().nth(2);
    match char {
        Some('X') => 1,
        Some('Y') => 2,
        Some('Z') => 3,
        Some(x) => panic!("Could not parse character '{}'", x),
        None => panic!("Got too short line '{}'", inp_line)
    }
}

fn q2_to_q1_converter(inp_line: &str) -> &str{
    match inp_line {
        "A X" => "A Z",
        "A Y" => "A X",
        "A Z" => "A Y",
        "B X" => "B X",
        "B Y" => "B Y",
        "B Z" => "B Z",
        "C X" => "C Y",
        "C Y" => "C Z",
        "C Z" => "C X",
        x => panic!("Got unexpected input {}", x),
    }
}