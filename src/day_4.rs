use crate::Puzzle;

pub struct Day4;

const DIRECTIONS: [Direction; 8] = [
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
    Direction::Left,
    Direction::Right,
    Direction::DownLeft,
    Direction::Down,
    Direction::DownRight,
];

enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

impl Direction {
    fn travel(&self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Self::UpLeft => (x - 1, y - 1),
            Self::Up => (x, y - 1),
            Self::UpRight => (x + 1, y - 1),
            Self::Left => (x - 1, y),
            Self::Right => (x + 1, y),
            Self::DownLeft => (x - 1, y + 1),
            Self::Down => (x, y + 1),
            Self::DownRight => (x + 1, y + 1),
        }
    }
}

fn check_search_was_safe(letter_grid: &Vec<Vec<char>>, x: isize, y: isize) -> Option<char> {
    if x.is_negative() || y.is_negative() {
        None
    } else {
        if let Some(row) = letter_grid.get(y as usize) {
            if let Some(letter) = row.get(x as usize) {
                Some(*letter)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn search_for_remaining(
    letter_grid: &Vec<Vec<char>>,
    c: char,
    x: usize,
    y: usize,
    direction: Direction,
) -> bool {
    let (x_1, y_1) = direction.travel(x as isize, y as isize);
    if let Some(letter) = check_search_was_safe(letter_grid, x_1, y_1) {
        if c == letter {
            if letter == 'S' {
                true
            } else {
                search_for_remaining(
                    letter_grid,
                    match c {
                        'M' => 'A',
                        'A' => 'S',
                        _ => panic!(),
                    },
                    x_1 as usize,
                    y_1 as usize,
                    direction,
                )
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn search(letter_grid: &Vec<Vec<char>>, c: char, x: usize, y: usize, direction: Direction) -> bool {
    let (x_1, y_1) = direction.travel(x as isize, y as isize);
    if let Some(letter) = check_search_was_safe(letter_grid, x_1, y_1) {
        c == letter
    } else {
        false
    }
}

impl Puzzle for Day4 {
    fn day() -> u8 {
        4
    }
    fn part_a(data: String) -> String {
        let letter_grid = data
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        let mut total_count = 0;
        for y in 0..letter_grid.len() {
            for x in 0..(letter_grid[y].len()) {
                let letter = letter_grid[y][x];
                if letter == 'X' {
                    for direction in DIRECTIONS {
                        total_count +=
                            search_for_remaining(&letter_grid, 'M', x, y, direction) as i32;
                    }
                }
            }
        }
        total_count.to_string()
    }
    fn part_b(data: String) -> String {
        let letter_grid = data
            .lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        let mut total_count = 0;
        for y in 0..letter_grid.len() {
            for x in 0..(letter_grid[y].len()) {
                let letter = letter_grid[y][x];
                if letter == 'A' {
                    let search_cross = |locs: [(char, Direction); 2]| {
                        locs.map(|(c, d)| search(&letter_grid, c, x, y, d))
                            .iter()
                            .all(|x| *x)
                    };
                    let left_cross =
                        search_cross([('M', Direction::UpLeft), ('S', Direction::DownRight)])
                            || search_cross([
                                ('S', Direction::UpLeft),
                                ('M', Direction::DownRight),
                            ]);
                    if left_cross {
                        let right_cross =
                            search_cross([('M', Direction::UpRight), ('S', Direction::DownLeft)])
                                || search_cross([
                                    ('S', Direction::UpRight),
                                    ('M', Direction::DownLeft),
                                ]);
                        if right_cross {
                            total_count += 1;
                        }
                    }
                }
            }
        }
        total_count.to_string()
    }
}
