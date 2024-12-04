use std::{fs, io, path::Path};

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

pub enum Part {
    A,
    B,
}

pub trait Puzzle {
    fn get_data() -> io::Result<String> {
        let path = format!("input\\{}.txt", Self::day());
        fs::read_to_string(Path::new(&path))
    }
    fn solve(input: Part) -> io::Result<String> {
        let data = Self::get_data()?;
        let result = match input {
            Part::A => Self::part_a(data),
            Part::B => Self::part_b(data),
        };
        Ok(result)
    }
    fn day() -> u8;
    fn part_a(data: String) -> String;
    fn part_b(data: String) -> String;
}

pub fn solve<T: Puzzle>() {
    println!(
        "{}: A={:?} B={:?}",
        T::day(),
        T::solve(Part::A),
        T::solve(Part::B)
    );
}
