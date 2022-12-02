use std::collections::BinaryHeap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default)]
pub struct Elf {
    meals: Vec<u32>,
}

#[aoc_generator(day1)]
pub fn generate(input: &str) -> Vec<Elf> {
    let mut out = Vec::<Elf>::default();
    let mut current = Elf::default();
    for line in input.lines() {
        if line.is_empty() {
            out.push(current);
            current = Elf::default();
        } else {
            current.meals.push(line.parse().unwrap());
        }
    }
    out
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Elf]) -> u32 {
    let mut best = 0;
    for elf in input {
        let summed = elf.meals.iter().sum();
        if summed > best {
            best = summed;
        }
    }
    best
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Elf]) -> u32 {
    let mut heap = BinaryHeap::<u32>::new();

    for elf in input {
        let summed = elf.meals.iter().sum();
        heap.push(summed);
    }

    let mut sum = 0;
    for _ in 0..3 {
        sum += heap.pop().unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2, Elf};

    impl Elf {
        fn new(meals: Vec<u32>) -> Self {
            Self { meals }
        }
        fn from_meals(meals: Vec<u32>) -> Self {
            Self::new(meals)
        }
    }

    #[test]
    fn test1() {
        let expected = 100;
        let actual = solve_part1(&vec![
            Elf::from_meals(vec![50]),
            Elf::from_meals(vec![100]),
            Elf::from_meals(vec![25]),
        ]);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let expected = 175;
        let actual = solve_part2(&vec![
            Elf::from_meals(vec![50]),
            Elf::from_meals(vec![100]),
            Elf::from_meals(vec![25]),
        ]);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test3() {
        let expected = 175;
        let actual = solve_part2(&vec![
            Elf::from_meals(vec![50]),
            Elf::from_meals(vec![100]),
            Elf::from_meals(vec![25]),
            Elf::from_meals(vec![5]),
        ]);
        assert_eq!(expected, actual)
    }
}
