use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let mut iter=  input.split("\n\n");
    let mut grid: Vec<Vec<char>> = iter.next().unwrap().lines().map(|l| l.chars().collect()).collect();
    let instructions: Vec<char> = iter.next().unwrap().chars().filter(|c| *c != '\n').collect();
    let mut robot: (usize, usize) = (0, 0);
    for (ri, row) in grid.iter().enumerate() {
        for (ci, char) in row.iter().enumerate() {
            match char {
                '@' => { robot = (ri, ci); break; },
                _ => (),
            }
        }
    }
    // println!("{:?}", robot);
    // println!("{:?}", boxes);
    for instruction in instructions {
        // for row in grid.iter() {
        //     println!("{:?}", row)
        // }
        // println!();
        let next_move: (i32, i32) = match instruction {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!(),
        };
        let mut moving_objects: Vec<(usize, usize)> = vec![];
        let mut next_object = robot;
        let mut can_move: bool = true;
        loop {
            next_object = ((next_object.0 as i32 + next_move.0) as usize, (next_object.1 as i32 + next_move.1) as usize);
            match grid[next_object.0][next_object.1] {
                '#' => { can_move = false; break; },
                'O' => { moving_objects.push(next_object); },
                '.' => { break; },
                _ => panic!(),
            }
        }
        if !can_move { continue; }
        grid[robot.0][robot.1] = '.';
        grid[(robot.0 as i32 + next_move.0) as usize][(robot.1 as i32 + next_move.1) as usize] = '@';
        for moved in moving_objects {
            grid[(moved.0 as i32 + next_move.0) as usize][(moved.1 as i32 + next_move.1) as usize] = 'O';
        }
        robot = ((robot.0 as i32 + next_move.0) as usize, (robot.1 as i32 + next_move.1) as usize)
    }

    // for row in grid.iter() {
    //     println!("{:?}", row)
    // }
    let mut result: u32 = 0;

    for (ri, row) in grid.iter().enumerate() {
        for (ci, char) in row.iter().enumerate() {
            if *char == 'O' {
                result += 100 * ri as u32 + ci as u32
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut iter=  input.split("\n\n");
    let mut grid: Vec<Vec<char>> = iter.next().unwrap().lines().map(|l| l.chars().collect()).collect();
    let instructions: Vec<char> = iter.next().unwrap().chars().filter(|c| *c != '\n').collect();
    let mut robot: (usize, usize) = (0, 0);
    for row in grid.iter_mut() {
        let mut new_row: Vec<char> = vec![];
        for char in row.iter() {
            match *char {
                '#' => {new_row.push('#'); new_row.push('#');},
                'O' => {new_row.push('['); new_row.push(']');},
                '.' => {new_row.push('.'); new_row.push('.');},
                '@' => {new_row.push('@'); new_row.push('.');},
                _ => panic!()
            }
        }
        *row = new_row.clone();
    }
    for (ri, row) in grid.iter().enumerate() {
        for (ci, char) in row.iter().enumerate() {
            match char {
                '@' => { robot = (ri, ci); break; },
                _ => (),
            }
        }
    }
    for row in grid.iter() {
        println!("{:?}", row)
    }
    println!();
    // println!("{:?}", robot);
    // println!("{:?}", boxes);
    for instruction in instructions {
        // for row in grid.iter() {
        //     for char in row.iter() {
        //         print!("{char}");
        //     }
        //     println!()
        // }
        // println!();
        let next_move: (i32, i32) = match instruction {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!(),
        };
        let mut moving_objects: HashSet<(usize, usize, char)> = HashSet::new();
        let mut object_queue: VecDeque<(usize, usize)> = VecDeque::new();
        object_queue.push_back(((robot.0 as i32 + next_move.0) as usize, (robot.1 as i32 + next_move.1) as usize));
        let mut can_move: bool = true;
        while let Some(next_object) = object_queue.pop_front() {
            match grid[next_object.0][next_object.1] {
                '#' => { can_move = false; break; },
                '[' => {
                    if moving_objects.contains(&(next_object.0, next_object.1, grid[next_object.0][next_object.1])) { continue; }
                    moving_objects.insert((next_object.0, next_object.1, grid[next_object.0][next_object.1]));
                    moving_objects.insert((next_object.0, next_object.1 + 1, grid[next_object.0][next_object.1 + 1]));
                    object_queue.push_back(((next_object.0 as i32 + next_move.0) as usize, (next_object.1 as i32 + next_move.1) as usize));
                    object_queue.push_back(((next_object.0 as i32 + next_move.0) as usize, (next_object.1 as i32 + 1 + next_move.1) as usize));
                },
                ']' => {
                    if moving_objects.contains(&(next_object.0, next_object.1, grid[next_object.0][next_object.1])) { continue; }
                    moving_objects.insert((next_object.0, next_object.1, grid[next_object.0][next_object.1]));
                    moving_objects.insert((next_object.0, next_object.1 - 1, grid[next_object.0][next_object.1 - 1]));
                    object_queue.push_back(((next_object.0 as i32 + next_move.0) as usize, (next_object.1 as i32 + next_move.1) as usize));
                    object_queue.push_back(((next_object.0 as i32 + next_move.0) as usize, (next_object.1 as i32 - 1 + next_move.1) as usize));
                },
                '.' => { continue; },
                _ => panic!(),
            }
        }
        if !can_move { continue; }
        for moved in moving_objects.iter() {
            grid[moved.0][moved.1] = '.';
        }
        for moved in moving_objects.iter() {
            grid[(moved.0 as i32 + next_move.0) as usize][(moved.1 as i32 + next_move.1) as usize] = moved.2;
        }
        grid[robot.0][robot.1] = '.';
        grid[(robot.0 as i32 + next_move.0) as usize][(robot.1 as i32 + next_move.1) as usize] = '@';
        robot = ((robot.0 as i32 + next_move.0) as usize, (robot.1 as i32 + next_move.1) as usize)
    }

    // for row in grid.iter() {
    //     for char in row.iter() {
    //         print!("{char}");
    //     }
    //     println!()
    // }
    let mut result: u32 = 0;

    for (ri, row) in grid.iter().enumerate() {
        for (ci, char) in row.iter().enumerate() {
            if *char == '[' {
                result += 100 * ri as u32 + ci as u32
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
