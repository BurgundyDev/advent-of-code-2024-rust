use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|i| i.to_digit(10).unwrap()).collect()).collect();

    let mut result: u32 = 0;

    for (ri, row) in grid.iter().enumerate()
    {
        for (ci, char) in row.iter().enumerate()
        {
            if grid[ri][ci] != 0
            {
                continue;
            }
            let mut local_result: u32 = 0;

            let mut stack: Vec<(usize, usize)> = vec![];
            let mut visited: HashSet<(usize, usize)> = HashSet::new();

            stack.push((ri, ci));
            visited.insert((ri, ci));

            while stack.len() > 0
            {
                let (curr_y, curr_x) = stack.pop().unwrap();
                visited.insert((curr_y, curr_x));

                if grid[curr_y][curr_x] == 9
                {
                    local_result += 1;
                    continue;
                }

                if curr_y > 0 {
                    if grid[curr_y - 1][curr_x] == grid[curr_y][curr_x] + 1 && !visited.contains(&(curr_y - 1, curr_x))
                    {
                        stack.push((curr_y - 1, curr_x));
                    }
                }

                if curr_x > 0 {
                    if grid[curr_y][curr_x - 1] == grid[curr_y][curr_x] + 1 && !visited.contains(&(curr_y, curr_x - 1))
                    {
                        stack.push((curr_y, curr_x - 1));
                    }
                }

                if curr_y + 1 < grid.len()
                {
                    if grid[curr_y + 1][curr_x] == grid[curr_y][curr_x] + 1 && !visited.contains(&(curr_y + 1, curr_x))
                    {
                        stack.push((curr_y + 1, curr_x));
                    }
                }

                if curr_x + 1 < grid[0].len()
                {
                    if grid[curr_y][curr_x + 1] == grid[curr_y][curr_x] + 1 && !visited.contains(&(curr_y, curr_x + 1))
                    {
                        stack.push((curr_y, curr_x + 1));
                    }
                }
            }

            result += local_result;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|i| i.to_digit(10).unwrap()).collect()).collect();

    let mut result: u32 = 0;

    for (ri, row) in grid.iter().enumerate()
    {
        for (ci, char) in row.iter().enumerate()
        {
            if grid[ri][ci] != 0
            {
                continue;
            }
            let mut local_result: u32 = 0;

            let mut stack: Vec<(usize, usize)> = vec![];

            stack.push((ri, ci));

            while stack.len() > 0
            {
                let (curr_y, curr_x) = stack.pop().unwrap();

                if grid[curr_y][curr_x] == 9
                {
                    local_result += 1;
                    continue;
                }

                if curr_y > 0 {
                    if grid[curr_y - 1][curr_x] == grid[curr_y][curr_x] + 1
                    {
                        stack.push((curr_y - 1, curr_x));
                    }
                }

                if curr_x > 0 {
                    if grid[curr_y][curr_x - 1] == grid[curr_y][curr_x] + 1
                    {
                        stack.push((curr_y, curr_x - 1));
                    }
                }

                if curr_y + 1 < grid.len()
                {
                    if grid[curr_y + 1][curr_x] == grid[curr_y][curr_x] + 1
                    {
                        stack.push((curr_y + 1, curr_x));
                    }
                }

                if curr_x + 1 < grid[0].len()
                {
                    if grid[curr_y][curr_x + 1] == grid[curr_y][curr_x] + 1
                    {
                        stack.push((curr_y, curr_x + 1));
                    }
                }
            }

            result += local_result;
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
