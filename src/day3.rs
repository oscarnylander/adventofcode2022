use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Debug)]
pub struct Backpack {
    compartment_one: String,
    compartment_two: String,
}

impl Backpack {
    fn new(compartment_one: &str, compartment_two: &str) -> Self {
        Self {
            compartment_one: compartment_one.to_string(),
            compartment_two: compartment_two.to_string(),
        }
    }
}

#[aoc_generator(day3)]
pub fn generate(input: &str) -> Vec<Backpack> {
    input
        .lines()
        .map(|l| {
            let pivot = l.len() / 2;
            let first = &l[..pivot];
            let second = &l[pivot..];
            Backpack::new(first, second)
        })
        .collect()
}

fn to_priority(c: char) -> u32 {
    let ascii: u32 = c.into();
    let ascii = ascii - 64;
    if ascii > 32 {
        ascii - 32
    } else {
        ascii + 26
    }
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Backpack]) -> u32 {
    input
        .iter()
        .map(|b| {
            let mut seen_chars = HashSet::new();
            for c in b.compartment_one.chars() {
                seen_chars.insert(c);
            }
            for c in b.compartment_two.chars() {
                if seen_chars.contains(&c) {
                    return to_priority(c);
                }
            }
            unreachable!()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test1() {
        let expected = vec![
            Backpack::new("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            Backpack::new("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            Backpack::new("PmmdzqPrV", "vPwwTWBwg"),
            Backpack::new("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
            Backpack::new("ttgJtRGJ", "QctTZtZT"),
            Backpack::new("CrZsJsPPZsGz", "wwsLwLmpwMDw"),
        ];
        let actual = generate(EXAMPLE);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let input = generate(EXAMPLE);

        let expected = 157;
        let actual = solve_part1(&input);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test3() {
        let expected = 1;
        let actual = to_priority('a');
        assert_eq!(expected, actual)
    }

    #[test]
    fn test4() {
        let expected = 27;
        let actual = to_priority('A');
        assert_eq!(expected, actual)
    }

    #[test]
    fn test5() {
        let expected = 52;
        let actual = to_priority('Z');
        assert_eq!(expected, actual)
    }
    #[test]
    fn test6() {
        let expected = 26;
        let actual = to_priority('z');
        assert_eq!(expected, actual)
    }
}
