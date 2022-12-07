use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn solve_generic(input: &str, window: usize) -> u32 {
    let mut out = window as u32;
    let mut counts = HashMap::<char, u32>::default();

    let mut all_unique = true;
    for c in input.chars().take(window) {
        let current = counts.get(&c);
        if current.is_some() {
            all_unique = false;
        }
        counts.insert(c, *current.unwrap_or(&0) + 1);
    }

    if all_unique {
        return out;
    }

    let mut l = input.chars();
    let mut r = input.chars().skip(window).peekable();
    while r.peek().is_some() {
        out += 1;
        let left = l.next().unwrap();
        let right = r.next().unwrap();

        counts.insert(left, counts.get(&left).unwrap() - 1);
        counts.insert(right, counts.get(&right).unwrap_or(&0) + 1);

        let mut has_more_than_one = false;
        for v in counts.values() {
            if v > &1 {
                has_more_than_one = true;
                break;
            }
        }
        if !has_more_than_one {
            return out;
        }
    }
    out
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> u32 {
    solve_generic(input, 4)
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> u32 {
    solve_generic(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let expected = 7;
        let actual = solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let expected = 5;
        let actual = solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test3() {
        let expected = 6;
        let actual = solve_part1("nppdvjthqldpwncqszvftbrmjlhg");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test4() {
        let expected = 10;
        let actual = solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test5() {
        let expected = 11;
        let actual = solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test6() {
        let expected = 19;
        let actual = solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test7() {
        let expected = 23;
        let actual = solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test8() {
        let expected = 23;
        let actual = solve_part2("nppdvjthqldpwncqszvftbrmjlhg");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test9() {
        let expected = 29;
        let actual = solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test10() {
        let expected = 26;
        let actual = solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

        assert_eq!(expected, actual)
    }
}
