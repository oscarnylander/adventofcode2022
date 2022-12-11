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
    currently_executing: Option<ExecutingInstruction>,
}

struct ExecutingInstruction {
    instruction: Instruction,
    cycles: u32,
}

impl ExecutingInstruction {
    fn new(instruction: &Instruction) -> Self {
        Self {
            instruction: instruction.clone(),
            cycles: 0,
        }
    }
}

impl CPU {
    fn new(program: &[Instruction]) -> Self {
        Self {
            tick: 1,
            x: 1,
            program: program.to_vec(),
            cursor: 0,
            currently_executing: Option::None,
        }
    }

    fn begin_cycle(&mut self) {
        if self.currently_executing.is_none() {
            self.currently_executing = Some(ExecutingInstruction::new(&self.program[self.cursor]));
            self.cursor += 1;
        }
    }

    fn end_cycle(&mut self) {
        let mut currently_executing = self.currently_executing.as_mut().unwrap();
        currently_executing.cycles += 1;
        match currently_executing.instruction {
            Instruction::Addx(value) => {
                if currently_executing.cycles == 2 {
                    self.x += value;
                    self.currently_executing = None;
                }
            }
            Instruction::Noop => {
                self.currently_executing = Option::None;
            }
        }
        self.tick += 1;
    }

    fn signal_strength(&self) -> i32 {
        self.x * self.tick as i32
    }

    fn pixel(&self) -> char {
        if self.x.abs_diff(((self.tick - 1) % 40) as i32) < 2 {
            '#'
        } else {
            '.'
        }
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
        cpu.begin_cycle();
        cpu.end_cycle();
    }

    cpu.begin_cycle();
    ret += cpu.signal_strength();
    cpu.end_cycle();

    for _ in 0..5 {
        for _ in 0..39 {
            cpu.begin_cycle();
            cpu.end_cycle();
        }
        cpu.begin_cycle();
        ret += cpu.signal_strength();
        cpu.end_cycle();
    }
    ret
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Instruction]) -> String {
    let mut cpu = CPU::new(input);

    let mut out = String::new();

    for _ in 0..6 {
        for _ in 0..40 {
            cpu.begin_cycle();
            out.push(cpu.pixel());
            cpu.end_cycle();
        }
        out.push('\n');
    }
    out
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

        cpu.begin_cycle();
        assert_eq!(cpu.tick, 1);
        assert_eq!(cpu.x, 1);
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.tick, 2);
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.tick, 3);
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.tick, 4);
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.tick, 5);
        cpu.end_cycle();

        assert_eq!(cpu.x, -1);
        assert_eq!(cpu.tick, 6);
    }

    #[test]
    fn test4() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        .to_string();

        let actual = solve_part2(&generate(EXAMPLE2));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test5() {
        let mut cpu = CPU::new(&generate(EXAMPLE2));

        cpu.begin_cycle();
        assert_eq!(cpu.pixel(), '#');
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.pixel(), '#');
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.pixel(), '.');
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.pixel(), '.');
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.pixel(), '#');
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.pixel(), '#');
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.pixel(), '.');
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.pixel(), '.');
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.pixel(), '#');
        cpu.end_cycle();

        cpu.begin_cycle();
        assert_eq!(cpu.pixel(), '#');
        cpu.end_cycle();
    }
}
