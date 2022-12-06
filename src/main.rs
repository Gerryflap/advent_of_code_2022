use std::env;
mod common;
use common::aoc_day;
mod solutions;
use solutions::day_06;

/**
 * Main function, for running the code
 */

fn main() {
    // vvvvvv Change the day here vvvvvv
    let day: &dyn aoc_day::AocDay = &day_06::Day06 {};
    // ^^^^^^ Change the day here ^^^^^^

    // Parse the input type (real input or debug) that should be used
    let args: Vec<String> = env::args().collect();
    let input_type: aoc_day::InputType;

    if args.len() <= 1 || args[1] != "--debug" {
        input_type = aoc_day::InputType::Real;
        println!("No --debug provided, attempting to use real input")
    } else {
        input_type = aoc_day::InputType::Debug;
        println!("Debug mode: using debug input")
    }
    println!("Input mode: {:?}", input_type);

    // Compute the answer and print the result
    let answer = aoc_day::run(day, input_type);
    println!();
    println!("Result:");
    println!("{}", answer)
}
