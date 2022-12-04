use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Debug)]
pub struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    fn covers(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

#[aoc_generator(day4)]
pub fn generate(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(',');
            let (a, b) = (split.next().unwrap(), split.next().unwrap());
            let mut split = a.split('-');
            let (n1, n2) = (split.next().unwrap(), split.next().unwrap());
            let first = Range::new(n1.parse().unwrap(), n2.parse().unwrap());
            let mut split = b.split('-');
            let (n1, n2) = (split.next().unwrap(), split.next().unwrap());
            let second = Range::new(n1.parse().unwrap(), n2.parse().unwrap());
            (first, second)
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[(Range, Range)]) -> u32 {
    input
        .iter()
        .filter(|(first, second)| first.covers(second) || second.covers(first))
        .count() as u32
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[(Range, Range)]) -> u32 {
    input
        .iter()
        .filter(|(first, second)| first.overlaps(second))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test1() {
        let expected = vec![
            (Range::new(2, 4), Range::new(6, 8)),
            (Range::new(2, 3), Range::new(4, 5)),
            (Range::new(5, 7), Range::new(7, 9)),
            (Range::new(2, 8), Range::new(3, 7)),
            (Range::new(6, 6), Range::new(4, 6)),
            (Range::new(2, 6), Range::new(4, 8)),
        ];
        let actual = generate(EXAMPLE);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let first = Range::new(2, 8);
        let second = Range::new(3, 7);

        assert!(first.covers(&second))
    }

    #[test]
    fn test3() {
        let first = Range::new(4, 6);
        let second = Range::new(6, 6);

        assert!(first.covers(&second))
    }

    #[test]
    fn test4() {
        let expected = 2;
        let actual = solve_part1(&generate(EXAMPLE));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test5() {
        let expected = 4;
        let actual = solve_part2(&generate(EXAMPLE));

        assert_eq!(expected, actual)
    }
}
