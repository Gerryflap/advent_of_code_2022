use std::fs;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum InputType {
    Debug,
    Real
}

pub fn run(day: &dyn AocDay, i_type: InputType) -> String {
    let fname: &str = if i_type == InputType::Debug {"debug.txt"} else {"real.txt"};
    let fpath = "./src/".to_string() + &day.get_path() + "/" + fname;
    return fs::read_to_string(fpath)
        .map_or_else(
            |e| format!("File Read Error!\n !{}", e.to_string()),
            |s| day.run(s)
        );
}

pub trait AocDay {
    // Runs the code for that day, given a String and returns the String output
    fn run(&self, inp: String) -> String;

    // Returns the folder path as seen from root, used for finding the input files
    fn get_path(&self) -> String;
}