use regex::Regex;

use crate::Puzzle;

pub struct Day3;

impl Puzzle for Day3 {
    fn day() -> u8 {
        3
    }
    fn part_a(data: String) -> String {
        let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
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
        let do_regex = Regex::new(r"do\(\)").unwrap();
        let do_idxs = do_regex.find_iter(&data).map(|x| (x.start(), true));

        let dont_regex = Regex::new(r"don't\(\)").unwrap();
        let dont_idxs = dont_regex.find_iter(&data).map(|x| (x.start(), false));

        let mut valid_idxs = [(0 as usize, true)]
            .into_iter()
            .chain(do_idxs)
            .chain(dont_idxs)
            .collect::<Vec<_>>();
        valid_idxs.sort_by(|a, b| a.0.cmp(&b.0));
        let valid_idxs = valid_idxs.windows(2).collect::<Vec<_>>();

        let mul_regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        let mul_idxs = mul_regex.captures_iter(&data).map(|x| {
            let (_, [a, b]) = x.extract();
            (x.get(0).unwrap().start(), [a, b])
        });

        mul_idxs
            .filter(|(i, _)| {
                valid_idxs
                    .iter()
                    .find(|w| {
                        let is_valid = w[0].1;
                        let range = w[0].0..w[1].0;
                        is_valid && range.contains(&i)
                    })
                    .is_some()
            })
            .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
            .sum::<i32>()
            .to_string()
    }
}
