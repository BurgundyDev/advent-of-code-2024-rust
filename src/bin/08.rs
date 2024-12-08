use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut antennas: HashMap<(usize, usize), char> = HashMap::new();
    let mut areas: HashSet<(usize, usize)> = HashSet::new();

    for (ri, row) in grid.iter().enumerate()
    {
        for (ci, char) in row.iter().enumerate()
        {
            if *char != '.'
            {
                antennas.insert((ri, ci), char.clone());
            }
        }
    }
    // println!("{:?}", antennas);
    for antenna in antennas.iter()
    {
        let antenna_position = antenna.0;

        let common_antennas: Vec<(&(usize, usize), &char)> = antennas.iter().filter(|a: &(&(usize, usize), &char)| a.1 == antenna.1).collect();
        for common_antenna in common_antennas.into_iter()
        {
            let common_antenna_position = common_antenna.0;

            if antenna_position == common_antenna_position { continue; }

            let mut area: (usize, usize) = (0, 0);

            if common_antenna_position.0 < antenna_position.0 || (common_antenna_position.0 - antenna_position.0) <= antenna_position.0
            {
                area.0 = 2 * antenna_position.0 - common_antenna_position.0;
            } else {
                continue;
            }
            if area.0 >= grid.len()
            {
                continue;
            }

            if common_antenna_position.1 < antenna_position.1 || (common_antenna_position.1 - antenna_position.1) <= antenna_position.1
            {
                area.1 = 2 * antenna_position.1 - common_antenna_position.1;
            } else {
                continue;
            }
            if area.1 >= grid[0].len()
            {
                continue;
            }
            areas.insert(area);
        }
    }
    // println!("{:?}", areas);

    // for (ri, row) in grid.iter().enumerate()
    // {
    //     for (ci, char) in row.iter().enumerate()
    //     {
    //         if areas.contains(&(ri, ci))
    //         {
    //             print!("#")
    //         } else {
    //             print!("{}", char)
    //         }
    //     }
    //     println!();
    // }
    Some(areas.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut antennas: HashMap<(usize, usize), char> = HashMap::new();
    let mut areas: HashSet<(usize, usize)> = HashSet::new();

    for (ri, row) in grid.iter().enumerate()
    {
        for (ci, char) in row.iter().enumerate()
        {
            if *char != '.'
            {
                antennas.insert((ri, ci), char.clone());
            }
        }
    }
    // println!("{:?}", antennas);
    for antenna in antennas.iter()
    {
        let antenna_position = antenna.0;
        areas.insert(antenna_position.clone());

        let common_antennas: Vec<(&(usize, usize), &char)> = antennas.iter().filter(|a: &(&(usize, usize), &char)| a.1 == antenna.1).collect();
        for common_antenna in common_antennas.into_iter()
        {
            let common_antenna_position = common_antenna.0;

            if antenna_position == common_antenna_position { continue; }

            let mut area: (usize, usize) = (0, 0);

            if common_antenna_position.0 < antenna_position.0 || (common_antenna_position.0 - antenna_position.0) <= antenna_position.0
            {
                area.0 = 2 * antenna_position.0 - common_antenna_position.0;
            } else {
                continue;
            }
            if area.0 >= grid.len()
            {
                continue;
            }

            if common_antenna_position.1 < antenna_position.1 || (common_antenna_position.1 - antenna_position.1) <= antenna_position.1
            {
                area.1 = 2 * antenna_position.1 - common_antenna_position.1;
            } else {
                continue;
            }
            if area.1 >= grid[0].len()
            {
                continue;
            }
            areas.insert(area);

            let mut x = area.clone();
            let mut y = antenna_position.clone();
            loop {
                let mut next_area: (usize, usize) = (0, 0);
                if y.0 < x.0 || (y.0 - x.0) <= x.0
                {
                    next_area.0 = 2 * x.0 - y.0;
                } else {
                    break;
                }
                if next_area.0 >= grid.len()
                {
                    break;
                }

                if y.1 < x.1 || (y.1 - x.1) <= x.1
                {
                    next_area.1 = 2 * x.1 - y.1;
                } else {
                    break;
                }
                if next_area.1 >= grid[0].len()
                {
                    break;
                }
                areas.insert(next_area);
                y = x.clone();
                x = next_area.clone();
            }
        }
    }
    println!("{:?}", areas);
    for (ri, row) in grid.iter().enumerate()
    {
        for (ci, char) in row.iter().enumerate()
        {
            if areas.contains(&(ri, ci))
            {
                print!("#")
            } else {
                print!("{}", char)
            }
        }
        println!();
    }
    Some(areas.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
