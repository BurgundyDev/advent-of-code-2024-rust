advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let labirynth: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut starting_point = (0, 0);
    for row in labirynth
    {
        for char in row
        {

        }
    }
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
