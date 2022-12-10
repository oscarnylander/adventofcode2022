use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Debug)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "L" => Self::Left,
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            _ => unreachable!("Invalid direction"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Instruction {
    direction: Direction,
    steps: u32,
}

impl Instruction {
    fn new(direction: Direction, steps: u32) -> Self {
        Self { direction, steps }
    }
}

#[aoc_generator(day9)]
pub fn generate(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(' ');
            let direction = Direction::from_str(split.next().unwrap());
            let steps = split.next().unwrap().parse().unwrap();
            Instruction::new(direction, steps)
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Instruction]) -> usize {
    let mut visited = HashSet::<(i32, i32)>::default();
    visited.insert((0, 0));

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    for instruction in input {
        for _ in 0..instruction.steps {
            let prev = head.clone();
            head = match instruction.direction {
                Direction::Left => (head.0 - 1, head.1),
                Direction::Up => (head.0, head.1 + 1),
                Direction::Right => (head.0 + 1, head.1),
                Direction::Down => (head.0, head.1 - 1),
            };

            tail = reconcile(&prev, &head, &tail);

            visited.insert(tail);
        }
    }

    visited.len()
}

fn reconcile(prev: &(i32, i32), head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    if (head.0.abs_diff(tail.0) > 1 && head.1.abs_diff(tail.1) == 1)
        || (head.0.abs_diff(tail.0) == 1 && head.1.abs_diff(tail.1) > 1)
        || (head.0.abs_diff(tail.0) > 1 && head.1.abs_diff(tail.1) > 1)
    {
        if head.0 > tail.0 && head.1 > tail.1 {
            return (tail.0 + 1, tail.1 + 1);
        }
        if head.0 < tail.0 && head.1 < tail.1 {
            return (tail.0 - 1, tail.1 - 1);
        }
        if head.0 > tail.0 && head.1 < tail.1 {
            return (tail.0 + 1, tail.1 - 1);
        }
        if head.0 < tail.0 && head.1 > tail.1 {
            return (tail.0 - 1, tail.1 + 1);
        }
        unreachable!()
    }
    if head.0.abs_diff(tail.0) > 1 {
        return (prev.0, tail.1);
    }
    if head.1.abs_diff(tail.1) > 1 {
        return (tail.0, prev.1);
    }
    tail.clone()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Instruction]) -> usize {
    let mut visited = HashSet::<(i32, i32)>::default();

    visited.insert((0, 0));

    let mut knots = vec![(0, 0); 10];

    for instruction in input {
        for _ in 0..instruction.steps {
            let mut prev = knots[0].clone();
            knots[0] = match instruction.direction {
                Direction::Left => (prev.0 - 1, prev.1),
                Direction::Up => (prev.0, prev.1 + 1),
                Direction::Right => (prev.0 + 1, prev.1),
                Direction::Down => (prev.0, prev.1 - 1),
            };

            for idx in 1..knots.len() {
                let knot = knots[idx];
                let new = reconcile(&prev, &knots[idx - 1], &knot);
                prev = knot;
                knots[idx] = new;
                if idx == knots.len() - 1 {
                    visited.insert(new);
                }
            }
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static EXAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test1() {
        let expected = vec![
            Instruction::new(Direction::Right, 4),
            Instruction::new(Direction::Up, 4),
            Instruction::new(Direction::Left, 3),
            Instruction::new(Direction::Down, 1),
            Instruction::new(Direction::Right, 4),
            Instruction::new(Direction::Down, 1),
            Instruction::new(Direction::Left, 5),
            Instruction::new(Direction::Right, 2),
        ];

        let actual = generate(EXAMPLE);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let expected = 13;
        let actual = solve_part1(&generate(EXAMPLE));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test3() {
        let expected = (2, 1);
        let actual = reconcile(&(2, 1), &(3, 1), &(1, 1));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test4() {
        let expected = (1, 2);
        let actual = reconcile(&(1, 2), &(1, 3), &(1, 1));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test5() {
        let expected = (2, 2);
        let actual = reconcile(&(2, 2), &(2, 1), &(2, 3));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test6() {
        let expected = (2, 2);
        let actual = reconcile(&(2, 2), &(3, 2), &(1, 3));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test7() {
        let expected = 1;
        let actual = solve_part2(&generate(EXAMPLE));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test8() {
        let expected = 36;
        let actual = solve_part2(&generate(EXAMPLE2));

        assert_eq!(expected, actual)
    }
}
