use crate::aoc_day;

pub struct Day01;

impl aoc_day::AocDay for Day01 {
    fn run(&self, inp: &str) -> String {
        format!("q1: {}\nq2: {}\n", q1(inp), q2(inp))
    }

    fn get_path(&self) -> &str {
        "day_01"
    }
}

fn q1(inp: &str) -> String {
    format!("{}", inp.split("\n\n").map(subseq_to_val).max().unwrap())
}

fn q2(inp: &str) -> String {
    let mut nums: Vec<i64> = inp.split("\n\n").map(subseq_to_val).collect();
    nums.sort_by(|a, b| b.cmp(a));
    let summed: i64 = nums[0..=2].iter().sum();
    format!("{}", summed)
}

fn subseq_to_val(subseq: &str) -> i64 {
    return subseq
        .split('\n')
        .filter_map(|s| s.parse::<i64>().ok())
        .sum();
}
