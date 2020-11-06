use std::cmp::min;

pub struct Dimensions {
    length: i32,
    width: i32,
    height: i32,
}

impl From<&str> for Dimensions {
    fn from(input: &str) -> Self {
        let mut nums = input.split("x");

        Dimensions {
            length: nums.next().unwrap().parse::<i32>().unwrap(),
            width: nums.next().unwrap().parse::<i32>().unwrap(),
            height: nums.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}

impl Dimensions {
    fn smallest_area(&self) -> i32 {
        min(
            self.length * self.width,
            min(self.width * self.height, self.height * self.length),
        )
    }

    fn surface_area(&self) -> i32 {
        2 * (self.length * self.width + self.width * self.height + self.height * self.length)
    }

    fn volume(&self) -> i32 {
        self.length * self.width * self.height
    }

    fn shortest_distance_around(&self) -> i32 {
        if self.length >= self.width && self.length >= self.height {
            2 * (self.width + self.height)
        } else if self.width >= self.length && self.width >= self.height {
            2 * (self.length + self.height)
        } else {
            2 * (self.length + self.width)
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Dimensions> {
    let mut dimensions = Vec::<Dimensions>::new();
    for line in input.lines() {
        dimensions.push(line.into());
    }
    dimensions
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Dimensions>) -> i32 {
    input
        .iter()
        .map(|d| d.smallest_area() + d.surface_area())
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Dimensions>) -> i32 {
    input
        .iter()
        .map(|d| d.shortest_distance_around() + d.volume())
        .sum()
}
