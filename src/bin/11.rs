advent_of_code::solution!(11);

pub fn num_digits(number: u64) -> u64 {
    let mut i: u64 = 10;
    let mut j: u64 = 1;
    while i <= number {
        i *= 10;
        j += 1;
    }
    j
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones: Vec<u64> = input.split(" ").map(|n| n.parse().unwrap()).collect();

    for n in 0..25
    {
        let mut temp_stones: Vec<u64> = vec![];

        while stones.len() > 0
        {
            let mut stone = stones.remove(0);

            if stone == 0 { stone = 1 }
            else if num_digits(stone) % 2 == 0 {
                temp_stones.push(stone / (10_u64.pow((num_digits(stone) / 2) as u32)));
                stone = stone % (10_u64.pow((num_digits(stone) / 2) as u32))
            } else {
                stone *= 2024;
            }
            temp_stones.push(stone);
        }
        // println!("{:?}: {:?}", n, temp_stones);
        stones = temp_stones;
    }
    Some(stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stones: Vec<u64> = input.split(" ").map(|n| n.parse().unwrap()).collect();

    for n in 0..75
    {
        let mut temp_stones: Vec<u64> = vec![];

        while stones.len() > 0
        {
            let mut stone = stones.remove(0);

            if stone == 0 { stone = 1 }
            else if num_digits(stone) % 2 == 0 {
                temp_stones.push(stone / (10_u64.pow((num_digits(stone) / 2) as u32)));
                stone = stone % (10_u64.pow((num_digits(stone) / 2) as u32))
            } else {
                stone *= 2024;
            }
            temp_stones.push(stone);
        }
        // println!("{:?}: {:?}", n, temp_stones);
        stones = temp_stones;
    }
    Some(stones.len() as u32)
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
        assert_eq!(result, None);
    }
}
