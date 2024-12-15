use regex::Regex;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    for equation in input.split("\n\n") {
        let re = Regex::new(r"[A-Za-z]+\s+A:\s+X\+([0-9]+),\s+Y\+([0-9]+)\n[A-Za-z]+\s+B:\s+X\+([0-9]+),\s+Y\+([0-9]+)\nPrize: X=([0-9]+), Y=([0-9]+)").unwrap();
        let captures = re.captures(equation).unwrap();
        let ax: f64 = captures.get(1).unwrap().as_str().parse().unwrap();
        let ay: f64 = captures.get(2).unwrap().as_str().parse().unwrap();
        let bx: f64 = captures.get(3).unwrap().as_str().parse().unwrap();
        let by: f64 = captures.get(4).unwrap().as_str().parse().unwrap();
        let px: f64 = captures.get(5).unwrap().as_str().parse().unwrap();
        let py: f64 = captures.get(6).unwrap().as_str().parse().unwrap();

        let count_a: f64 = (px * by - py * bx) / (ax * by - ay * bx);
        let count_b: f64 = (px - ax * count_a) / bx;

        if count_a % 1.0 == 0.0 && count_b % 1.0 == 0.0 {
            result += (count_a * 3.0 + count_b) as u64
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    for equation in input.split("\n\n") {
        let re = Regex::new(r"[A-Za-z]+\s+A:\s+X\+([0-9]+),\s+Y\+([0-9]+)\n[A-Za-z]+\s+B:\s+X\+([0-9]+),\s+Y\+([0-9]+)\nPrize: X=([0-9]+), Y=([0-9]+)").unwrap();
        let captures = re.captures(equation).unwrap();
        let ax: f64 = captures.get(1).unwrap().as_str().parse().unwrap();
        let ay: f64 = captures.get(2).unwrap().as_str().parse().unwrap();
        let bx: f64 = captures.get(3).unwrap().as_str().parse().unwrap();
        let by: f64 = captures.get(4).unwrap().as_str().parse().unwrap();
        let px: f64 = captures.get(5).unwrap().as_str().parse::<f64>().unwrap() + 10000000000000.0;
        let py: f64 = captures.get(6).unwrap().as_str().parse::<f64>().unwrap() + 10000000000000.0;

        let count_a: f64 = (px * by - py * bx) / (ax * by - ay * bx);
        let count_b: f64 = (px - ax * count_a) / bx;

        if count_a % 1.0 == 0.0 && count_b % 1.0 == 0.0 {
            result += (count_a * 3.0 + count_b) as u64
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
