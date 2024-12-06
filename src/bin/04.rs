advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let char_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut result: u32 = 0;
    for (li, line) in char_map.iter().enumerate()
    {
        for (ci, char) in line.iter().enumerate()
        {
            if *char == 'X'
            {
                if ci >= 3
                {
                    if line[ci-1] == 'M'
                    && line[ci-2] == 'A'
                    && line[ci-3] == 'S'
                    {
                        result += 1
                    }
                    if li >= 3
                    {
                        if char_map[li-1][ci-1] == 'M'
                        && char_map[li-2][ci-2] == 'A'
                        && char_map[li-3][ci-3] == 'S'
                        {
                            result += 1
                        }
                    }
                    if li + 3 < char_map.len()
                    {
                        if char_map[li+1][ci-1] == 'M'
                        && char_map[li+2][ci-2] == 'A'
                        && char_map[li+3][ci-3] == 'S'
                        {
                            result += 1
                        }
                    }
                }
                if ci + 3 < line.len()
                {
                    if line[ci+1] == 'M'
                    && line[ci+2] == 'A'
                    && line[ci+3] == 'S'
                    {
                        result += 1
                    }
                    if li >= 3
                    {
                        if char_map[li-1][ci+1] == 'M'
                        && char_map[li-2][ci+2] == 'A'
                        && char_map[li-3][ci+3] == 'S'
                        {
                            result += 1
                        }
                    }
                    if li + 3 < char_map.len()
                    {
                        if char_map[li+1][ci+1] == 'M'
                        && char_map[li+2][ci+2] == 'A'
                        && char_map[li+3][ci+3] == 'S'
                        {
                            result += 1
                        }
                    }
                }
                if li >= 3
                {
                    if char_map[li-1][ci] == 'M'
                    && char_map[li-2][ci] == 'A'
                    && char_map[li-3][ci] == 'S'
                    {
                        result += 1
                    }
                }
                if li + 3 < char_map.len()
                {
                    if char_map[li+1][ci] == 'M'
                    && char_map[li+2][ci] == 'A'
                    && char_map[li+3][ci] == 'S'
                    {
                        result += 1
                    }
                }
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let char_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut result: u32 = 0;
    for (li, line) in char_map.iter().enumerate()
    {
        for (ci, char) in line.iter().enumerate()
        {
            if *char == 'A' && ci > 0 && ci + 1 < line.len() && li > 0 && li + 1 < char_map.len()
            {
                let chars: Vec<char> = vec![char_map[li-1][ci-1], char_map[li-1][ci+1],
                                            char_map[li+1][ci+1], char_map[li+1][ci-1]];
                if chars == ['M', 'M', 'S', 'S']
                || chars == ['M', 'S', 'S', 'M']
                || chars == ['S', 'S', 'M', 'M']
                || chars == ['S', 'M', 'M', 'S']
                {
                    result += 1
                }
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
