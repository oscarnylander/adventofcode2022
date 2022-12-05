use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(PartialEq, Debug, Clone)]
struct Move {
    times: usize,
    source: usize,
    destination: usize,
}

impl Move {
    fn new(times: usize, source: usize, destination: usize) -> Self {
        Self {
            times,
            source,
            destination,
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Input {
    stacks: HashMap<usize, Vec<char>>,
    moves: Vec<Move>,
}

impl Input {
    fn new(stacks: HashMap<usize, Vec<char>>, moves: Vec<Move>) -> Self {
        Self { stacks, moves }
    }
}

impl Clone for Input {
    fn clone(&self) -> Self {
        Self::new(
            self.stacks.clone(),
            self.moves.iter().map(|x| x.clone()).collect(),
        )
    }
}

#[aoc_generator(day5)]
pub fn generate(input: &str) -> Input {
    let mut out: Input = Default::default();
    let mut lines = input.lines();
    let mut line = lines.next().unwrap();
    while !line.is_empty() {
        for (idx, mut chunk) in line
            .chars()
            .enumerate()
            .filter(|(i, _)| i % 4 != 3 || i == &0)
            .map(|(_, item)| item)
            .chunks(3)
            .into_iter()
            .enumerate()
        {
            let idx = idx + 1;
            if chunk.next().unwrap() == ' ' {
                continue;
            }
            if !out.stacks.contains_key(&idx) {
                out.stacks.insert(idx, Default::default());
            }
            out.stacks
                .get_mut(&idx)
                .unwrap()
                .insert(0, chunk.next().unwrap());
        }
        line = lines.next().unwrap();
    }
    for line in lines {
        let mut iter = line.split(' ').skip(1);
        let times = iter.next().unwrap().parse().unwrap();
        iter.next().unwrap();
        let source = iter.next().unwrap().parse().unwrap();
        iter.next().unwrap();
        let destination = iter.next().unwrap().parse().unwrap();
        out.moves.push(Move::new(times, source, destination));
    }
    out
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> String {
    let mut input = input.clone();
    for m in &input.moves {
        for _ in 0..m.times {
            let from_source = input.stacks.get_mut(&m.source).unwrap().pop().unwrap();
            input
                .stacks
                .get_mut(&m.destination)
                .unwrap()
                .push(from_source);
        }
    }
    let mut out = String::new();
    for (_, stack) in input.stacks.iter().sorted_by_key(|(k, _)| *k) {
        out.push(*stack.last().unwrap())
    }
    out
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> String {
    let mut input = input.clone();
    for m in &input.moves {
        let mut from_source_buf = Vec::<char>::default();
        for _ in 0..m.times {
            from_source_buf.push(input.stacks.get_mut(&m.source).unwrap().pop().unwrap())
        }
        while !from_source_buf.is_empty() {
            input
                .stacks
                .get_mut(&m.destination)
                .unwrap()
                .push(from_source_buf.pop().unwrap());
        }
    }
    let mut out = String::new();
    for (_, stack) in input.stacks.iter().sorted_by_key(|(k, _)| *k) {
        out.push(*stack.last().unwrap())
    }
    out
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;

    use super::*;

    static EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test1() {
        let expected = Input::new(
            hashmap! {
                1 => vec!['Z','N',],
                2 => vec!['M','C','D',],
                3 => vec!['P']
            },
            vec![
                Move::new(1, 2, 1),
                Move::new(3, 1, 3),
                Move::new(2, 2, 1),
                Move::new(1, 1, 2),
            ],
        );

        let actual = generate(EXAMPLE);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let expected = "CMZ".to_string();
        let actual = solve_part1(&generate(EXAMPLE));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test3() {
        let expected = "MCD".to_string();
        let actual = solve_part2(&generate(EXAMPLE));

        assert_eq!(expected, actual)
    }
}
