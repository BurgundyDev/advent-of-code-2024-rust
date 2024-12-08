use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Eq, Hash, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn rotate(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let labirynth: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // (y, x) format
    let mut curr_point= (0, 0);
    let mut next_point = (0, 0);
    let mut direction: Direction = Direction::Up;
    for (ri, row) in labirynth.iter().enumerate()
    {
        for (ci, char) in row.iter().enumerate()
        {
            if *char == '^'
            {
                curr_point = (ri, ci);
                next_point = (ri - 1, ci);
            }
        }
    }
    let mut labirynt_move_map: HashSet<(usize, usize)> = HashSet::new();
    labirynt_move_map.insert(curr_point);
    while next_point.0 < labirynth.len() && next_point.1 < labirynth[0].len()
    {
        // println!("{:?}", curr_point);
        // println!("{:?}", next_point);
        match labirynth[next_point.0][next_point.1] {
            '#' => {
                direction = rotate(direction);
                match direction {
                    Direction::Up => next_point = (curr_point.0 - 1, curr_point.1),
                    Direction::Right => next_point = (curr_point.0, curr_point.1 + 1),
                    Direction::Down => next_point = (curr_point.0 + 1, curr_point.1),
                    Direction::Left => next_point = (curr_point.0, curr_point.1 - 1),
                }
            }
            _ => {
                curr_point = next_point;
                labirynt_move_map.insert(curr_point);
                if (curr_point.0 == 0 && direction == Direction::Up) || (curr_point.1 == 0 && direction == Direction::Left) { break; }
                match direction {
                    Direction::Up => next_point = (curr_point.0 - 1, curr_point.1),
                    Direction::Right => next_point = (curr_point.0, curr_point.1 + 1),
                    Direction::Down => next_point = (curr_point.0 + 1, curr_point.1),
                    Direction::Left => next_point = (curr_point.0, curr_point.1 - 1),
                }
            }
        }
    }

    Some(labirynt_move_map.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let labirynth: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // (y, x) format
    let mut start_point: (usize, usize) = (0, 0);
    let mut curr_point: (usize, usize)= (0, 0);
    let mut next_point: (usize, usize) = (0, 0);
    let mut direction: Direction = Direction::Up;
    for (ri, row) in labirynth.iter().enumerate()
    {
        for (ci, char) in row.iter().enumerate()
        {
            if *char == '^'
            {
                start_point = (ri, ci);
                curr_point = (ri, ci);
                next_point = (ri - 1, ci);
            }
        }
    }
    let mut labirynt_move_map: HashSet<(usize, usize)> = HashSet::new();
    labirynt_move_map.insert(curr_point);
    while next_point.0 < labirynth.len() && next_point.1 < labirynth[0].len()
    {
        // println!("{:?}", curr_point);
        // println!("{:?}", next_point);
        match labirynth[next_point.0][next_point.1] {
            '#' => {
                direction = rotate(direction);
                match direction {
                    Direction::Up => next_point = (curr_point.0 - 1, curr_point.1),
                    Direction::Right => next_point = (curr_point.0, curr_point.1 + 1),
                    Direction::Down => next_point = (curr_point.0 + 1, curr_point.1),
                    Direction::Left => next_point = (curr_point.0, curr_point.1 - 1),
                }
            }
            _ => {
                curr_point = next_point;
                labirynt_move_map.insert(curr_point);
                if (curr_point.0 == 0 && direction == Direction::Up) || (curr_point.1 == 0 && direction == Direction::Left) { break; }
                match direction {
                    Direction::Up => next_point = (curr_point.0 - 1, curr_point.1),
                    Direction::Right => next_point = (curr_point.0, curr_point.1 + 1),
                    Direction::Down => next_point = (curr_point.0 + 1, curr_point.1),
                    Direction::Left => next_point = (curr_point.0, curr_point.1 - 1),
                }
            }
        }
    }

    let mut result: u32 = 0;
    let mut modifiable_labirynth = labirynth.clone();
    for position in labirynt_move_map.iter()
    {
        if *position == start_point || modifiable_labirynth[position.0][position.1] == '#' { continue; }
        modifiable_labirynth[position.0][position.1] = '#';

        curr_point = start_point;

        direction = Direction::Up;

        let mut local_move_map: HashSet<(usize, usize, Direction)> = HashSet::new();
        loop
        {
            if local_move_map.contains(&(curr_point.0, curr_point.1, direction))
            {
                result += 1;
                break;
            }
            local_move_map.insert((curr_point.0, curr_point.1, direction));

            if (curr_point.0 == 0 && direction == Direction::Up) || (curr_point.1 == 0 && direction == Direction::Left) { break; }
            match direction {
                Direction::Up => next_point = (curr_point.0 - 1, curr_point.1),
                Direction::Right => next_point = (curr_point.0, curr_point.1 + 1),
                Direction::Down => next_point = (curr_point.0 + 1, curr_point.1),
                Direction::Left => next_point = (curr_point.0, curr_point.1 - 1),
            }
            if next_point.0 >= modifiable_labirynth.len() || next_point.1 >= modifiable_labirynth[0].len() { break; }

            if modifiable_labirynth[next_point.0][next_point.1] == '#'
            {
                direction = rotate(direction);
            } else {
                curr_point = next_point;
            }
        }
        modifiable_labirynth[position.0][position.1] = '.';
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
