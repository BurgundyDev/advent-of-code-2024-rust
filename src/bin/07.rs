advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<i64> {
    let mut equations: Vec<(i64, Vec<i64>)> = vec![];
    for line in input.lines()
    {
        let target: i64 = line.split_once(":").unwrap().0.parse().unwrap();
        let nums: Vec<i64> = line.split_once(":").unwrap().1.split(" ").filter(|s| !s.is_empty()).map(|n| n.parse().unwrap()).collect();
        equations.push((target, nums));
    }
    let mut result: i64 = 0;
    for equation in equations
    {
        let mut results: Vec<i64> = vec![equation.1[0]];

        for &n in &equation.1[1..]
        {
            results = results.iter().flat_map(|&r| vec![r * n, r + n]).collect();
        }
        if results.contains(&equation.0)
        {
            result += equation.0;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut equations: Vec<(i64, Vec<i64>)> = vec![];
    for line in input.lines()
    {
        let target: i64 = line.split_once(":").unwrap().0.parse().unwrap();
        let nums: Vec<i64> = line.split_once(":").unwrap().1.split(" ").filter(|s| !s.is_empty()).map(|n| n.parse().unwrap()).collect();
        equations.push((target, nums));
    }
    let mut result: i64 = 0;
    for equation in equations
    {
        let mut results: Vec<i64> = vec![equation.1[0]];

        for &n in &equation.1[1..]
        {
            results = results.iter().flat_map(|&r| vec![r * n, r + n, r * 10_i64.pow(n.checked_ilog10().unwrap_or(0) + 1) + n]).collect();
        }
        if results.contains(&equation.0)
        {
            result += equation.0;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
