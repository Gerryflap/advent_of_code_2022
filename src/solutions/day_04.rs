use crate::aoc_day;

pub struct Day04;

impl aoc_day::AocDay for Day04 {
    fn run(&self, inp: &str) -> String {
        format!("q1: {}\nq2: {}", q1(inp), q2(inp))
    }

    fn get_path(&self) -> &str {
        "day_04"
    }
}

fn q1(inp: &str) -> usize {
    inp.split('\n')
        .filter(|&line| q1_line_ranges_fully_contain(line))
        .count()
}

fn q2(inp: &str) -> usize {
    inp.split('\n')
        .filter(|&line| q2_line_ranges_overlap(line))
        .count()
}

struct Range {
    start: i64,
    end: i64,
}

// Tests whether range a fully contains range b
fn fully_contains(a: &Range, b: &Range) -> bool {
    a.start <= b.start && a.end >= b.end
}

// Tests whether range a fully contains range b, or the other way around
fn bidirectional_fully_contains(a: &Range, b: &Range) -> bool {
    fully_contains(a, b) || fully_contains(b, a)
}

// Parses a string "<num>-<num>" to a range. Panics if there is no such string
fn parse_range(text: &str) -> Range {
    let mut range_nums = text.split('-').map(|s| s.parse::<i64>().unwrap());
    Range {
        start: range_nums.next().unwrap(),
        end: range_nums.next().unwrap(),
    }
}

fn q1_line_ranges_fully_contain(line: &str) -> bool {
    let mut ranges = line.split(',').map(parse_range);
    bidirectional_fully_contains(&ranges.next().unwrap(), &ranges.next().unwrap())
}

// Whether range a an range b have some overlap
fn overlap(a: &Range, b: &Range) -> bool {
    bidirectional_fully_contains(a, b)
        || (a.start >= b.start && a.start <= b.end)
        || (b.start >= a.start && b.start <= a.end)
}

fn q2_line_ranges_overlap(line: &str) -> bool {
    let mut ranges = line.split(',').map(parse_range);
    overlap(&ranges.next().unwrap(), &ranges.next().unwrap())
}
