#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut floor = 0;

    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }

    floor
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut floor = 0;

    for (step, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }

        if floor == -1 {
            return step as i32 + 1;
        }
    }

    -1
}
