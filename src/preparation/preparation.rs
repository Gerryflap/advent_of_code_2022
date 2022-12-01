use crate::aoc_day;
use std::iter::zip;

/**
 * Implementation for 2021 Day 01, part one. Done as an example for myself.
 */

 // Empty struct, used to represent this day as an object. 
pub struct DayPreparation;

// Implement the trait AocDay for this advent of code day, which allows us to easily run it.
impl aoc_day::AocDay for DayPreparation {
    fn run(&self, inp: &str) -> String {
        format!("{}", count_increases(parse_lines(inp)))
    }

    fn get_path(&self) -> &str{
        "preparation"
    }
}

// Parses lines into a vector of ints
fn parse_lines(inp: &str) -> Vec<i64> {
    return inp.split('\n')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
}

// Counts the number of times where the depth increases compared to the previous measurement
fn count_increases(inp: Vec<i64>) -> usize {
    return zip(inp[1..].iter(), inp.iter())
            .filter(|(a, b)| b < a)
            .count();
}