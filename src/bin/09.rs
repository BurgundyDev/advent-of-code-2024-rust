use std::result;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut filesystem: Vec<Option<u64>> = vec![];
    let mut curr_index: u64 = 0;
    for (di, digit) in input.chars().enumerate()
    {
        if di % 2 == 0
        {
            for _n in 0..digit.to_digit(10).unwrap()
            {
                filesystem.push(Some(curr_index));
            }
            curr_index+=1;
        } else {
            for _n in 0..digit.to_digit(10).unwrap()
            {
                filesystem.push(None);
            }
        }
    }
    // println!("{}", uncompressed);

    let mut first_free: usize = 0;
    while filesystem[first_free] != None
    {
        first_free += 1;
    }
    let mut current_block: usize = filesystem.len() - 1;
    while filesystem[current_block] == None
    {
        current_block -= 1;
    }

    while current_block > first_free
    {
        filesystem[first_free] = filesystem[current_block];
        filesystem[current_block] = None;
        while filesystem[current_block] == None
        {
            current_block -= 1;
        }
        while filesystem[first_free] != None
        {
            first_free += 1;
        }
    }
    // println!("{:?}", filesystem);
    let mut result: u64 = 0;
    for (bi, block) in filesystem.iter().enumerate()
    {
        if block.is_some()
        {
            result += bi as u64 * block.unwrap();
        } else {
            continue;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut filesystem: Vec<Option<u64>> = vec![];
    let mut curr_index: u64 = 0;

    let mut size: Vec<usize> = vec![0; input.chars().count()];
    let mut loc: Vec<usize> = vec![0; input.chars().count()];

    for (di, digit) in input.chars().enumerate()
    {
        if di % 2 == 0
        {

            loc[curr_index as usize] = filesystem.len();
            size[curr_index as usize] = digit.to_digit(10).unwrap() as usize;
            for _n in 0..digit.to_digit(10).unwrap()
            {
                filesystem.push(Some(curr_index));
            }
            curr_index+=1;
        } else {
            for _n in 0..digit.to_digit(10).unwrap()
            {
                filesystem.push(None);
            }
        }
    }
    // println!("{}", uncompressed);

    let mut big: usize = 0;

    while size[big] > 0 {
        big += 1
    }
    big -= 1;

    let mut to_move = big;
    while to_move > 0
    {
        let mut free_space: usize = 0;
        let mut first_free: usize = 0;

        while first_free < loc[to_move] && free_space < size[to_move]
        {
            first_free = first_free + free_space;
            free_space = 0;
            while filesystem[first_free] != None
            {
                first_free += 1
            }
            while first_free + free_space < filesystem.len() && filesystem[first_free + free_space] == None {
                free_space += 1
            }
        }

        if first_free >= loc[to_move]
        {
            to_move -= 1;
            continue;
        }

        for idx in first_free..first_free+size[to_move]
        {
            filesystem[idx] = Some(to_move as u64);
        }
        for idx in loc[to_move]..loc[to_move]+size[to_move]
        {
            filesystem[idx] = None;
        }
        to_move -= 1;
    }

    // println!("{:?}", filesystem);
    let mut result: u64 = 0;
    for (bi, block) in filesystem.iter().enumerate()
    {
        if block.is_some()
        {
            print!("{:?}", block.unwrap());
            result += bi as u64 * block.unwrap();
        } else {
            print!(".");
            continue;
        }
    }
    println!();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
