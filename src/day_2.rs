use std::ops::Range;

use crate::Puzzle;

pub struct Day2;

const MAX_CHANGE: Range<i32> = 1..4;

fn line_is_safe(line: &str, dampener: bool) -> bool {
    let levels: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .flatten()
        .collect();
    if !dampener {
        let mut previous = levels[0];
        let mut direction = None;
        for level in &levels[1..] {
            if !check_level(previous, *level, &mut direction) {
                return false;
            }
            previous = *level;
        }
        true
    } else {
        'fail_loop: for i in 0..levels.len() {
            let mut levels_iter = levels.iter().enumerate().filter(|(i_, _)| i != *i_);
            let (_, mut previous) = levels_iter.next().unwrap();
            let mut direction = None;
            for (_, level) in levels_iter {
                if !check_level(*previous, *level, &mut direction) {
                    continue 'fail_loop;
                }
                previous = level;
            }
            return true;
        }
        false
    }
}

fn check_level(previous: i32, level: i32, direction: &mut Option<bool>) -> bool {
    let diff = level - previous;
    let new_direction = diff.is_positive();
    if direction.is_none() {
        *direction = Some(new_direction);
    }
    if !MAX_CHANGE.contains(&diff.abs()) || direction.is_some_and(|x| x != new_direction) {
        return false;
    }
    true
}

impl Puzzle for Day2 {
    fn day() -> u8 {
        2
    }
    fn part_a(data: String) -> String {
        data.lines()
            .filter(|x| line_is_safe(x, false))
            .count()
            .to_string()
    }
    fn part_b(data: String) -> String {
        data.lines()
            .filter(|x| {
                let is_safe = line_is_safe(x, true);
                println!("{x} -> {is_safe}");
                is_safe
            })
            .count()
            .to_string()
    }
}
