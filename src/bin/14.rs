advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    part_one_impl(input, 101, 103)
}

pub fn part_one_impl(input: &str, width: i32, height: i32) -> Option<u32> {
    let mut result: Vec<u32> = vec![0, 0 ,0, 0];
    let mut final_positions: Vec<(i32, i32)> = vec![];
    let width_meridian: i32 = (width - 1) / 2;
    let height_meridian: i32 = (height - 1) / 2;

    for robot in input.lines() {
        let mut iterator = robot.split_whitespace();
        let mut position: Vec<i32> = iterator.next().unwrap().strip_prefix("p=").unwrap().split(",").map(|n| n.parse::<i32>().unwrap()).collect();
        let mval: Vec<i32> = iterator.next().unwrap().strip_prefix("v=").unwrap().split(",").map(|n| n.parse::<i32>().unwrap()).collect();
        for _n in 0..100 {
            position[0] = (position[0] + mval[0]) % width;
            if position[0] < 0 { position[0] = width + position[0] }
            position[1] = (position[1] + mval[1]) % height;
            if position[1] < 0 { position[1] = height + position[1] }
        }
        final_positions.push((position[0], position[1]));
        println!("Pos: {:?}", position);
        if position[0] > width_meridian {
            if position[1] > height_meridian {
                result[0] += 1;
            } else if position[1] < height_meridian {
                result[1] += 1
            }
        } else if position[0] < width_meridian {
            if position[1] > height_meridian {
                result[2] += 1;
            } else if position[1] < height_meridian {
                result[3] += 1;
            }
        }
    }
    println!("{:?}", final_positions);
    for h in 0..height {
        for w in 0..width {
            if final_positions.iter().filter(|r| r.0 == w && r.1 == h).count() > 0 { print!("#") } else { print!(".") }
        }
        println!()
    }
    Some(result.into_iter().reduce(|a, b| a * b).unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let width: i32 = 101;
    let height: i32 = 103;
    let mut robots: Vec<((i32, i32), (i32, i32))> = vec![];

    for robot in input.lines() {
        let mut iterator = robot.split_whitespace();
        let position: Vec<i32> = iterator.next().unwrap().strip_prefix("p=").unwrap().split(",").map(|n| n.parse::<i32>().unwrap()).collect();
        let mval: Vec<i32> = iterator.next().unwrap().strip_prefix("v=").unwrap().split(",").map(|n| n.parse::<i32>().unwrap()).collect();

        robots.push(((position[0], position[1]), (mval[0], mval[1])));
    }

    println!("Second 0");
    for h in 0..height {
        for w in 0..width {
            if robots.iter().filter(|r| r.0.0 == w && r.0.1 == h).count() > 0  {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
    println!();

    for n in 1..10100 {
        for (position, mval) in robots.iter_mut() {
            position.0 = (position.0 + mval.0) % width;
            if position.0 < 0 { position.0 = width + position.0 }
            position.1 = (position.1 + mval.1) % height;
            if position.1 < 0 { position.1 = height + position.1 }
        }
        println!("Second {n}");
        for h in 0..height {
            for w in 0..width {
                if robots.iter().filter(|r| r.0.0 == w && r.0.1 == h).count() > 0  {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!()
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_impl(&advent_of_code::template::read_file("examples", DAY), 11, 7);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
