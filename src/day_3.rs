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
        let regex = Regex::new(r"do\(\)|mul\((\d+),(\d+)\)|don't\(\)").unwrap();
        let mut valid = true;
        regex
            .captures_iter(&data)
            .map(|x| {
                let m = x.get(0).unwrap().as_str();
                if m == "do()" {
                    valid = true;
                } else if m == "don't()" {
                    valid = false;
                }
                if valid {
                    x.get(1)
                        .map(|x_| x_.as_str())
                        .unwrap_or_else(|| "0")
                        .parse::<i32>()
                        .unwrap()
                        * x.get(2)
                            .map(|x_| x_.as_str())
                            .unwrap_or_else(|| "0")
                            .parse::<i32>()
                            .unwrap()
                } else {
                    0
                }
            })
            .sum::<i32>()
            .to_string()
    }
}
