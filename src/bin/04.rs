use regex::Regex;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let char_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let xmas_regex = Regex::new(r"XMAS").unwrap();
    let samx_regex = Regex::new(r"SAMX").unwrap();
    let mut result: u32 = 0;
    for (li, line) in char_map.iter().enumerate()
    {
        for (ci, char) in line.iter().enumerate()
        {
            if *char == 'X'
            {
                if ci >= 4
                {
                    // check left
                    if li >= 4
                    {
                        // check left-up
                    }
                    if li + 3 < char_map.len()
                    {
                        // check left-down
                    }
                }
                if ci + 3 < line.len()
                {
                    // check right
                    if li >= 4
                    {
                        // check right-up
                    }
                    if li + 3 < char_map.len()
                    {
                        // check right-down
                    }
                }
                if li >= 4
                {
                    // check up
                }
                if li + 3 < char_map.len()
                {
                    // check down
                }
            }
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
