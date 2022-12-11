use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(PartialEq, Debug, Clone)]
pub enum Operation {
    Add(u64),
    Multiply(u64),
    MultiplySelf,
}

impl Operation {
    fn execute(&self, val: u64) -> u64 {
        match self {
            Operation::Add(term) => term + val,
            Operation::Multiply(factor) => val * factor,
            Operation::MultiplySelf => val * val,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    on_true: usize,
    on_false: usize,
}

impl Monkey {
    fn new(
        items: &[u64],
        operation: Operation,
        divisor: u64,
        on_true: usize,
        on_false: usize,
    ) -> Self {
        Self {
            items: VecDeque::from_iter(items.iter().map(|i| *i)),
            operation,
            divisor,
            on_true,
            on_false,
        }
    }
}

#[aoc_generator(day11)]
pub fn generate(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines().skip(1);
            let items = lines
                .next()
                .unwrap()
                .trim_start()
                .split(' ')
                .skip(2)
                .map(|i| i.replace(',', "").parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let op = lines
                .next()
                .unwrap()
                .split('=')
                .skip(1)
                .next()
                .unwrap()
                .trim_start();
            let op = if op == "old * old" {
                Operation::MultiplySelf
            } else {
                let term = op
                    .trim_start()
                    .split(' ')
                    .skip(2)
                    .next()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                if op.contains('*') {
                    Operation::Multiply(term)
                } else {
                    Operation::Add(term)
                }
            };
            let divisor = lines
                .next()
                .unwrap()
                .trim_start()
                .split(' ')
                .skip(3)
                .next()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let on_true = lines
                .next()
                .unwrap()
                .trim_start()
                .split(' ')
                .skip(5)
                .next()
                .unwrap()
                .parse()
                .unwrap();
            let on_false = lines
                .next()
                .unwrap()
                .trim_start()
                .split(' ')
                .skip(5)
                .next()
                .unwrap()
                .parse()
                .unwrap();
            Monkey::new(&items, op, divisor, on_true, on_false)
        })
        .collect()
}

fn run_round(monkeys: &mut [Monkey]) -> Vec<u64> {
    let mut inspects = vec![0; monkeys.len()];
    for idx in 0..monkeys.len() {
        let monkey = monkeys.get_mut(idx).unwrap();
        let mut to_send = Vec::<(usize, u64)>::default();
        loop {
            let item = monkey.items.pop_front();
            if item.is_none() {
                break;
            }
            inspects[idx] = inspects[idx] + 1;
            let item = monkey.operation.execute(item.unwrap());
            let item = item / 3;
            let test_succeeded = item % monkey.divisor == 0;
            let receiver = if test_succeeded {
                monkey.on_true
            } else {
                monkey.on_false
            };
            to_send.push((receiver, item));
        }
        for (receiver, item) in to_send {
            monkeys.get_mut(receiver).unwrap().items.push_back(item);
        }
    }
    inspects
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Monkey]) -> u64 {
    let mut monkeys = input.to_vec();
    let mut inspects = vec![0; monkeys.len()];

    for _ in 0..20 {
        let round_inspects = run_round(&mut monkeys);
        for idx in 0..inspects.len() {
            inspects[idx] = inspects[idx] + round_inspects[idx];
        }
    }

    inspects.iter().sorted().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test1() {
        let expected = vec![
            Monkey::new(&[79, 98], Operation::Multiply(19), 23, 2, 3),
            Monkey::new(&[54, 65, 75, 74], Operation::Add(6), 19, 2, 0),
            Monkey::new(&[79, 60, 97], Operation::MultiplySelf, 13, 1, 3),
            Monkey::new(&[74], Operation::Add(3), 17, 0, 1),
        ];
        let actual = generate(EXAMPLE);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let mut monkeys = generate(EXAMPLE);
        let inspects = run_round(&mut monkeys);

        assert_eq!(VecDeque::from(vec![20, 23, 27, 26]), monkeys[0].items);
        assert_eq!(
            VecDeque::from(vec![2080, 25, 167, 207, 401, 1046]),
            monkeys[1].items,
        );
        assert_eq!(VecDeque::default(), monkeys[2].items);
        assert_eq!(VecDeque::default(), monkeys[3].items);
        assert_eq!(vec![2, 4, 3, 5], inspects);
    }

    #[test]
    fn test3() {
        let expected = 10605;
        let actual = solve_part1(&generate(EXAMPLE));

        assert_eq!(expected, actual)
    }
}
