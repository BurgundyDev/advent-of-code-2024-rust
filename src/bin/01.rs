use std::path::absolute;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];
    for line in input.lines()
    {
        left_list.push(line.split_once("   ").unwrap().0.parse().unwrap());
        right_list.push(line.split_once("   ").unwrap().1.parse().unwrap());
    }
    left_list.sort();
    right_list.sort();
    let mut distances= 0;
    for (index, element) in left_list.iter().enumerate()
    {
        distances += element.abs_diff(right_list[index]);
    }
    print!("{}", distances);
    Some(distances)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];
    for line in input.lines()
    {
        left_list.push(line.split_once("   ").unwrap().0.parse().unwrap());
        right_list.push(line.split_once("   ").unwrap().1.parse().unwrap());
    }
    left_list.sort();
    right_list.sort();
    let mut similarity= 0;
    for (index, element) in left_list.iter().enumerate()
    {
        similarity += element * (right_list.iter().filter(|&n| *n == *element).count() as u32);
    }
    print!("{}", similarity);
    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
