use aoc_runner_derive::{aoc, aoc_generator};

pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

pub enum Outcome {
    Win,
    Loss,
    Draw,
}

impl RPS {
    fn as_outcome(&self) -> Outcome {
        match self {
            RPS::Rock => Outcome::Loss,
            RPS::Paper => Outcome::Draw,
            RPS::Scissors => Outcome::Win,
        }
    }
}

pub struct RPSRound {
    yours: RPS,
    theirs: RPS,
}

impl RPSRound {
    fn new(theirs: char, yours: char) -> Self {
        Self {
            yours: match yours {
                'X' => RPS::Rock,
                'Y' => RPS::Paper,
                'Z' => RPS::Scissors,
                _ => panic!(),
            },
            theirs: match theirs {
                'A' => RPS::Rock,
                'B' => RPS::Paper,
                'C' => RPS::Scissors,
                _ => panic!(),
            },
        }
    }

    fn value(&self) -> u32 {
        let mut out = 0;
        out += match self.yours {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };
        out += match (&self.yours, &self.theirs) {
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => 0,
            (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => 3,
            (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => 6,
        };
        out
    }

    fn value_outcome(&self) -> u32 {
        let mut out = 0;
        let outcome = self.yours.as_outcome();

        out += match (&self.theirs, &outcome) {
            (RPS::Rock, Outcome::Draw)
            | (RPS::Paper, Outcome::Loss)
            | (RPS::Scissors, Outcome::Win) => 1,
            (RPS::Rock, Outcome::Win)
            | (RPS::Paper, Outcome::Draw)
            | (RPS::Scissors, Outcome::Loss) => 2,
            (RPS::Rock, Outcome::Loss)
            | (RPS::Paper, Outcome::Win)
            | (RPS::Scissors, Outcome::Draw) => 3,
        };

        out += match outcome {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        };
        out
    }
}

#[aoc_generator(day2)]
pub fn generate(input: &str) -> Vec<RPSRound> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let theirs = chars.next().unwrap();
            chars.next().unwrap();
            let yours = chars.next().unwrap();
            RPSRound::new(theirs, yours)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[RPSRound]) -> u32 {
    input.iter().map(RPSRound::value).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[RPSRound]) -> u32 {
    input.iter().map(RPSRound::value_outcome).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let expected = 4 + 5 + 6;
        let actual = solve_part1(&[
            RPSRound::new('A', 'X'),
            RPSRound::new('B', 'Y'),
            RPSRound::new('C', 'Z'),
        ]);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let expected = 8;
        let actual = RPSRound::new('A', 'Y').value();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test3() {
        let expected = 1;
        let actual = RPSRound::new('B', 'X').value();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test4() {
        let expected = 6;
        let actual = RPSRound::new('C', 'Z').value();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test5() {
        let expected = 8 + 1 + 6;
        let actual = solve_part1(&[
            RPSRound::new('A', 'Y'),
            RPSRound::new('B', 'X'),
            RPSRound::new('C', 'Z'),
        ]);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test6() {
        let expected = 4 + 1 + 7;
        let actual = solve_part2(&[
            RPSRound::new('A', 'Y'),
            RPSRound::new('B', 'X'),
            RPSRound::new('C', 'Z'),
        ]);
        assert_eq!(expected, actual)
    }
}
