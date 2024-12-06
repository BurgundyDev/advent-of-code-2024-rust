use std::iter::Map;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let rules: Vec<(i32, i32)> = input.lines().filter(|l| l.contains("|")).map(|l| l.split_once("|").unwrap()).map(|n| (n.0.parse().unwrap(), n.1.parse().unwrap())).collect();
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}