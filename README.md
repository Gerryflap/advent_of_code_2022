# Advent of Code 2022
And this time, it's time to learn some Rust 🦀🦀🦀.

## How to use
For now, change the "day" variable in the main.rs file (and imports) to run the respective day, then run `cargo run` in the terminal from the root folder.
Using `cargo run -- --debug` will run that day with the debug input (often given with the question) for verification.
Of course you can also `cargo build` and then execute the executable in `target/debug/advent_of_code`.

## General idea of this repo
Every day will become a separate module, with a struct implementing the trait `AocDay` from `common`. 
The files `real.txt` (for the actual big input string) and `debug.txt` (for the example given) are expected to be supplied, containing the required input strings.