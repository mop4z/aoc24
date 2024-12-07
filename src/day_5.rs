use regex::Regex;

use crate::Puzzle;

pub struct Day5;

#[derive(Debug)]
struct Rule {
    a: i32,
    b: i32,
}

impl Rule {
    fn applicable(&self, row: &[i32]) -> bool {
        row.contains(&self.a) && row.contains(&self.b)
    }
    fn valid_ordering(&self, row: &[i32]) -> bool {
        if let Some(idx_a) = row.iter().position(|x| x == &self.a) {
            if let Some(idx_b) = row.iter().position(|x| x == &self.b) {
                return idx_a < idx_b;
            }
        }
        false
    }
    fn swap_positions(&self, row: &mut [i32]) {
        if let Some(idx_a) = row.iter().position(|x| x == &self.a) {
            if let Some(idx_b) = row.iter().position(|x| x == &self.b) {
                row[idx_a] = self.b;
                row[idx_b] = self.a;
            }
        }
    }
}

fn rules(regex: &Regex, data: &String) -> Vec<Rule> {
    regex
        .captures_iter(data)
        .flat_map(|x| x.get(1))
        .flat_map(|x| x.as_str().split_once("|"))
        .map(|(a, b)| Rule {
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
        })
        .collect::<Vec<_>>()
}

fn sum_middle_rows(rows: impl Iterator<Item = Vec<i32>>) -> String {
    rows.map(|row| row[row.len() / 2]).sum::<i32>().to_string()
}

impl Puzzle for Day5 {
    fn day() -> u8 {
        5
    }
    fn part_a(data: String) -> String {
        let regex = Regex::new(r"(\d+\|\d+)|(\d+,?)+").unwrap();
        let rules = rules(&regex, &data);
        let rows = regex
            .captures_iter(&data)
            .filter_map(|x| {
                if x.get(2).is_some() {
                    x.get(0).map(|x_| {
                        x_.as_str()
                            .split(",")
                            .map(|page| page.parse::<i32>().unwrap())
                            .collect::<Vec<_>>()
                    })
                } else {
                    None
                }
            })
            .filter(|row| {
                rules
                    .iter()
                    .filter(|rule| rule.applicable(&row))
                    .all(|rule| rule.valid_ordering(&row))
            });
        sum_middle_rows(rows)
    }
    fn part_b(data: String) -> String {
        let regex = Regex::new(r"(\d+\|\d+)|(\d+,?)+").unwrap();
        let rules = rules(&regex, &data);
        let mut rows = regex
            .captures_iter(&data)
            .filter_map(|x| {
                if x.get(2).is_some() {
                    x.get(0).map(|x_| {
                        x_.as_str()
                            .split(",")
                            .map(|page| page.parse::<i32>().unwrap())
                            .collect::<Vec<_>>()
                    })
                } else {
                    None
                }
            })
            .filter(|row| {
                rules
                    .iter()
                    .filter(|rule| rule.applicable(&row))
                    .any(|rule| !rule.valid_ordering(&row))
            })
            .collect::<Vec<_>>();
        for row in rows.iter_mut() {
            let applicable_rules = rules
                .iter()
                .filter(|rule| rule.applicable(&row))
                .collect::<Vec<_>>();
            while applicable_rules
                .iter()
                .any(|rule| !rule.valid_ordering(row))
            {
                for rule in &applicable_rules {
                    if !rule.valid_ordering(row) {
                        rule.swap_positions(row);
                    }
                }
            }
        }
        sum_middle_rows(rows.into_iter())
    }
}
