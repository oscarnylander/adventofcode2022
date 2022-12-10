use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Debug, Clone)]
pub enum Instruction {
    Addx(i32),
    Noop,
}

struct CPU {
    tick: u32,
    x: i32,
    program: Vec<Instruction>,
    cursor: usize,
    busy: bool,
}

impl CPU {
    fn new(program: &[Instruction]) -> Self {
        Self {
            tick: 1,
            x: 1,
            program: program.to_vec(),
            cursor: 0,
            busy: false,
        }
    }

    fn run_cycle(&mut self) {
        match self.program[self.cursor] {
            Instruction::Addx(value) => {
                if self.busy {
                    self.busy = false;
                    self.cursor += 1;
                    self.x += value;
                } else {
                    self.busy = true;
                }
            }
            Instruction::Noop => {
                self.cursor += 1;
            }
        }

        self.tick += 1
    }

    fn signal_strength(&self) -> i32 {
        self.x * self.tick as i32
    }
}

#[aoc_generator(day10)]
pub fn generate(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(' ');

            match split.next().unwrap() {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx(split.next().unwrap().parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Instruction]) -> i32 {
    let mut cpu = CPU::new(input);

    let mut ret = 0;

    for _ in 0..19 {
        cpu.run_cycle();
    }

    ret += cpu.signal_strength();
    cpu.run_cycle();

    for _ in 0..5 {
        for _ in 0..39 {
            cpu.run_cycle();
        }
        ret += cpu.signal_strength();
        cpu.run_cycle();
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "noop
addx 3
addx -5";

    static EXAMPLE2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test1() {
        let expected = vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ];
        let actual = generate(EXAMPLE1);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let expected = 13140;
        let actual = solve_part1(&generate(EXAMPLE2));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test3() {
        let mut cpu = CPU::new(&generate(EXAMPLE1));

        assert_eq!(cpu.tick, 1);
        assert_eq!(cpu.x, 1);

        cpu.run_cycle();
        assert_eq!(cpu.tick, 2);

        cpu.run_cycle();
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.tick, 3);

        cpu.run_cycle();
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.tick, 4);

        cpu.run_cycle();
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.tick, 5);

        cpu.run_cycle();
        assert_eq!(cpu.x, -1);
        assert_eq!(cpu.tick, 6);
    }
}
