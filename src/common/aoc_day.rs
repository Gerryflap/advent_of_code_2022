use std::fs;

/**
 * Implementation of the common AoC day components
 */

// Enum for representing the input type. Used to switch between debug.txt and real.txt input files.
#[derive(PartialEq, Eq, Debug)]
pub enum InputType {
    Debug,
    Real,
}

// Runs the given AocDay on the given input type (real or debug) and returns the answer as a string
pub fn run(day: &dyn AocDay, i_type: InputType) -> String {
    let fname: &str = if i_type == InputType::Debug {
        "debug.txt"
    } else {
        "real.txt"
    };
    let fpath = "./src/input/".to_string() + day.get_path() + "/" + fname;
    fs::read_to_string(fpath).map_or_else(
        |e| format!("File Read Error!\n !{}", e),
        |s| day.run(&s),
    )
}

// Defines the required functions that have to be implemented for every advent of code day
pub trait AocDay {
    // Runs the code for that day, given a String and returns the String output
    fn run(&self, inp: &str) -> String;

    // Returns the folder path as seen from root, used for finding the input files
    fn get_path(&self) -> &str;
}
