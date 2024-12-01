use std::collections::HashMap;

use crate::Puzzle;

pub struct Day1;

fn get_lists(data: String) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for mut a_b in data.lines().map(|x| x.split_whitespace()) {
        let a = a_b.next().map(|x| x.parse());
        let b = a_b.next().map(|x| x.parse());
        match (a, b) {
            (Some(Ok(a)), Some(Ok(b))) => {
                left.push(a);
                right.push(b);
            }
            _ => {}
        }
    }
    (left, right)
}

impl Puzzle for Day1 {
    fn day() -> u8 {
        1
    }
    fn part_a(data: String) -> String {
        let (mut left, mut right) = get_lists(data);
        left.sort();
        right.sort();
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| (b - a).abs())
            .sum::<i32>()
            .to_string()
    }
    fn part_b(data: String) -> String {
        let (left, right) = get_lists(data);
        let mut map: HashMap<i32, (usize, usize)> = HashMap::new();
        for l_i in left.iter() {
            if !map.contains_key(l_i) {
                let l_count = left.iter().filter(|x| *x == l_i).count();
                let r_count = right.iter().filter(|x| *x == l_i).count();
                map.insert(*l_i, (l_count, r_count));
            }
        }
        map.iter()
            .map(|(k, (v_1, v_2))| k * *v_1 as i32 * *v_2 as i32)
            .sum::<i32>()
            .to_string()
    }
}
