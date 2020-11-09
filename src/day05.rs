use regex::Regex;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<String>) -> i32 {
    let bad_pairs = Regex::new("ab|cd|pq|xy").unwrap();
    let three_vowels = Regex::new("^.*[aeiou].*[aeiou].*[aeiou].*$").unwrap();
    let pairs =
        Regex::new("aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz")
            .unwrap();

    input
        .iter()
        .filter(|i| three_vowels.is_match(i) && pairs.is_match(i) && !bad_pairs.is_match(i))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let example = "ugknbfddgicrmopn";
    }
}
