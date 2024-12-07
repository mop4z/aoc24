use regex::Regex;

use crate::Puzzle;

pub struct Day3;

impl Puzzle for Day3 {
    fn day() -> u8 {
        3
    }
    fn part_a(data: String) -> String {
        let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        regex
            .captures_iter(&data)
            .map(|x| {
                let (_, [a, b]) = x.extract();
                a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
            })
            .sum::<i32>()
            .to_string()
    }
    fn part_b(data: String) -> String {
        let regex = Regex::new(r"(do\(\))|mul\((\d+),(\d+)\)|(don't\(\))").unwrap();
        let mut valid = true;
        regex
            .captures_iter(&data)
            .map(|x| {
                if x.get(1).is_some() {
                    valid = true;
                } else if x.get(4).is_some() {
                    valid = false;
                }
                if valid {
                    x.get(2)
                        .map(|x_| x_.as_str().parse::<i32>().unwrap())
                        .and_then(|a| x.get(3).map(|x_| a * x_.as_str().parse::<i32>().unwrap()))
                        .unwrap_or_else(|| 0)
                } else {
                    0
                }
            })
            .sum::<i32>()
            .to_string()
    }
}
