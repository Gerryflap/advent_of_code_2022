use crate::aoc_day;

pub struct Day01;

impl aoc_day::AocDay for Day01 {
        fn run(&self, inp: String) -> String {
            // I need a fix for this lol, pointer time?
            let inpp: String = inp.clone();
            return format!("q1: {}\nq2: {}\n", q1(inp), q2(inpp));
        }

        fn get_path(&self) -> String {
            return "day_01".to_string();
        }
}

fn q1(inp: String) -> String {
    return format!("{}",
        inp.split("\n\n")
        .map(subseq_to_val)
        .max()
        .unwrap()
    )
        
}

fn q2(inp: String) -> String {
    let mut nums: Vec<i64> = inp.split("\n\n")
        .map(subseq_to_val)
        .collect();
    nums.sort_by(|a, b| b.cmp(a));
    let summed: i64 = nums[0..=2].iter().sum();
    return format!("{}", summed);        
}

fn subseq_to_val(subseq: &str) -> i64 {
    return subseq.split("\n")
        .filter_map(|s| s.parse::<i64>().ok())
        .sum()
}