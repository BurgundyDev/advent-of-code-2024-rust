advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports: Vec<Vec<i32>> = input.lines().map(|line| line.split(" ").map(|num| num.parse().unwrap()).collect()).collect();
    let mut safe_reports: u32 = 0;
    for report in &reports
    {
        let mut max_diff = 0;
        if report.iter().is_sorted() || report.iter().rev().is_sorted()
        {
            for (index, node) in report.iter().enumerate()
            {
                if (index + 1) == report.len() { break };
                let diff = node - report[index+1];
                if diff == 0 { max_diff = 0; break;}
                max_diff = max_diff.max(diff.abs())
            }
        }

        if max_diff < 4 && max_diff > -4 && max_diff != 0
        {
            safe_reports += 1
        }
    }
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports: Vec<Vec<i32>> = input.lines().map(|line| line.split(" ").map(|num| num.parse().unwrap()).collect()).collect();
    let mut safe_reports: u32 = 0;
    let mut safe_indices: Vec<usize> = vec![];
    for (ri, report) in reports.iter().enumerate()
    {
        let mut max_diff = 0;
        if report.iter().is_sorted() || report.iter().rev().is_sorted()
        {
            for (index, node) in report.iter().enumerate()
            {
                if (index + 1) == report.len() { break };
                let diff = node - report[index+1];
                if diff == 0 { max_diff = 0; break;}
                max_diff = max_diff.max(diff.abs())
            }
        }

        if max_diff < 4 && max_diff > -4 && max_diff != 0
        {
            safe_reports += 1;
            safe_indices.push(ri);
        }
    }
    for (ri, report) in reports.iter().enumerate()
    {
        if safe_indices.contains(&ri) { continue; }

        let mut to_drop: usize = 0;

        while to_drop < report.len()
        {
            let mut corrected_report = report.clone();
            corrected_report.remove(to_drop);

            let mut max_diff = 0;
            if corrected_report.iter().is_sorted() || corrected_report.iter().rev().is_sorted()
            {
                for (index, node) in corrected_report.iter().enumerate()
                {
                    if (index + 1) == corrected_report.len() { break };
                    let diff = node - corrected_report[index+1];
                    if diff == 0 { max_diff = 0; break;}
                    max_diff = max_diff.max(diff.abs())
                }
            }

            if max_diff < 4 && max_diff > -4 && max_diff != 0
            {
                safe_reports += 1;
                break;
            }

            to_drop += 1
        }
    }

    Some(safe_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
