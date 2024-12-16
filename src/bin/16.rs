use std::cmp::Ordering;
use std::collections::BinaryHeap;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Point {
        cost: usize,
        position_r: usize,
        position_c: usize,
    }

    impl Ord for Point {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost).then_with(|| self.position_r.cmp(&other.position_r)).then_with(|| self.position_c.cmp(&other.position_c))
        }
    }

    impl PartialOrd for Point {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut heap: BinaryHeap<Point> = BinaryHeap::new();

    heap.push(Point { cost: 8, position_r: 9, position_c: 10 });
    heap.push(Point { cost: 2, position_r: 3, position_c: 4 });

    while let Some(Point { cost, position_r, position_c }) = heap.pop() {
        println!("{:?}", cost);
    }
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
