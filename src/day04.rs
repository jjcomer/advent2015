use md5;

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> i32 {
    (1..)
        .filter(|i| format!("{:x}", md5::compute(format!("{}{}", input, i))).starts_with("00000"))
        .next()
        .unwrap()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> i32 {
    (1..)
        .filter(|i| format!("{:x}", md5::compute(format!("{}{}", input, i))).starts_with("000000"))
        .next()
        .unwrap()
}
