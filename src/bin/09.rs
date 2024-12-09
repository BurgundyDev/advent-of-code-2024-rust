advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let mut uncompressed: Vec<char> = vec![];
    let mut curr_index: u32 = 0;
    for (di, digit) in input.chars().enumerate()
    {
        if di % 2 == 0
        {
            for _n in 0..digit.to_digit(10).unwrap()
            {
                uncompressed.push(char::from_digit(curr_index, 10).unwrap());
            }
            curr_index+=1;
        } else {
            for _n in 0..digit.to_digit(10).unwrap()
            {
                uncompressed.push('.');
            }
        }
    }
    // println!("{}", uncompressed);
    let mut compressed: String = String::new();
    for block in uncompressed.iter_mut()
    {
        if *block != '.'
        {
            compressed.push(*block);
        } else {

        }
    }
    println!("{:?}", compressed);
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
