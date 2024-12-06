use std::iter::Map;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let rules: Vec<(i32, i32)> = input.lines().filter(|l| l.contains("|")).map(|l| l.split_once("|").unwrap()).map(|n| (n.0.parse().unwrap(), n.1.parse().unwrap())).collect();
    let updates: Vec<Vec<i32>> = input.lines().filter(|l| l.contains(",")).map(|l| l.split(",").map(|n| n.parse().unwrap()).collect()).collect();
    let mut result: u32 = 0;

    for update in updates.iter()
    {
        let mut works: bool = true;
        for rule in rules.iter()
        {
            if update.contains(&rule.0) && update.contains(&rule.1)
            {
                if update.iter().position(|l| l == &rule.0) > update.iter().position(|l| l == &rule.1)
                {
                    works = false;
                }
            }
        }
        if works {
            result += update[update.len()/2] as u32;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules: Vec<(i32, i32)> = input.lines().filter(|l| l.contains("|")).map(|l| l.split_once("|").unwrap()).map(|n| (n.0.parse().unwrap(), n.1.parse().unwrap())).collect();
    let mut updates: Vec<Vec<i32>> = input.lines().filter(|l| l.contains(",")).map(|l| l.split(",").map(|n| n.parse().unwrap()).collect()).collect();
    let mut result: u32 = 0;

    for mut update in updates.into_iter()
    {
        let mut works: bool = true;
        for rule in rules.iter()
        {
            if update.contains(&rule.0) && update.contains(&rule.1)
            {
                let left_pos = update.iter().position(|l| l == &rule.0).unwrap();
                let right_pos = update.iter().position(|l| l == &rule.1).unwrap();
                if &left_pos > &right_pos
                {
                    works = false;
                }
            }
        }
        if works == true { continue; }
        while !works
        {
            for rule in rules.iter()
            {
                if update.contains(&rule.0) && update.contains(&rule.1)
                {
                    let left_pos = update.iter().position(|l| l == &rule.0).unwrap();
                    let right_pos = update.iter().position(|l| l == &rule.1).unwrap();
                    if &left_pos > &right_pos
                    {
                        let moved_item = update.remove(right_pos);
                        update.insert(left_pos, moved_item);
                    }
                }
            }
            let mut local_works: bool = true;
            for rule in rules.iter()
            {
                if update.contains(&rule.0) && update.contains(&rule.1)
                {
                    let left_pos = update.iter().position(|l| l == &rule.0).unwrap();
                    let right_pos = update.iter().position(|l| l == &rule.1).unwrap();
                    if &left_pos > &right_pos
                    {
                        local_works = false;
                    }
                }
            }
            works = local_works;
        }
        result += update[update.len()/2] as u32;

    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
