use std::collections::HashMap;

advent_of_code::solution!(11);

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct StoneKey
{
    value: u64,
    depth: u64
}

pub fn num_digits(number: u64) -> u64 {
    let mut i: u64 = 10;
    let mut j: u64 = 1;
    while i <= number {
        i *= 10;
        j += 1;
    }
    j
}

pub fn blink(stone: u64, depth: u64, memory: &mut HashMap<StoneKey, u64>) -> u64 {
    if memory.contains_key(&StoneKey { value: stone, depth: depth })
    {
        return memory[&StoneKey { value: stone, depth: depth }];
    }
    let mut result: u64 = 1;
    if depth == 0 { return result }
    else if stone == 0 {
        result = blink(1, depth - 1, memory);
    }
    else if num_digits(stone) % 2 == 0 {
        result = (blink(stone / (10_u64.pow((num_digits(stone) / 2) as u32)), depth - 1, memory) + blink(stone % (10_u64.pow((num_digits(stone) / 2) as u32)), depth - 1, memory))
    }
    else {
        result = blink(stone * 2024, depth - 1, memory)
    }

    memory.insert(StoneKey { value: stone, depth: depth }, result);

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input.split(" ").map(|n| n.parse().unwrap()).collect();

    let mut result: u64 = 0;

    let mut memory: HashMap<StoneKey, u64> = HashMap::new();

    for stone in stones
    {
        result += blink(stone, 25, &mut memory)
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input.split(" ").map(|n| n.parse().unwrap()).collect();

    let mut result: u64 = 0;

    let mut memory: HashMap<StoneKey, u64> = HashMap::new();

    for stone in stones
    {
        result += blink(stone, 75, &mut memory)
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
