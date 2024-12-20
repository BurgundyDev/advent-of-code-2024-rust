advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let numbers: i32 = regex.captures_iter(input).
    map(|f| f.get(0).unwrap().as_str()
    .strip_prefix("mul(").unwrap().strip_suffix(")").unwrap().split(",").
    map(|f| f.parse().unwrap()).reduce(|a, b| a * b).unwrap())
    .reduce(|c, d| c + d).unwrap();
    Some(numbers as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let blockers = Regex::new(r"don't\(\)(.*?)do\(\)").unwrap();
    let end_blocker = Regex::new(r"don't\(\)(.*)").unwrap();
    let newline = Regex::new(r"\n").unwrap();
    let line = newline.replace_all(input, "");
    let text = blockers.replace_all(&line, "");
    let fin_text = end_blocker.replace(&text, "");
    let numbers: i32 = regex.captures_iter(&fin_text)
    .map(|f| f.get(0).unwrap().as_str()
    .strip_prefix("mul(").unwrap().strip_suffix(")").unwrap().split(",")
    .map(|f| f.parse().unwrap()).reduce(|a, b| a * b).unwrap())
    .reduce(|c, d| c + d).unwrap();
    Some(numbers as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
