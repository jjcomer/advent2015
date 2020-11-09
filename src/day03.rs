use std::collections::hash_set::HashSet;

pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn translate(&self, direction: &Direction) -> Self {
        match direction {
            Direction::NORTH => Position::new(self.x, self.y + 1),
            Direction::SOUTH => Position::new(self.x, self.y - 1),
            Direction::EAST => Position::new(self.x + 1, self.y),
            Direction::WEST => Position::new(self.x - 1, self.y),
        }
    }

    fn key(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|c| match c {
            '^' => Direction::NORTH,
            '>' => Direction::EAST,
            '<' => Direction::WEST,
            _ => Direction::SOUTH,
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Direction>) -> i32 {
    let mut deliveries = HashSet::new();
    let location = Position { x: 0, y: 0 };
    deliveries.insert(location.key());

    input.iter().fold(location, |l, d| {
        let new_location = l.translate(&d);
        deliveries.insert(l.key());
        new_location
    });
    deliveries.len() as i32
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Direction>) -> i32 {
    let mut deliveries = HashSet::new();
    let santa_start_location = Position::new(0, 0);
    let robo_start_location = Position::new(0, 0);
    deliveries.insert(santa_start_location.key());
    input.iter().step_by(2).fold(santa_start_location, |l, d| {
        let new_location = l.translate(&d);
        deliveries.insert(l.key());
        new_location
    });
    input
        .iter()
        .skip(1)
        .step_by(2)
        .fold(robo_start_location, |l, d| {
            let new_location = l.translate(&d);
            deliveries.insert(l.key());
            new_location
        });
    deliveries.len() as i32
}
