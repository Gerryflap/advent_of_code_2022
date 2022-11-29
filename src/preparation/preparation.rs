use crate::aoc_day;

pub struct DayPreparation {}

impl aoc_day::AocDay for DayPreparation {
    fn run(&self, inp: String) -> String {
        return "Henlo world".to_string();
    }

    fn get_path(&self) -> String{
        return "preparation".to_string();
    }
}